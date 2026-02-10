use std::fmt::Display;

use crate::{Object, types::Type};

#[derive(Clone, PartialEq, Debug)]
pub struct Addr(usize, Type);

impl std::ops::Deref for Addr {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<addr to a `{}`>", self.1)
    }
}

impl Addr {
    pub const fn new(addr: usize, ty: Type) -> Addr {
        Addr(addr, ty)
    }

    pub fn read_from<'a>(&self, heap: &'a Heap) -> Option<&'a Object> {
        heap.read(self)
    }

    pub fn read_mut_from<'a>(&self, heap: &'a mut Heap) -> Option<&'a mut Object> {
        heap.read_mut(self)
    }

    pub fn r#type(&self) -> Type {
        self.1.clone()
    }
}

pub struct Heap {
    store: Vec<Object>,
}

impl Heap {
    pub const fn new() -> Heap {
        Heap { store: Vec::new() }
    }

    pub fn alloc(&mut self, value: Object) -> Addr {
        let ty = value.r#type();
        let addr = self.store.len();
        self.store.push(value);

        Addr(addr, ty)
    }

    pub fn read(&self, addr: &Addr) -> Option<&Object> {
        self.store.get(addr.0)
    }

    pub fn read_mut(&mut self, addr: &Addr) -> Option<&mut Object> {
        self.store.get_mut(addr.0)
    }

    pub fn drop(&mut self, addr: Addr) -> Option<Object> {
        if addr.0 < self.store.len() {
            None
        } else {
            Some(self.store.remove(addr.0))
        }
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
