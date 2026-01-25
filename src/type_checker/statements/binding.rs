use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker, expressions::Identifier, types::Type,
};

impl TypeChecker<'_> {
    pub fn check_binding(
        &mut self,
        ident: &Identifier,
        type_annotation: Option<&Type>,
        value: &Expression,
    ) -> CheckerResult<()> {
        let value_ty = self.check_expression(value)?;
        if let Some(ty_a) = type_annotation {
            if value_ty.satisfies(ty_a) {
                self.symbol_table.define(ident.clone(), ty_a.clone());
                Ok(())
            } else {
                Err(CheckerError::Unsatisfied(value_ty, ty_a.clone()))
            }
        } else {
            self.symbol_table.define(ident.clone(), value_ty);
            Ok(())
        }
    }
}
