use std::collections::HashMap;

use crate::{Literal, types::Type};

#[derive(Clone, Hash, Debug)]
pub struct ValueMetadata {
    pub ty: Type,
    pub is_constant: bool,
}

/// [`SymbolTable`] Scope
pub type STScope = HashMap<Literal, ValueMetadata>;
#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<STScope>,
    counter: u64,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            scopes: vec![STScope::new()],
            counter: 0,
        }
    }
}

impl SymbolTable {
    /// returns scope id
    pub fn enter_scope(&mut self) -> usize {
        self.scopes.push(STScope::new());
        self.scopes.len()
    }
    pub fn leave_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn current_scope(&self) -> usize {
        self.scopes.len()
    }

    pub fn define(&mut self, name: Literal, ty: Type, is_constant: bool) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, ValueMetadata { ty, is_constant });
        }
    }

    /// clones the type if exists
    pub fn resolve(&mut self, name: &Literal) -> Option<ValueMetadata> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(typ) = scope.get(name).cloned() {
                return Some(typ);
            }
        }
        None
    }

    pub fn resolve_ref(&self, name: &Literal) -> Option<&ValueMetadata> {
        for scope in self.scopes.iter().rev() {
            if let Some(typ) = scope.get(name) {
                return Some(typ);
            }
        }
        None
    }

    pub fn clear(&mut self) {
        self.scopes.clear();
        self.scopes.push(STScope::new());
    }

    pub fn crate_id(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }
}
