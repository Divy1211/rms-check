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