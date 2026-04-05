use crate::{CheckerResult, Literal, TypeChecker, TypeCheckerDiagnostic, types::Type};

impl TypeChecker<'_> {
    pub fn infer_identifier(&mut self, ident: &Literal) -> CheckerResult<Type> {
        match self.symbol_table.resolve(ident) {
            Some(v) => Ok(v.ty.clone()),
            None => Err(TypeCheckerDiagnostic::Undefined(ident.clone())),
        }
    }
}
