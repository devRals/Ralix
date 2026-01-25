use crate::{CheckerError, CheckerResult, Literal, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn infer_identifier(&mut self, ident: &Literal) -> CheckerResult<Type> {
        match self.symbol_table.resolve(ident) {
            Some(ty) => Ok(ty.clone()),
            None => Err(CheckerError::Undefined(ident.clone())),
        }
    }
}
