use std::{collections::HashMap, fmt::Display};

use crate::Value;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct Addr(usize);

impl Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<addr>")
    }
}

impl Addr {
    pub fn read_from<'a>(&self, heap: &'a Heap) -> Option<&'a Value> {
        heap.read(self)
    }

    pub fn read_mut_from<'a>(&self, heap: &'a mut Heap) -> Option<&'a mut Value> {
        heap.read_mut(self)
    }
}

type HeapStore = HashMap<Addr, Value>;

#[derive(Debug)]
pub struct Heap {
    store: HashMap<Addr, Value>,
}

impl Heap {
    /// Constructor for [`Heap`]
    pub fn new() -> Heap {
        Heap {
            store: HeapStore::new(),
        }
    }

    /// Allocates a new location for the given `value`
    pub fn alloc(&mut self, value: Value) -> Addr {
        let addr = Addr(self.store.len());
        self.store.insert(addr, value);

        addr
    }

    pub fn update(&mut self, addr: Addr, value: Value) -> Option<Addr> {
        let holder = self.store.get_mut(&addr)?;
        *holder = value;
        Some(addr)
    }

    pub fn read(&self, addr: &Addr) -> Option<&Value> {
        self.store.get(addr)
    }

    pub fn read_mut(&mut self, addr: &Addr) -> Option<&mut Value> {
        self.store.get_mut(addr)
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn free(&mut self, addr: &Addr) -> Option<Value> {
        self.store.remove(addr)
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
