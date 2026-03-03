use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker,
    expressions::{Identifier, PrefixOperator},
    type_checker::statements::binding::infer_generics,
};

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
        let mut value_ty = self.check_expression(value)?;

        if let Expression::Index { .. } = left {
            left_ty = left_ty.unwrap_nullable()
        }

        infer_generics(&left_ty, &mut value_ty);
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
            Expression::Prefix {
                operator: PrefixOperator::Deref,
                right,
            } => self.value_exists_and_is_a_constant(right),
            _ => false,
        }
    }
}
