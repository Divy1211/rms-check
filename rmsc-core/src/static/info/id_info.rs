use crate::parsing::{Expr, Type};
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Debug, Clone)]
pub struct IdInfo {
    pub type_: Type,
    pub src_loc: SrcLoc,
    pub init: Option<Expr>,
}

impl IdInfo {
    pub fn from(type_: &Type, src_loc: SrcLoc) -> Self {
        Self { type_: type_.clone(), init: None, src_loc }
    }

    pub fn new(type_: Type, src_loc: SrcLoc) -> Self {
        Self { type_, init: None, src_loc }
    }
    
    pub fn dummy(type_: Type) -> Self {
        Self { type_, init: None, src_loc: Default::default() }
    }
}