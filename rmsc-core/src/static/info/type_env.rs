use std::collections::{HashMap, HashSet};
use std::ops::Not;
use std::path::PathBuf;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use chumsky::container::{Container};

use crate::parsing::{Identifier};
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
}

#[derive(Debug, Clone)]
pub struct TypeEnv {
    pub identifiers: HashMap<Identifier, IdInfo>,
    pub guard: Arc<RwLock<Guard>>,
    pub chance_increases: HashMap<u32, u32>,
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
}

impl Drop for NestedGuard {
    fn drop(&mut self) {
        *self.guard.write().expect("Not concurrent") = self.prev_guard.take()
            .expect("Internal Error: No previous guard");
    }
}

impl TypeEnv {
    pub fn nested_guard(&mut self) -> NestedGuard {
        self.last_block += 1;
        NestedGuard {
            prev_guard: Some(self.guard.read().expect("Not concurrent").clone()),
            guard: self.guard.clone(),
        }
    }

    pub fn guard(&self) -> RwLockReadGuard<'_, Guard> {
        self.guard.read().expect("Not concurrent")
    }

    pub fn in_arm(&mut self, arm: u32, chance: u32) {
        self.guard.write().expect("Not concurrent").in_arm(self.last_block, arm, chance);
    }

    pub fn truthify(&mut self, v: &str) {
        self.guard.write().expect("Not concurrent").truthify(self.last_block, v);
    }
    pub fn falsify(&mut self, v: &str) {
        self.guard.write().expect("Not concurrent").falsify(self.last_block, v);
        let Some(IdInfo { guard: Prop::Var(Symbol::Random { block, chance, .. }), .. }) = self.identifiers.get(&v.into()) else { return };
        let chance_increases = self.chance_increases.entry(*block).or_insert(0);
        *chance_increases += *chance;
    }
    
    pub fn errs(&self) -> &HashMap<PathBuf, Vec<RmsError>> {
        &self.errs
    }
    
    pub fn new(include_dirs: Vec<PathBuf>, check_dead_paths: bool, skip_includes: bool) -> Self {
        Self {
            identifiers: HashMap::new(),
            guard: Arc::new(RwLock::new(Guard::new())),
            chance_increases: HashMap::new(),
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
    
    pub fn get_mut(&mut self, id: &Identifier) -> Option<&mut IdInfo> {
        self.identifiers.get_mut(id)
    }
    
    pub fn get(&self, id: &Identifier) -> Option<IdInfo> {
        self.identifiers.get(id).cloned()
    }
    
    pub fn set(&mut self, id: &Identifier, info: IdInfo) {
        self.identifiers.push((id.clone(), info))
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

    pub fn substitute_singletons(&self, prop: &Prop, guard: &Guard, seen: &mut HashSet<Identifier>) -> Prop {
        match prop {
            Prop::True | Prop::False => prop.clone(),
            Prop::Var(Symbol::Name(id)) | Prop::Not(Symbol::Name(id)) => {
                let result = 'result: {
                    if !seen.insert(id.clone()) {
                        break 'result Prop::False;
                    }
                    if guard.is_true(id) {
                        break 'result Prop::True;
                    }
                    if guard.is_false(id) {
                        break 'result Prop::False;
                    }
                    let Some(info) = self.identifiers.get(id) else {
                        break 'result Prop::Var(Symbol::Name(id.clone()));
                    };
                    let r = self.substitute_singletons(&info.guard, guard, seen);
                    seen.remove(id);
                    if r.is_singleton() {
                        r
                    } else {
                        Prop::Var(Symbol::Name(id.clone()))
                    }
                };
                if prop.is_not() {
                    result.not()
                } else {
                    result
                }
            }
            Prop::Var(Symbol::Random { .. }) | Prop::Not(Symbol::Random { .. }) => prop.clone(),
            Prop::And(et) => {
                et.iter()
                    .map(|prop| self.substitute_singletons(prop, guard, seen))
                    .collect::<Vec<_>>()
                    .simplify_and()
            }
            Prop::Or(vel) => {
                vel.iter()
                    .map(|prop| self.substitute_singletons(prop, guard, seen))
                    .collect::<Vec<_>>()
                    .simplify_or(Some(&self.chance_increases))
            }
        }
    }

    pub fn check_live(&self, id: &Identifier) -> Liveness {
        let guard = self.guard();
        let var_guard = self.substitute_singletons(&Prop::from_id(id), &guard, &mut HashSet::new());
        let prop = var_guard.simplify(&guard);

        match prop {
            Prop::True => Liveness::Live,
            Prop::False => Liveness::Dead,
            Prop::Var(Symbol::Name(id)) => {
                if /*id.is_default_name() ||*/ self.identifiers.contains_key(&id) {
                    Liveness::Maybe
                } else {
                    Liveness::Dead
                }
            }
            _ => Liveness::Maybe,
        }
    }
}