use crate::{EvalResult, Evaluator, Object, Statement};

impl Evaluator<'_> {
    pub fn evaluate_scope_expression(&mut self, statements: Vec<Statement>) -> EvalResult<Object> {
        let mut result = EvalResult::NoValue;

        self.ctx.enter_scope();
        for s in statements {
            result = self.evaluate_statement(s);

            match &result {
                EvalResult::Value(_) => {}
                EvalResult::NoValue => {}
                EvalResult::Return(_) => {
                    self.ctx.leave_scope();
                    return result;
                }
                EvalResult::Err(err) => self.panic(err),
            }
        }
        self.ctx.leave_scope();

        result
    }
}
