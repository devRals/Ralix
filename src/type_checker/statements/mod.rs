use crate::{CheckerResult, Statement, TypeChecker, types::Type};

mod assignment;
mod binding;
mod r#return;

impl TypeChecker<'_> {
    pub fn check_statement(&mut self, stmt: &Statement) -> CheckerResult<Option<Type>> {
        match stmt {
            Statement::Binding {
                ident,
                type_annotation,
                value,
                is_constant,
            } => self
                .check_binding(ident, (*type_annotation).as_ref(), value, *is_constant)
                .map(|_| None),
            Statement::Expression(expr) => self.check_expression(expr).map(Some),
            Statement::Assign { left, value } => {
                self.check_assignment_statement(left, value).map(|_| None)
            }
            Statement::Return(expr) => self.check_return_statement(expr.as_ref()).map(|_| None),
            Statement::Alias { .. } => Ok(None),
        }
    }
}
