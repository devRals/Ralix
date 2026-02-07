use crate::{CheckerError, CheckerResult, Expression, TypeChecker};

impl TypeChecker<'_> {
    pub fn check_assignment_statement(
        &mut self,
        left: &Expression,
        value: &Expression,
    ) -> CheckerResult<()> {
        if let Expression::Identifier(ident) = left
            && let Some(found) = self.symbol_table.resolve_ref(ident)
            && found.is_constant
        {
            return Err(CheckerError::IsAConstant(ident.clone()));
        }

        let mut left_ty = self.check_expression(left)?;
        let value_ty = self.check_expression(value)?;

        if let Expression::Index { .. } = left {
            left_ty = left_ty.unwrap_nullable()
        }

        if value_ty.satisfies(&left_ty) {
            Ok(())
        } else {
            Err(CheckerError::Unsatisfied(left_ty, value_ty))
        }
    }
}
