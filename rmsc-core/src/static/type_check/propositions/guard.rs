use std::collections::HashSet;
use crate::r#static::type_check::propositions::prop::Prop;
use crate::r#static::type_check::propositions::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Guard {
    truthy: HashSet<Symbol>,
    falsy: HashSet<Symbol>,
}

impl Guard {
    pub fn new() -> Self {
        Self {
            truthy: HashSet::new(),
            falsy: HashSet::new(),
        }
    }

    pub fn mk_true(&mut self, v: &str) {
        self.truthy.insert(v.into());
    }

    pub fn mk_false(&mut self, v: &str) {
        self.falsy.insert(v.into());
    }

    pub fn lookup(&self, v: &Symbol) -> Prop {
        if self.truthy.contains(v) {
            return Prop::True;
        }
        if self.falsy.contains(v) {
            return Prop::False;
        }
        Prop::Var(v.clone())
    }
}