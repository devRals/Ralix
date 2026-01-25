use std::collections::HashMap;

use crate::{Literal, types::Type};

pub type Scope = HashMap<Literal, Type>;
#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            scopes: vec![Scope::new()],
        }
    }
}

impl SymbolTable {
    /// returns scope id
    pub fn enter_scope(&mut self) -> usize {
        self.scopes.push(Scope::new());
        self.scopes.len()
    }
    pub fn leave_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn current_scope(&self) -> usize {
        self.scopes.len()
    }

    pub fn define(&mut self, name: Literal, typ: Type) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, typ);
        }
    }

    /// clones the type if exists
    pub fn resolve(&mut self, name: &Literal) -> Option<Type> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(typ) = scope.get(name).cloned() {
                return Some(typ);
            }
        }
        None
    }

    pub fn resolve_ref(&mut self, name: &Literal) -> Option<&Type> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(typ) = scope.get(name) {
                return Some(typ);
            }
        }
        None
    }
}
