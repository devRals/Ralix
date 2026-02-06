use crate::{CheckerError, CheckerResult, Expression, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn check_try_expression(&mut self, expr: &Expression) -> CheckerResult<Type> {
        let expr_ty = self.check_expression(expr)?;

        let current_fn_type = self.current_fn_return_type();

        if !(current_fn_type.is_nullish() || matches!(current_fn_type, Type::Nullable(_))) {
            return Err(CheckerError::CannotUseTry(current_fn_type.clone()));
        }

        if let Type::Nullable(ty) = expr_ty {
            Ok(*ty)
        } else {
            Err(CheckerError::IsNotNullable(expr_ty))
        }
    }
}
