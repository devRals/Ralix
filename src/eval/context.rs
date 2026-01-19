use crate::{Environment, Key, Object, SymbolTable};

pub struct Context<'env> {
    pub symbol_table: &'env mut SymbolTable,
    pub environment: &'env mut Environment,
}

impl Context<'_> {
    pub fn define(&mut self, key: Key, value: Object) -> Option<Object> {
        self.symbol_table.define(key.clone(), value.object_type());
        self.environment.define(key, value)
    }

    pub fn get(&mut self, name: &Key) -> Option<Object> {
        self.symbol_table.resolve(name);
        self.environment.get(name)
    }

    pub fn get_cloned(&mut self, name: &Key) -> Option<Object> {
        self.symbol_table.resolve_cloned(name);
        self.environment.get_cloned(name)
    }
}
