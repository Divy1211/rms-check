use std::collections::{HashMap, HashSet};
use crate::r#static::type_check::propositions::prop::{Prop, Simplifiable};
use crate::r#static::type_check::propositions::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Guard {
    truthy: HashMap<Symbol, HashSet<u32>>,
    falsy: HashMap<Symbol, HashSet<u32>>,
    block_arm: Option<(u32, u32, u32)>,
}

impl Guard {
    pub fn new() -> Self {
        Self {
            truthy: HashMap::new(),
            falsy: HashMap::new(),
            block_arm: None,
        }
    }

    pub fn in_arm(&mut self, block: u32, arm: u32, chance: u32) {
        self.block_arm = Some((block, arm, chance));
    }

    pub fn truthify(&mut self, block: u32, v: &str) {
        let key = Symbol::from(v);
        let truthy =self.truthy.entry(key.clone()).or_insert_with(|| HashSet::new());
        truthy.insert(block);

        if let Some(falsy) = self.falsy.get_mut(&key) {
        falsy.remove(&block);
            if falsy.is_empty() {
                self.falsy.remove(&key);
            }
        }
    }

    pub fn falsify(&mut self, block: u32, v: &str) {
        let key = Symbol::from(v);
        let falsy =self.falsy.entry(key.clone()).or_insert_with(|| HashSet::new());
        falsy.insert(block);

        if let Some(truthy) = self.truthy.get_mut(&key) {
            truthy.remove(&block);
            if truthy.is_empty() {
                self.truthy.remove(&key);
            }
        }
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
        if self.truthy.contains_key(v) {
            return Prop::True;
        }
        if self.falsy.contains_key(v) {
            return Prop::False;
        }
        Prop::Var(v.clone())
    }

    pub fn get_prop(&self) -> Prop {
        let mut et = Vec::with_capacity(self.truthy.len() + self.falsy.len());
        for (var, _blocks) in &self.truthy {
            et.push(Prop::Var(var.clone()))
        }
        for (var, _blocks) in &self.falsy {
            et.push(Prop::Not(var.clone()))
        }
        if let Some((block, arm, chance)) = self.block_arm {
            et.push(Prop::from_block(block, arm, chance));
        }

        et.simplify_and()
    }
}