use crate::{Environment, Object, expressions::Identifier};

pub struct Context<'env> {
    pub(super) environment: &'env mut Environment,
}

impl Context<'_> {
    pub fn define(&mut self, key: Identifier, value: Object) -> Option<Object> {
        self.environment.define(key, value)
    }

    pub fn drop(&mut self, name: &Identifier) -> Option<Object> {
        self.environment.drop(name)
    }

    pub fn get_cloned(&mut self, name: &Identifier) -> Option<Object> {
        self.environment.get_cloned(name)
    }

    pub fn get_addr(&mut self, name: &Identifier) -> Option<&Object> {
        self.environment.get_ptr(name)
    }

    pub fn get_mut(&mut self, key: &Identifier) -> Option<&mut Object> {
        self.environment.get_mut(key)
    }

    pub fn enter_scope(&mut self) {
        self.environment.enter_scope()
    }

    pub fn leave_scope(&mut self) {
        self.environment.leave_scope()
    }
}
