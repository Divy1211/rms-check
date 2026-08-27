use std::collections::{HashMap, HashSet};
use crate::Identifier;
use crate::r#static::type_check::propositions::prop::{Prop, Simplifiable};
use crate::r#static::type_check::propositions::symbol::Symbol;

#[derive(Debug, Clone)]
pub struct Guard {
    /* HashSet<u32> as value because a nested block should not remove a truthy/falsy symbol from the outer block */
    truthy: HashMap<Symbol, HashSet<u32>>,
    falsy: HashMap<Symbol, HashSet<u32>>,
    block_arm: Option<(u32, u32, u32)>,
    pub falsy_block_arms: HashMap<u32, (u32, HashSet<u32>)>,
    pub truthy_block_arm: HashMap<u32, u32>,
}

impl Default for Guard {
    fn default() -> Self {
        Self::new()
    }
}

impl Guard {
    pub fn new() -> Self {
        Self {
            truthy: HashMap::new(),
            falsy: HashMap::new(),
            block_arm: None,
            falsy_block_arms: HashMap::new(),
            truthy_block_arm: HashMap::new(),
        }
    }

    pub fn is_arm_chosen(&self, block: u32, arm: u32) -> bool {
        self.truthy_block_arm.get(&block).is_some_and(|chosen_arm| *chosen_arm == arm)
    }

    pub fn chance_increase(&self, block: u32) -> u32 {
        self.falsy_block_arms
            .get(&block)
            .map(|(increase, _arms)| *increase)
            .unwrap_or(0)
    }

    pub fn arm_is_falsy(&self, block: u32, arm: u32) -> bool {
        self.falsy_block_arms
            .get(&block)
            .is_some_and(|(_increase, arms)| arms.contains(&arm))
    }

    pub fn in_arm(&mut self, block: u32, arm: u32, chance: u32) {
        self.block_arm = Some((block, arm, chance));
    }

    pub fn truthify(&mut self, block: u32, v: &str) {
        let key = Symbol::from(v);
        let truthy = self.truthy.entry(key.clone()).or_default();
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
        let falsy = self.falsy.entry(key.clone()).or_default();
        falsy.insert(block);

        if let Some(truthy) = self.truthy.get_mut(&key) {
            truthy.remove(&block);
            if truthy.is_empty() {
                self.truthy.remove(&key);
            }
        }
    }

    pub fn is_true(&self, v: &Identifier) -> bool {
        self.truthy.contains_key(&Symbol::Name(v.clone()))
    }

    pub fn is_false(&self, v: &Identifier) -> bool {
        self.falsy.contains_key(&Symbol::Name(v.clone()))
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
            (Symbol::Random { block, arm, chance }, None) => {
                if let Some(chosen_arm) = self.truthy_block_arm.get(block) {
                    return if chosen_arm == arm {
                        Prop::True
                    } else {
                        Prop::False
                    }
                }
                if self.arm_is_falsy(*block, *arm) {
                    return Prop::False;
                }
                if chance + self.chance_increase(*block) >= 100 {
                    return Prop::True;
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
        for var in self.truthy.keys() {
            et.push(Prop::Var(var.clone()))
        }
        for var in self.falsy.keys() {
            et.push(Prop::Not(var.clone()))
        }
        if let Some((block, arm, chance)) = self.block_arm {
            et.push(Prop::from_block(block, arm, chance));
        }

        et.simplify_and(Some(self))
    }
}