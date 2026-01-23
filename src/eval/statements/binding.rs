use crate::{EvalResult, Evaluator, Object, statements::Binding};

impl Evaluator<'_> {
    pub fn evaluate_binding(&mut self, binding: Binding) -> EvalResult<Object> {
        let result = self.evaluate_expression(binding.value);

        match result {
            EvalResult::Value(value) => self.ctx.define(binding.ident, value).into(),
            EvalResult::NoValue => self.ctx.define(binding.ident, Object::NULL).into(),
            EvalResult::Err(_) => result,
        }
    }
}
