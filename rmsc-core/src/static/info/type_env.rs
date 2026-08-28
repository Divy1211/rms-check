use std::collections::{HashMap, HashSet};
use std::ops::Not;
use std::path::PathBuf;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::parsing::{Identifier, Type};
use crate::r#static::info::id_info::IdInfo;
use crate::r#static::info::rms_error::RmsError;
use crate::r#static::type_check::propositions::{Guard, Prop, Symbol, Simplifiable};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Liveness {
    Live,
    Dead,
    Maybe,
}

impl Liveness {
    pub fn invert(self) -> Liveness {
        match self {
            Liveness::Live => Liveness::Dead,
            Liveness::Dead => Liveness::Live,
            Liveness::Maybe => Liveness::Maybe,
        }
    }

    pub fn is_maybe_live(&self) -> bool {
        matches!(self, Liveness::Live | Liveness::Maybe)
    }
}

#[derive(Debug, Clone)]
pub struct TypeEnv {
    identifiers: HashMap<Identifier, IdInfo>,
    object_groups: HashMap<Identifier, IdInfo>,
    pub guard: Arc<RwLock<Guard>>,

    pub substitution_cache_id: Arc<RwLock<HashMap<Identifier, Prop>>>,
    pub substitution_cache_grp: Arc<RwLock<HashMap<Identifier, Prop>>>,

    pub errs: HashMap<PathBuf, Vec<RmsError>>,
    
    pub current_ignores: Arc<RwLock<Option<HashSet<u32>>>>,
    
    pub include_dirs: Arc<Vec<PathBuf>>,
    pub dependencies: Option<HashMap<PathBuf, HashSet<PathBuf>>>,

    pub last_block: u32,

    pub skip_includes: bool,
    pub check_dead_paths: bool,
}

pub struct TempIgnore {
    ignores: Arc<RwLock<Option<HashSet<u32>>>>,
}

impl Drop for TempIgnore {
    fn drop(&mut self) {
        self.ignores.write().expect("Not concurrent").take();
    }
}

pub struct NestedGuard {
    prev_guard: Option<Guard>,
    guard: Arc<RwLock<Guard>>,
    cache1: Arc<RwLock<HashMap<Identifier, Prop>>>,
    cache2: Arc<RwLock<HashMap<Identifier, Prop>>>,
}

impl Drop for NestedGuard {
    fn drop(&mut self) {
        *self.guard.write().expect("Not concurrent") = self.prev_guard.take()
            .expect("Internal Error: No previous guard");
        self.cache1.write().expect("Not concurrent").clear();
        self.cache2.write().expect("Not concurrent").clear();
    }
}

impl TypeEnv {
    pub fn nested_guard(&mut self) -> NestedGuard {
        self.last_block += 1;

        let mut guard = self.guard_mut();
        let prev_guard = Some(guard.clone());

        guard.current_block = self.last_block;
        NestedGuard {
            prev_guard,
            guard: self.guard.clone(),
            cache1: self.substitution_cache_id.clone(),
            cache2: self.substitution_cache_grp.clone(),
        }
    }

    pub fn guard(&self) -> RwLockReadGuard<'_, Guard> {
        self.guard.read().expect("Not concurrent")
    }

    pub fn guard_mut(&self) -> RwLockWriteGuard<'_, Guard> {
        self.clear_cache();
        self.guard.write().expect("Not concurrent")
    }

    pub fn in_arm(&mut self, arm: u32, chance: u32) {
        self.guard_mut().in_arm(arm, chance);
    }

    pub fn truthify_guard(&mut self, guard: &Prop) {
        match guard {
            Prop::True => {}
            Prop::False => {}
            Prop::Var(Symbol::Random { block, arm, .. }) => {
                let mut current_guard = self.guard_mut();
                current_guard.truthy_block_arm.insert(*block, *arm);
                /* truthy block arm dominates falsy in a simplification */
                current_guard.falsy_block_arms.remove(block);
            }
            Prop::Var(Symbol::Name(id)) => {
                self.truthify(&id.0.clone());
            }
            Prop::And(et) => {
                for e in et {
                    self.truthify_guard(e)
                }
            }
            Prop::Or(_) => {}
            Prop::Not(Symbol::Random { block, arm, chance, .. }) => {
                let mut current_guard = self.guard_mut();
                let (chance_increase, arms) = current_guard.falsy_block_arms.entry(*block).or_default();
                *chance_increase += chance;
                arms.insert(*arm);
                /* For nested code, it is possible to have a different chosen arm, in which case falsy is meaningless */
                if current_guard.is_arm_chosen(*block, *arm) {
                    current_guard.truthy_block_arm.remove(block);
                }
            }
            Prop::Not(Symbol::Name(id)) => {
                self.falsify(&id.0.clone());
            }
        }
    }

    pub fn truthify(&mut self, v: &str) {
        self.guard_mut().truthify(v);
        let Some(IdInfo { guard, .. }) = self.identifiers.get(&v.into()) else { return };
        self.truthify_guard(&guard.clone());
    }

    pub fn falsify_guard(&mut self, guard: &Prop) {
        match guard {
            Prop::True => {}
            Prop::False => {}
            Prop::Var(Symbol::Random { block, arm, chance }) => {
                let mut current_guard = self.guard_mut();
                let (chance_increase, arms) = current_guard.falsy_block_arms.entry(*block).or_default();
                *chance_increase += chance;
                arms.insert(*arm);
                /* For nested code, it is possible to have a different chosen arm, in which case falsy is meaningless */
                if current_guard.is_arm_chosen(*block, *arm) {
                    current_guard.truthy_block_arm.remove(block);
                }
            }
            Prop::Var(Symbol::Name(id)) => {
                self.falsify(&id.0.clone());
            }
            Prop::And(_) => {}
            Prop::Or(vel) => {
                for v in vel {
                    self.falsify_guard(v)
                }
            }
            Prop::Not(Symbol::Random { block, arm, .. }) => {
                let mut current_guard = self.guard_mut();
                current_guard.truthy_block_arm.insert(*block, *arm);
                /* truthy block arm dominates falsy in a simplification */
                current_guard.falsy_block_arms.remove(block);
            }
            Prop::Not(Symbol::Name(id)) => {
                self.truthify(&id.0.clone());
            }
        }
    }

    pub fn falsify(&mut self, v: &str) {
        self.guard_mut().falsify(v);
        let Some(IdInfo { guard, .. }) = self.identifiers.get(&v.into()) else { return };
        self.falsify_guard(&guard.clone());
    }
    
    pub fn errs(&self) -> &HashMap<PathBuf, Vec<RmsError>> {
        &self.errs
    }
    
    pub fn new(include_dirs: Vec<PathBuf>, check_dead_paths: bool, skip_includes: bool) -> Self {
        Self {
            identifiers: HashMap::new(),
            object_groups: HashMap::new(),
            guard: Arc::new(RwLock::new(Guard::new())),

            substitution_cache_id: Arc::new(RwLock::new(HashMap::new())),
            substitution_cache_grp: Arc::new(RwLock::new(HashMap::new())),

            errs: HashMap::new(),

            include_dirs: Arc::new(include_dirs),
            dependencies: Some(HashMap::new()),

            current_ignores: Arc::new(RwLock::new(None)),

            last_block: 0,

            check_dead_paths,
            skip_includes,
        }
    }

    pub fn is_warning_ignored(&self, ignore: u32) -> bool {
        let current_ignores = self.current_ignores.read().expect("Not concurrent");
        let Some(ignores) = current_ignores.as_ref() else {
            return false;
        };
        ignores.contains(&ignore)
    }
    
    pub fn temp_ignore(&mut self, ignores: HashSet<u32>) -> TempIgnore {
        self.current_ignores.write().expect("Not concurrent").replace(ignores);
        TempIgnore { ignores: self.current_ignores.clone() }
    }

    pub fn clear_cache(&self) {
        self.substitution_cache_grp.write().expect("Not concurrent").clear();
        self.substitution_cache_id.write().expect("Not concurrent").clear();
    }

    pub fn get_id_mut(&mut self, id: &Identifier) -> Option<&mut IdInfo> {
        if self.identifiers.contains_key(id) {
            self.clear_cache();
        }
        self.identifiers.get_mut(id)
    }

    pub fn get_group_mut(&mut self, id: &Identifier) -> Option<&mut IdInfo> {
        if self.identifiers.contains_key(id) {
            self.clear_cache();
        }
        self.object_groups.get_mut(id)
    }

    pub fn is_dead_id(&self, id: &Identifier) -> bool {
        self.identifiers.contains_key(id) && self.check_live(id, false) == Liveness::Dead
    }

    pub fn get_mut(&mut self, id: &Identifier) -> Option<&mut IdInfo> {
        if self.is_dead_id(id) {
            self.object_groups
                .get_mut(id)
                .or_else(|| self.identifiers.get_mut(id))
        } else {
            self.identifiers
                .get_mut(id)
                .or_else(|| self.object_groups.get_mut(id))
        }
    }

    pub fn get_ref(&self, id: &Identifier) -> Option<&IdInfo> {
        if self.is_dead_id(id) {
            self.object_groups
                .get(id)
                .or_else(|| self.identifiers.get(id))
        } else {
            self.identifiers
                .get(id)
                .or_else(|| self.object_groups.get(id))
        }
    }

    pub fn get(&self, id: &Identifier) -> Option<IdInfo> {
        self.get_ref(id).cloned()
    }
    
    pub fn set(&mut self, id: &Identifier, info: IdInfo) {
        if info.type_ == Type::ObjectGroup {
            self.object_groups.insert(id.clone(), info);
        } else {
            self.identifiers.insert(id.clone(), info);
        }
    }

    fn process_err(&mut self, err: &mut RmsError) {
        if !err.is_warning() {
            return;
        }
        let to_ignore = self.is_warning_ignored(err.code());
        match err {
            RmsError::Warning { ignored, .. } => *ignored = to_ignore,
            _ => unreachable!("is_warning check above"),
        }
    }
    
    pub fn add_err(&mut self, path: &PathBuf, mut err: RmsError) {
        self.process_err(&mut err);
        
        self.errs
            .entry(path.clone())
            .or_insert(vec![])
            .push(err);
    }
    
    pub fn add_errs(&mut self, path: &PathBuf, mut errs: Vec<RmsError>) {
        for err in errs.iter_mut() {
            self.process_err(err);
        }
        
        self.errs
            .entry(path.clone())
            .or_insert(vec![])
            .extend(errs);
    }

    pub fn substitute_singletons(
        &self,
        prop: &Prop,
        guard: &Guard,
        seen: &mut HashSet<Identifier>,
        group_name: bool
    ) -> Prop {
        match prop {
            Prop::True | Prop::False => prop.clone(),
            Prop::Var(Symbol::Name(id)) | Prop::Not(Symbol::Name(id)) => {
                let result = 'result: {
                    let cached_result = if group_name {
                        self.substitution_cache_grp.read().expect("Not concurrent").get(id).cloned()
                    } else {
                        self.substitution_cache_id.read().expect("Not concurrent").get(id).cloned()
                    };
                    if let Some(cached_result) = cached_result {
                        break 'result cached_result;
                    }

                    if !seen.insert(id.clone()) {
                        break 'result Prop::False;
                    }
                    if guard.is_true(id) {
                        break 'result Prop::True;
                    }
                    if guard.is_false(id) {
                        break 'result Prop::False;
                    }
                    let info = if group_name {
                        self.object_groups.get(id)
                    } else {
                        self.identifiers.get(id)
                    };
                    let Some(info) = info else {
                        break 'result Prop::Var(Symbol::Name(id.clone()));
                    };
                    let r = self.substitute_singletons(&info.guard, guard, seen, false);
                    seen.remove(id);
                    let r = if r.is_singleton() {
                        r
                    } else {
                        Prop::Var(Symbol::Name(id.clone()))
                    };
                    if group_name {
                        self.substitution_cache_grp.write().expect("Not Concurrent").insert(id.clone(), r.clone());
                    } else {
                        self.substitution_cache_id.write().expect("Not Concurrent").insert(id.clone(), r.clone());
                    }
                    r
                };
                if prop.is_not() {
                    result.not()
                } else {
                    result
                }
            }
            Prop::Var(s @ Symbol::Random { .. }) => guard.lookup(s),
            Prop::Not(s @ Symbol::Random { .. }) => guard.lookup(s).not(),
            Prop::And(et) => {
                et.iter()
                    .map(|prop| self.substitute_singletons(prop, guard, seen, false))
                    .collect::<Vec<_>>()
                    .simplify_and(Some(guard))
            }
            Prop::Or(vel) => {
                vel.iter()
                    .map(|prop| self.substitute_singletons(prop, guard, seen, false))
                    .collect::<Vec<_>>()
                    .simplify_or(Some(guard))
            }
        }
    }

    pub fn check_live(&self, id: &Identifier, group_name: bool) -> Liveness {
        let guard = self.guard();
        let var_guard = self.substitute_singletons(&Prop::from_id(id), &guard, &mut HashSet::new(), group_name);
        let prop = var_guard.simplify(&guard);

        match prop {
            Prop::True => Liveness::Live,
            Prop::False => Liveness::Dead,
            Prop::Var(Symbol::Name(id)) => {
                if group_name && self.object_groups.contains_key(&id) || !group_name && self.identifiers.contains_key(&id) {
                    Liveness::Maybe
                } else {
                    Liveness::Dead
                }
            }
            _ => Liveness::Maybe,
        }
    }
}