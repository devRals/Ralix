use crate::{Addr, Environment, Heap, Value, expressions::Identifier, object::module::Module};

pub struct RuntimeContext<'env> {
    pub(crate) environment: &'env mut Environment,
    pub(crate) heap: &'env mut Heap,
    pub(crate) self_module: Module,
    pub(crate) module_cache: &'env mut super::module_cache::ModuleCache,
}

impl RuntimeContext<'_> {
    pub fn define(&mut self, key: Identifier, value: Value) {
        let addr = self.heap.alloc(value);
        self.environment.define(key, addr);
    }

    pub fn get_addr(&mut self, name: &Identifier) -> Option<Addr> {
        self.environment.get_ptr(name)
    }

    pub fn get_cloned(&mut self, name: &Identifier) -> Option<Value> {
        let addr = self.environment.get_ptr(name)?;
        self.heap.read(&addr).cloned()
    }

    pub fn get(&mut self, name: &Identifier) -> Option<&Value> {
        let addr = self.environment.get_ptr(name)?;
        self.heap.read(&addr)
    }

    pub fn get_mut(&mut self, key: &Identifier) -> Option<&mut Value> {
        let addr = self.environment.get_ptr(key)?;
        self.heap.read_mut(&addr)
    }

    pub fn enter_scope(&mut self) {
        self.environment.enter_scope()
    }

    pub fn leave_scope(&mut self) {
        self.environment.leave_scope();
    }
}
