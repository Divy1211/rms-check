use std::collections::HashSet;
use crate::parsing::Identifier;
use crate::r#static::type_check::propositions::prop::Prop;
use crate::r#static::type_check::propositions::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Guard {
    truthy: HashSet<Symbol>,
    falsy: HashSet<Symbol>,
    block_arm: Option<(u32, u32, u32)>,
}

impl Guard {
    pub fn new() -> Self {
        Self {
            truthy: HashSet::new(),
            falsy: HashSet::new(),
            block_arm: None,
        }
    }

    pub fn in_arm(&mut self, block: u32, arm: u32, chance: u32) {
        self.block_arm = Some((block, arm, chance));
    }

    pub fn remove(&mut self, v: &str) {
        self.truthy.remove(&(v.into()));
        self.falsy.remove(&(v.into()));
    }

    pub fn truthify(&mut self, v: &str) {
        let v = v.into();
        self.falsy.remove(&v);
        self.truthy.insert(v);
    }

    pub fn falsify(&mut self, v: &str) {
        let v = v.into();
        self.truthy.remove(&v);
        self.falsy.insert(v);
    }

    pub fn lookup(&self, v: &Symbol) -> Prop {
        match (v, self.block_arm) {
            (Symbol::Random { block, arm, .. }, Some((current_block, current_arm, _chance))) if *block == current_block => {
                return if *arm == current_arm {
                    Prop::True
                } else {
                    Prop::False
                }
            }
            _ => {}
        }
        if self.truthy.contains(v) {
            return Prop::True;
        }
        if self.falsy.contains(v) {
            return Prop::False;
        }
        Prop::Var(v.clone())
    }

    pub fn get_prop(&self) -> Prop {
        let mut et = Vec::with_capacity(self.truthy.len() + self.falsy.len());
        for var in &self.truthy {
            et.push(Prop::Var(var.clone()))
        }
        for var in &self.falsy {
            et.push(Prop::Not(var.clone()))
        }
        if let Some((block, arm, chance)) = self.block_arm {
            et.push(Prop::from_block(block, arm, chance));
        }

        if et.is_empty() {
            return Prop::True;
        } else if et.len() == 1 {
            return et.into_iter().next().unwrap();
        }

        Prop::And(et)
    }
}