use std::collections::HashMap;

use crate::{Literal, Object};

pub type Key = Literal;
type Store = HashMap<Key, Object>;

#[derive(Default, Debug)]
pub struct Environment {
    store: Store,
}

impl Environment {
    pub fn define(&mut self, key: Key, value: Object) -> Option<Object> {
        self.store.insert(key, value)
    }

    /// Drops it if exists
    pub fn get(&mut self, key: &Key) -> Option<Object> {
        self.store.remove(key)
    }

    /// Deep copies the value its holding and returns it
    /// Does not drops the value
    pub fn get_cloned(&mut self, key: &Key) -> Option<Object> {
        self.store.get(key).cloned()
    }

    /// Can be used when changing the already bindings values
    /// `let a = 3; a = 5;` for example
    pub fn get_mut(&mut self, key: &Key) -> Option<&mut Object> {
        self.store.get_mut(key)
    }

    /// Returns immutable referance
    pub fn get_ptr(&self, key: &Key) -> Option<&Object> {
        self.store.get(key)
    }
}
