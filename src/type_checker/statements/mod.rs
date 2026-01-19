use crate::{CheckerResult, Statement, TypeChecker};

mod binding;

impl TypeChecker<'_> {
    pub fn check_statement(&mut self, stmt: &Statement) -> CheckerResult<()> {
        match stmt {
            Statement::Binding(binding) => self.check_binding(binding),
            Statement::Expression(expr) => self.check_expression(expr).map(|_| ()),
        }
    }
}
