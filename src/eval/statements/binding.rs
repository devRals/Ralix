use crate::{EvalResult, Evaluator, Object, statements::Binding, try_eval_result};

impl Evaluator<'_> {
    pub fn evaluate_binding(&mut self, binding: Binding) -> EvalResult<Object> {
        let value = try_eval_result!(self.evaluate_expression(binding.value));
        let value_ty = binding.type_annotation.unwrap_or(value.r#type());

        self.ctx.define(binding.ident, value, value_ty).into()
    }
}
