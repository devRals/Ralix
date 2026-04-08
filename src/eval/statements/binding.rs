use crate::{EvalResult, Evaluator, Expression, Value, expressions::Identifier};

impl Evaluator<'_> {
    pub fn evaluate_binding(&mut self, ident: Identifier, value: Expression) -> EvalResult<Value> {
        let result = self.evaluate_expression(value);

        match result {
            EvalResult::Value(value) => self.ctx.define(ident, value),
            EvalResult::NoValue => self.ctx.define(ident, Value::NULL),
            EvalResult::Err(_) | EvalResult::Return(_) => return result,
        };

        EvalResult::NoValue
    }
}
