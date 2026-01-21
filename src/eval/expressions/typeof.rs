use crate::{EvalResult, Evaluator, Expression, Object, try_eval_result};

impl Evaluator<'_> {
    pub fn evaluate_typeof_expression(&mut self, expr: Expression) -> EvalResult<Object> {
        let obj = try_eval_result!(self.evaluate_expression(expr));

        EvalResult::Value(Object::Type(obj.r#type()))
    }
}
