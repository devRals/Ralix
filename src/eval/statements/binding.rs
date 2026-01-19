use crate::{EvalResult, Evaluator, Object, statements::Binding, try_eval_result};

impl Evaluator<'_> {
    pub fn evaluate_binding(&mut self, binding: Binding) -> EvalResult<Object> {
        let value = try_eval_result!(self.evaluate(binding.value));

        self.ctx.define(binding.ident, value).into()
    }
}
