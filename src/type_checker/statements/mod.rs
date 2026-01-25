use crate::{CheckerResult, Statement, TypeChecker, types::Type};

mod assignment;
mod binding;

impl TypeChecker<'_> {
    pub fn check_statement(&mut self, stmt: &Statement) -> CheckerResult<Option<Type>> {
        match stmt {
            Statement::Binding {
                ident,
                type_annotation,
                value,
            } => self
                .check_binding(ident, (*type_annotation).as_ref(), value)
                .map(|_| None),
            Statement::Expression(expr) => self.check_expression(expr).map(Some),
            Statement::Assign { left, value } => {
                self.check_assignment_statement(left, value).map(|_| None)
            }
        }
    }
}
