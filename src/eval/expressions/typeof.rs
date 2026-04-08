use crate::{EvalResult, Evaluator, Expression, Value, try_eval_result, types::Type};

impl Evaluator<'_> {
    pub fn evaluate_typeof_expression(&mut self, expr: Expression) -> EvalResult<Value> {
        if let Expression::Scope { statements } = &expr
            && statements.is_empty()
        {
            return EvalResult::Value(Value::Type(Type::Void));
        }

        let obj = try_eval_result!(self.evaluate_expression(expr));

        EvalResult::Value(Value::Type(obj.r#type(self.ctx.heap)))
    }
}
