use std::collections::HashMap;

use crate::{Object, expressions::Identifier};

/// [`Environment`] Scope
pub type EnvScope = HashMap<Identifier, Object>;

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<EnvScope>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            scopes: vec![EnvScope::new()],
        }
    }
}

impl Environment {
    pub fn define(&mut self, key: Identifier, value: Object) -> Option<Object> {
        if let Some(last_scope) = self.scopes.last_mut()
            && let Some(old_v) = last_scope.insert(key, value)
        {
            return old_v.into();
        }
        None
    }

    /// Drops it if exists
    pub fn drop(&mut self, key: &Identifier) -> Option<Object> {
        for s in self.scopes.iter_mut().rev() {
            if let Some(val) = s.remove(key) {
                return val.into();
            }
        }
        None
    }

    /// Deep copies the value its holding and returns it
    /// Does not drops the value
    pub fn get_cloned(&self, key: &Identifier) -> Option<Object> {
        for s in self.scopes.iter().rev() {
            if let Some(val) = s.get(key).cloned() {
                return val.into();
            }
        }
        None
    }

    /// Can be used when changing the already bindings values
    /// `let a = 3; a = 5;` for example
    pub fn get_mut(&mut self, key: &Identifier) -> Option<&mut Object> {
        for s in self.scopes.iter_mut().rev() {
            if let Some(val) = s.get_mut(key) {
                return val.into();
            }
        }
        None
    }

    /// Returns immutable referance
    pub fn get_ptr(&self, key: &Identifier) -> Option<&Object> {
        for s in self.scopes.iter().rev() {
            if let Some(val) = s.get(key) {
                return val.into();
            }
        }
        None
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(EnvScope::new());
    }

    pub fn leave_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn extend_from(&mut self, other: &EnvScope) {
        for (ident, value) in other {
            self.define(ident.clone(), value.clone());
        }
    }

    /// Clones the last scope
    pub fn current_scope(&self) -> EnvScope {
        self.scopes.last().unwrap().clone()
    }
}
