use crate::{CheckerError, CheckerResult, Expression, TypeChecker, expressions::Identifier};

impl TypeChecker<'_> {
    pub fn check_assignment_statement(
        &mut self,
        left: &Expression,
        value: &Expression,
    ) -> CheckerResult<()> {
        if self.value_exists_and_is_a_constant(left) {
            return Err(CheckerError::IsAConstant(Identifier::from(
                left.to_string(),
            )));
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

    fn value_exists_and_is_a_constant(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Identifier(ident) => {
                if let Some(found) = self.symbol_table.resolve_ref(ident) {
                    found.is_constant
                } else {
                    false
                }
            }

            Expression::Index { left, .. } => self.value_exists_and_is_a_constant(left),
            _ => false,
        }
    }
}
