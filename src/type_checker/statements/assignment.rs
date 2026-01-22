use crate::{CheckerError, CheckerResult, Expression, TypeChecker};

impl TypeChecker<'_> {
    pub fn check_assignment_statement(
        &mut self,
        left: &Expression,
        value: &Expression,
    ) -> CheckerResult<()> {
        let left_ty = self.check_expression(left)?;
        let value_ty = self.check_expression(value)?;

        if value_ty.satisfies(&left_ty) {
            Ok(())
        } else {
            Err(CheckerError::Unsatisfied(left_ty, value_ty))
        }
    }
}
