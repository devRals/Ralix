use crate::{CheckerError, CheckerResult, Statement, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn infer_scope_expression(&mut self, statements: &[Statement]) -> CheckerResult<Type> {
        self.symbol_table.enter_scope();

        let mut last_stmt_ty = Type::Void;

        for stmt in statements {
            if let Statement::Return(v) = stmt {
                if let Some(ret_ty) = v {
                    let ret_ty = self.check_expression(ret_ty)?;
                    let cur_fn_ret_ty = self.current_fn_return_type();

                    if !ret_ty.satisfies(cur_fn_ret_ty) {
                        return Err(CheckerError::Unsatisfied(ret_ty, cur_fn_ret_ty.clone()));
                    }
                }
                self.symbol_table.leave_scope();
                return Ok(Type::Never);
            }

            if let Some(stmt_ty) = self.check_statement(stmt)? {
                last_stmt_ty = stmt_ty
            }
        }

        self.symbol_table.leave_scope();
        Ok(last_stmt_ty)
    }
}
