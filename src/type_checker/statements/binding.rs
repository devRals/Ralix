use crate::{CheckerError, CheckerResult, TypeChecker, statements::Binding};

impl TypeChecker<'_> {
    pub fn check_binding(&mut self, stmt: &Binding) -> CheckerResult<()> {
        let value_ty = self.check_expression(&stmt.value)?;
        if let Some(ty_a) = &stmt.type_annotation {
            if value_ty.satisfies(ty_a) {
                self.symbol_table.define(stmt.ident.clone(), ty_a.clone());
                Ok(())
            } else {
                Err(CheckerError::Unsatisfied(value_ty, ty_a.clone()))
            }
        } else {
            self.symbol_table.define(stmt.ident.clone(), value_ty);
            Ok(())
        }
    }
}
