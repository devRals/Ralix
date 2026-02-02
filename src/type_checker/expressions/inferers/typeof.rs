use crate::{CheckerError, CheckerResult, Expression, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn check_typeof_expression(&mut self, expr: &Expression) -> CheckerResult<Type> {
        let expr_ty = self.check_expression(expr)?;

        if let Type::Nullable(_) = expr_ty {
            return Err(CheckerError::TypeofHadNullableExpr);
        }

        Ok(Type::AsValue(expr_ty.into()))
    }
}
