use std::mem;
use crate::parsing::{Expr, Type};
use crate::r#static::info::src_loc::SrcLoc;
use crate::r#static::type_check::propositions::{Guard, Prop};

#[derive(Debug, Clone)]
pub struct IdInfo {
    pub type_: Type,
    pub src_loc: SrcLoc,
    pub guard: Prop,
}

impl IdInfo {
    pub fn from(type_: &Type, src_loc: SrcLoc, guard: &Guard) -> Self {
        Self { type_: type_.clone(), src_loc, guard: guard.get_prop() }
    }

    pub fn join(&mut self, guard: &Guard) {
        let current = mem::replace(&mut self.guard, Prop::False);
        self.guard = current | guard.get_prop()
    }
}