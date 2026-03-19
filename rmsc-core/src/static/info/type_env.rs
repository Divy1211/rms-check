use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use chumsky::container::{Container};

use crate::parsing::{Identifier};
use crate::r#static::info::id_info::IdInfo;
use crate::r#static::info::rms_error::RmsError;
use crate::r#static::type_check::propositions::{Guard, Prop, Symbol};

#[derive(Debug, Clone)]
pub enum Liveness {
    Live,
    Dead,
    Maybe,
}

impl Liveness {
    pub fn not(self) -> Liveness {
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
    pub errs: HashMap<PathBuf, Vec<RmsError>>,
    
    pub current_ignores: Arc<RwLock<Option<HashSet<u32>>>>,
    
    pub include_dirs: Arc<Vec<PathBuf>>,
    pub dependencies: Option<HashMap<PathBuf, HashSet<PathBuf>>>,

    pub last_block: u32
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

    pub fn in_arm(&self, arm: u32, chance: u32) {
        self.guard.write().expect("Not concurrent").in_arm(self.last_block, arm, chance);
    }

    pub fn truthify(&self, v: &str) {
        self.guard.write().expect("Not concurrent").truthify(self.last_block, v);
    }
    pub fn falsify(&self, v: &str) {
        self.guard.write().expect("Not concurrent").falsify(self.last_block, v);
    }
    
    pub fn errs(&self) -> &HashMap<PathBuf, Vec<RmsError>> {
        &self.errs
    }
    
    pub fn new(include_dirs: Vec<PathBuf>) -> Self {
        Self {
            identifiers: HashMap::new(),
            guard: Arc::new(RwLock::new(Guard::new())),
            errs: HashMap::new(),

            include_dirs: Arc::new(include_dirs),
            dependencies: Some(HashMap::new()),

            current_ignores: Arc::new(RwLock::new(None)),

            last_block: 0,
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

    pub fn set_global(&mut self, id: &Identifier, info: IdInfo) {
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

    pub fn check_live(&mut self, id: &Identifier) -> Liveness {
        let _nested = self.nested_guard();
        for (var, info) in &self.identifiers {
            match info.guard {
                Prop::True => self.truthify(&var.0),
                Prop::False => self.falsify(&var.0),
                _ => {},
            }
        }

        let guard = self.guard.clone();
        let Some(info) = self.identifiers.get(id) else {
            return Liveness::Dead;
        };

        match info.guard.simplify(&guard.read().expect("Not concurrent")) {
            Prop::True => Liveness::Live,
            Prop::False => Liveness::Dead,
            _ => Liveness::Maybe,
        }
    }
}