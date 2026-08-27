use std::fmt::{Display, Formatter};
use crate::parsing::Identifier;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Symbol {
    Name(Identifier),
    Random { block: u32, arm: u32, chance: u32 },
}

impl Symbol {
    pub fn from_name(name: &str) -> Self {
        Symbol::Name(name.into())
    }
    pub fn from_block(block: u32, arm: u32, chance: u32) -> Self {
        Symbol::Random { block, arm, chance }
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Symbol::from_name(value)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Name(id) => { write!(f, "{}", id) }
            Symbol::Random { block, arm, chance } => { write!(f, "Rnd({}, {}, {})", block, arm, chance) }
        }
    }
}