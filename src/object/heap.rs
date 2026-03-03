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
    /// Constructor for [`Heap`]
    pub const fn new() -> Heap {
        Heap { store: Vec::new() }
    }

    /// Allocates a new location for the given `value`
    pub fn alloc(&mut self, value: Object) -> Addr {
        let ty = value.r#type();
        let addr = self.store.len();
        self.store.push(value);

        Addr(addr, ty)
    }

    pub fn update(&mut self, addr: Addr, value: Object) -> Option<Addr> {
        let holder = self.store.get_mut(addr.0)?;
        *holder = value;
        Some(addr)
    }

    pub fn read(&self, addr: &Addr) -> Option<&Object> {
        self.store.get(addr.0)
    }

    pub fn read_mut(&mut self, addr: &Addr) -> Option<&mut Object> {
        self.store.get_mut(addr.0)
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
