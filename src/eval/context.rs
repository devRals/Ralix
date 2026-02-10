use crate::{Addr, Environment, Heap, Object, expressions::Identifier};

pub struct Context<'env> {
    pub(super) environment: &'env mut Environment,
    pub(super) heap: &'env mut Heap,
}

impl Context<'_> {
    pub fn define(&mut self, key: Identifier, value: Object) {
        let addr = self.heap.alloc(value);
        self.environment.define(key, addr);
    }

    pub fn drop(&mut self, name: &Identifier) -> Option<Object> {
        let addr = self.environment.drop(name)?;
        self.heap.drop(addr)
    }

    pub fn get_addr(&mut self, name: &Identifier) -> Option<&Addr> {
        self.environment.get_ptr(name)
    }

    pub fn get_addr_cloned(&self, name: &Identifier) -> Option<Addr> {
        self.environment.get_ptr(name).cloned()
    }

    pub fn get_cloned(&mut self, name: &Identifier) -> Option<Object> {
        let addr = self.environment.get_ptr(name)?;
        self.heap.read(addr).cloned()
    }

    pub fn get(&mut self, name: &Identifier) -> Option<&Object> {
        let addr = self.environment.get_ptr(name)?;
        self.heap.read(addr)
    }

    pub fn get_mut(&mut self, key: &Identifier) -> Option<&mut Object> {
        let addr = self.environment.get_ptr(key)?;
        self.heap.read_mut(addr)
    }

    pub fn enter_scope(&mut self) {
        self.environment.enter_scope()
    }

    pub fn leave_scope(&mut self) {
        self.environment.leave_scope()
    }
}
