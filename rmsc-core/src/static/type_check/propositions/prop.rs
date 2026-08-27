use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitOr, Not};
use crate::parsing::Identifier;
use crate::r#static::type_check::propositions::guard::Guard;
use crate::r#static::type_check::propositions::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Prop {
    True,
    False,

    Var(Symbol),
    And(Vec<Prop>),
    Or(Vec<Prop>),
    Not(Symbol),
}

impl Not for Prop {
    type Output = Prop;

    fn not(self) -> Prop {
        match self {
            Prop::True => Prop::False,
            Prop::False => Prop::True,
            Prop::Var(s) => Prop::Not(s.clone()),
            Prop::Not(s) => Prop::Var(s.clone()),
            Prop::And(et) => Prop::Or(et.into_iter().map(Not::not).collect()),
            Prop::Or(vel) => Prop::And(vel.into_iter().map(Not::not).collect()),
        }
    }
}

impl BitAnd for Prop {
    type Output = Prop;

    fn bitand(self, var: Prop) -> Prop {
        match (self, var) {
            (_, Prop::False) | (Prop::False, _) => Prop::False,
            (p, Prop::True) | (Prop::True, p) => p,
            (Prop::And(mut et), p @ (Prop::Var(_) | Prop::Not(_)))
            | (p @ (Prop::Var(_) | Prop::Not(_)), Prop::And(mut et)) => {
                et.push(p);
                et.simplify_and()
            },
            (Prop::And(mut et1), Prop::And(mut et2)) => {
                et1.append(&mut et2);
                et1.simplify_and()
            },
            (p1 @ (Prop::Var(_) | Prop::Not(_)), p2 @ (Prop::Var(_) | Prop::Not(_))) => {
                vec![p1, p2].simplify_and()
            },
            (Prop::Or(mut vel), p @ (Prop::Var(_) | Prop::Not(_) | Prop::And(_)))
            | (p @ (Prop::Var(_) | Prop::Not(_) | Prop::And(_)), Prop::Or(mut vel)) => {
                for prop in vel.iter_mut() {
                    *prop = std::mem::replace(prop, Prop::False) & p.clone();
                }
                vel.simplify_or()
            }
            (p1, p2) => unreachable!("Internal Error: Attempting to and {:?} {:?}. Too expensive", p1, p2),
        }
    }
}

impl BitOr for Prop {
    type Output = Prop;

    fn bitor(self, var: Prop) -> Prop {
        match (self, var) {
            (_, Prop::True) | (Prop::True, _) => Prop::True,
            (p, Prop::False) | (Prop::False, p) => p,
            (Prop::Or(mut vel), p @ (Prop::Var(_) | Prop::Not(_)))
            | (p @ (Prop::Var(_) | Prop::Not(_)), Prop::Or(mut vel)) => {
                vel.push(p);
                vel.simplify_or()
            }
            (Prop::Or(mut vel1), Prop::Or(mut vel2)) => {
                vel1.append(&mut vel2);
                vel1.simplify_or()
            },
            (Prop::Or(mut vel), p @ Prop::And(_)) => {
                vel.push(p);
                vel.simplify_or()
            },
            (p1, p2) => vec![p1, p2].simplify_or(),
        }
    }
}

impl Prop {
    pub fn is_singleton(&self) -> bool {
        matches!(self, Prop::True | Prop::False | Prop::Var(_) | Prop::Not(_))
    }

    pub fn is_not(&self) -> bool {
        matches!(self, Prop::Not(_))
    }

    pub fn from_name(name: &str) -> Prop {
        Prop::Var(Symbol::from_name(name))
    }

    pub fn from_id(id: &Identifier) -> Prop {
        Prop::Var(Symbol::from_name(&id.0))
    }

    pub fn from_block(block: u32, arm: u32, chance: u32) -> Self {
        Prop::Var(Symbol::from_block(block, arm, chance))
    }

    pub fn simplify(&self, guard: &Guard) -> Prop {
        match self {
            Prop::True | Prop::False => self.clone(),
            Prop::Var(v) => {
                guard.lookup(v)
            },
            Prop::And(xs) => 'et: {
                let mut et = Vec::with_capacity(xs.len());
                for x in xs {
                    match x.simplify(guard) {
                        Prop::True => continue,
                        Prop::False => break 'et Prop::False,
                        v => { et.push(v); }
                    }
                }
                et.simplify_and()
            }
            Prop::Or(xs) => 'vel: {
                let mut vel = Vec::with_capacity(xs.len());
                for x in xs {
                    match x.simplify(guard) {
                        Prop::False => continue,
                        Prop::True => break 'vel Prop::True,
                        v => { vel.push(v); }
                    }
                }
                vel.simplify_or()
            }
            Prop::Not(v) => {
                guard.lookup(v).not()
            }
        }
    }
}

pub trait Simplifiable {
    fn simplify_and(self) -> Prop;
    fn simplify_or(self) -> Prop;
}

impl Simplifiable for Vec<Prop> {
    fn simplify_and(mut self) -> Prop {
        /* A.A = A */
        self = self.into_iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();

        if self.is_empty() {
            return Prop::True;
        } else if self.len() == 1 {
            return self.into_iter().next().unwrap();
        }

        let mut symbols = HashSet::with_capacity(self.len());
        let mut blocks = HashSet::with_capacity(self.len());

        let mut removal = HashSet::with_capacity(self.len());
        let mut do_removal = false;

        for (i, prop) in self.iter().enumerate() {
            match prop {
                Prop::True => {},
                Prop::False => return Prop::False,
                Prop::Not(s) | Prop::Var(s) => match s {
                    Symbol::Name(id) => {
                        /* A.A' = 0 */
                        if symbols.contains(&(id, !prop.is_not())) {
                            return Prop::False;
                        }
                        symbols.insert((id, prop.is_not()));
                    }
                    Symbol::Random { block, .. } => {
                        /* P_in(x).P_im(x) = 0 */
                        if blocks.contains(&(block, false)) && !prop.is_not() {
                            return Prop::False;
                        }
                        /* P_in(x).P'_im(x) = P_in(x) */
                        /* if we encounter any regular P(x) blocks, remove any complimentary ones */
                        if !prop.is_not() {
                            do_removal = true;
                        } else {
                            removal.insert(i);
                        }
                        blocks.insert((block, prop.is_not()));
                    }
                }
                _ => unreachable!("Internal Error: Propositions are always in DNF {:?}", prop)
            }
        }

        if do_removal && !removal.is_empty() {
            self = self
                .into_iter()
                .enumerate()
                .filter_map(|(i, x)| if removal.contains(&i) { None } else { Some(x) })
                .collect()
        }

        if self.len() == 1 {
            return self.into_iter().next().unwrap();
        }

        Prop::And(self)
    }

    fn simplify_or(mut self) -> Prop {
        /* A + A = A */
        self = self.into_iter().collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();

        if self.is_empty() {
            return Prop::True
        } else if self.len() == 1 {
            return self.into_iter().next().unwrap();
        }

        let mut symbols = HashSet::with_capacity(self.len());
        let mut blocks = HashMap::with_capacity(self.len());

        let mut removal = HashSet::with_capacity(self.len());
        let mut has_complimentary_random = false;

        for (i, prop) in self.iter().enumerate() {
            match prop {
                Prop::True => return Prop::True,
                Prop::False => {},
                Prop::Not(s) | Prop::Var(s) => match s {
                    Symbol::Name(id) => {
                        /* A + A' = 1 */
                        if symbols.contains(&(id, !prop.is_not())) {
                            return Prop::True;
                        }
                        symbols.insert((id, prop.is_not()));
                    }
                    Symbol::Random { block, arm, chance } => {
                        /* P_in(x1) + P_im(x2) = P_inm(x1 + x2) */
                        let (entry, new_block) = match blocks.entry(*block) {
                            Entry::Vacant(entry) => {
                                (entry.insert((*arm, 0)), true)
                            }
                            Entry::Occupied(entry) => {
                                (entry.into_mut(), false)
                            }
                        };
                        if !prop.is_not() {
                            entry.1 += *chance;
                            if !new_block {
                                removal.insert(i);
                            }
                        } else {
                            /* P'_in(x1) + P'_im(x2) = 1 */
                            if has_complimentary_random {
                                return Prop::True;
                            }
                            has_complimentary_random = true;
                        }
                        if entry.1 >= 100 {
                            return Prop::True;
                        }
                    }
                }
                _ => {}
            }
        }

        if !removal.is_empty() {
            self = self
                .into_iter()
                .enumerate()
                .filter_map(|(i, mut x)| if removal.contains(&i) {
                    None
                } else {
                    if let Prop::Not(s) | Prop::Var(s) = &mut x
                    && let Symbol::Random { block, arm, chance } = s {
                        *chance = blocks.get(block).unwrap_or(&(*arm, *chance)).1;
                    }
                    Some(x)
                })
                .collect()
        }

        if self.len() == 1 {
            return self.into_iter().next().unwrap();
        }

        Prop::Or(self)
    }
}

impl Display for Prop {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Prop::True => { write!(f, "true") }
            Prop::False => { write!(f, "false") }
            Prop::Var(v) => { write!(f, "{}", v) }
            Prop::And(et) => {
                write!(f, "(")?;
                for (i, p) in et.iter().enumerate() {
                    if i < et.len() - 1 {
                        write!(f, "{} & ", p)?;
                    } else {
                        write!(f, "{}", p)?;
                    }
                }
                write!(f, ")")
            }
            Prop::Or(vel) => {
                write!(f, "(")?;
                for (i, p) in vel.iter().enumerate() {
                    if i < vel.len() - 1 {
                        write!(f, "{} | ", p)?;
                    } else {
                        write!(f, "{}", p)?;
                    }
                }
                write!(f, ")")
            }
            Prop::Not(neg) => { write!(f, "{}'", neg) }
        }
    }
}