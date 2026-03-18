use std::collections::{HashMap, HashSet};
use crate::parsing::Identifier;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Symbol {
    Name(Identifier),
    Random { block: u32, arm: u32, chance: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Prop {
    True,
    False,

    Var(Symbol),
    And(Vec<Prop>),
    Or(Vec<Prop>),
    Not(Symbol),
}

#[derive(Debug, Clone)]
pub struct Guard {
    truthy: HashSet<Symbol>,
    falsy: HashSet<Symbol>,
}

impl Guard {
    fn lookup(&self, v: &Symbol) -> Prop {
        if self.truthy.contains(v) {
            return Prop::True;
        }
        if self.falsy.contains(v) {
            return Prop::False;
        }
        Prop::Var(v.clone())
    }
}

impl Prop {
    pub fn simplify(&self, guard: &Guard) -> Prop {
        match self {
            Prop::True | Prop::False => self.clone(),
            Prop::Var(v) => guard.lookup(v),
            Prop::And(xs) => 'et: {
                let mut et = HashSet::with_capacity(xs.len());
                for x in xs {
                    match x.simplify(guard) {
                        Prop::True => continue,
                        Prop::False => break 'et Prop::False,
                        v => { et.insert(v); } /* A.A = A */
                    }
                }
                et.simplify_and()
            }
            Prop::Or(xs) => 'vel: {
                let mut vel = HashSet::with_capacity(xs.len());
                for x in xs {
                    match x.simplify(guard) {
                        Prop::False => continue,
                        Prop::True => break 'vel Prop::True,
                        v => { vel.insert(v); }, /* A + A = A */
                    }
                }
                Prop::Or(vel.into_iter().collect())
            }
            Prop::Not(x) => {
                match guard.lookup(x) {
                    Prop::False => Prop::True,
                    Prop::True => Prop::False,
                    v => v,
                }
            }
        }
    }
}

trait Simplifiable {
    fn simplify_and(self) -> Prop;
    fn simplify_or(self) -> Prop;
}

impl Simplifiable for HashSet<Prop> {
    fn simplify_and(self) -> Prop {
        let mut symbols = HashSet::with_capacity(self.len());
        let mut blocks = HashSet::with_capacity(self.len());

        for prop in &self {
            match prop {
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

        Prop::And(self.into_iter().collect())
    }

    fn simplify_or(self) -> Prop {
        let mut symbols = HashSet::with_capacity(self.len());
        let mut blocks = HashMap::with_capacity(self.len());

        for prop in &self {
            match prop {
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
                        // todo: A P_11(50) + A P_12(50) = A
                        let entry = blocks.entry(block).or_insert((*arm, *chance));
                        if entry.0 != *arm {
                            entry.1 += *chance;
                        }
                        if entry.1 >= 100 {
                            return Prop::True;
                        }
                    }
                }
                _ => unreachable!("Internal Error: simplify_or called on non CNF")
            }
        }

        Prop::Or(self.into_iter().collect())
    }
}