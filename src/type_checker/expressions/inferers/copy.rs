use crate::{CheckerError, CheckerResult, Literal, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn infer_copy_expression(&mut self, ident: &Literal) -> CheckerResult<Type> {
        match self.symbol_table.resolve(ident) {
            Some(v) => Ok(v.ty.clone()),
            None => Err(CheckerError::Undefined(ident.clone())),
        }
    }
}
