use crate::{CheckerError, CheckerResult, Literal, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn infer_addrof_expression(&mut self, ident: &Literal) -> CheckerResult<Type> {
        let addr = match self.symbol_table.resolve(ident) {
            Some(a) => a,
            None => return Err(CheckerError::Undefined(ident.clone())),
        };

        Ok(Type::Addr(Box::new(addr.ty)))
    }
}
