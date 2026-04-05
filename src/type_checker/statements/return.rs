use crate::{CheckerResult, Expression, TypeChecker, TypeCheckerDiagnostic, types::Type};

impl TypeChecker<'_> {
    pub fn check_return_statement(&mut self, expr: Option<&Expression>) -> CheckerResult<()> {
        let expr_ty = match expr {
            Some(e) => self.check_expression(e)?,
            None => Type::Void,
        };
        let func_ty = self.current_fn_return_type();

        if !expr_ty.satisfies(func_ty) {
            Err(TypeCheckerDiagnostic::Unsatisfied(expr_ty, func_ty.clone()))
        } else {
            Ok(())
        }
    }
}
