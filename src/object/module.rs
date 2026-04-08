use std::collections::HashMap;

use crate::{Addr, expressions::Identifier};

#[derive(Default, Debug)]
pub struct Module {
    exports: HashMap<Identifier, Addr>,
}

impl std::ops::Index<Identifier> for Module {
    type Output = Addr;
    fn index(&self, index: Identifier) -> &Self::Output {
        &self.exports[&index]
    }
}

impl Module {
    pub fn export(&mut self, ident: Identifier, value_addr: Addr) {
        self.exports.insert(ident, value_addr);
    }
}
