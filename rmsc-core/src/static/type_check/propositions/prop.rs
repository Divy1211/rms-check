use std::collections::{HashMap, HashSet};
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
            p => unreachable!("Internal Error: Not over compound proposition {:?}", p),
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
            (p1, p2) => unreachable!("Internal Error: Attempting to and {:?} {:?}. DNF must be maintained", p1, p2),
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

trait Simplifiable {
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

        for prop in &self {
            match prop {
                Prop::True => {},
                Prop::False => return Prop::False,
                Prop::Not(s) | Prop::Var(s) => match s {
                    Symbol::Name(id) => {
                        /* A.A' = 0 */
                        if symbols.contains(id) {
                            return Prop::False;
                        }
                        symbols.insert(id);
                    }
                    Symbol::Random { block, .. } => {
                        /* P_in(x).P_im(x) = 0 */
                        if blocks.contains(block) {
                            return Prop::False;
                        }
                        blocks.insert(block);
                    }
                }
                _ => unreachable!("Internal Error: Propositions are always in DNF")
            }
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

        for prop in &self {
            match prop {
                Prop::True => return Prop::True,
                Prop::False => {},
                Prop::Not(s) | Prop::Var(s) => match s {
                    Symbol::Name(id) => {
                        /* A + A' = 1 */
                        if symbols.contains(id) {
                            return Prop::True;
                        }
                        symbols.insert(id);
                    }
                    Symbol::Random { block, arm, chance } => {
                        /* P_in(x1) + P_im(x2) = P_inm(x1 + x2) */
                        let entry = blocks.entry(block).or_insert((*arm, *chance));
                        if entry.0 != *arm {
                            entry.1 += *chance;
                        }
                        if entry.1 >= 100 {
                            return Prop::True;
                        }
                    }
                }
                _ => {}
            }
        }

        Prop::Or(self)
    }
}