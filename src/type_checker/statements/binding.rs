use crate::{CheckerError, CheckerResult, TypeChecker, statements::Binding};

impl TypeChecker<'_> {
    pub fn check_binding(&mut self, stmt: &Binding) -> CheckerResult<()> {
        if let Some(ty_a) = &stmt.type_annotation {
            let value_ty = self.check_expression(&stmt.value)?;
            if value_ty.satisfies(ty_a) {
                self.symbol_table.define(stmt.ident.clone(), value_ty);
                Ok(())
            } else {
                Err(CheckerError::Unsatisfied(ty_a.clone(), value_ty))
            }
        } else {
            Ok(())
        }
    }
}
