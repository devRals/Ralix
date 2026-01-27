use crate::{EvalResult, Evaluator, Expression, Object, expressions::Identifier};

impl Evaluator<'_> {
    pub fn evaluate_binding(&mut self, ident: Identifier, value: Expression) -> EvalResult<Object> {
        let result = self.evaluate_expression(value);

        match result {
            EvalResult::Value(value) => self.ctx.define(ident, value).into(),
            EvalResult::NoValue => self.ctx.define(ident, Object::NULL).into(),
            EvalResult::Err(_) | EvalResult::Return(_) => result,
        }
    }
}
