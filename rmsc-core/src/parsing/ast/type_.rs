use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Label,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Label => write!(f, "label"),
        }
    }
}