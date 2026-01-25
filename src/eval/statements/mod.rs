use crate::{EvalResult, Evaluator, Object, Statement};

mod assignment;
mod binding;

impl Evaluator<'_> {
    pub fn evaluate_statement(&mut self, stmt: Statement) -> EvalResult<Object> {
        match stmt {
            Statement::Binding { ident, value, .. } => self.evaluate_binding(ident, value),
            Statement::Expression(expr) => self.evaluate_expression(expr),
            Statement::Assign { left, value } => self.evaluate_assignment_statement(left, value),
        }
    }
}
