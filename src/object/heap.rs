use crate::{Object, types::Type};

#[derive(Clone)]
pub struct Addr(usize, Type);

impl Addr {
    pub const fn new(addr: usize, ty: Type) -> Addr {
        Addr(addr, ty)
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
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
