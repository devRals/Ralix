use std::collections::HashMap;

use crate::{Literal, types::Type};

pub type Scope = HashMap<Literal, Type>;
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        let mut st = Self {
            scopes: vec![Scope::new()],
        };

        let primitive_types = HashMap::from([
            ("int", Type::String),
            ("int", Type::Int),
            ("str", Type::String),
            ("float", Type::Float),
            ("char", Type::Char),
            ("bool", Type::Bool),
            ("null", Type::Null),
        ]);

        for (lit, ty) in primitive_types {
            st.define(lit.into(), ty);
        }

        st
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

    pub fn resolve(&mut self, name: &Literal) -> Option<Type> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(typ) = scope.remove(name) {
                return Some(typ);
            }
        }
        None
    }

    pub fn resolve_cloned(&mut self, name: &Literal) -> Option<Type> {
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
