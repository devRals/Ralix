use crate::{CheckerResult, Statement, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn infer_scope_expression(&mut self, statements: &[Statement]) -> CheckerResult<Type> {
        self.symbol_table.enter_scope();

        let mut last_stmt_ty = Type::Void;

        for stmt in statements {
            if let Some(stmt_ty) = self.check_statement(stmt)? {
                last_stmt_ty = stmt_ty
            }
        }

        self.symbol_table.leave_scope();
        Ok(last_stmt_ty)
    }
}
