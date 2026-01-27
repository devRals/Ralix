use crate::{EvalResult, Evaluator, Expression, Object, try_eval_result, types::Type};

impl Evaluator<'_> {
    pub fn evaluate_typeof_expression(&mut self, expr: Expression) -> EvalResult<Object> {
        if let Expression::Scope { statements } = &expr
            && statements.is_empty()
        {
            return EvalResult::Value(Object::Type(Type::Void));
        }

        let obj = try_eval_result!(self.evaluate_expression(expr));

        EvalResult::Value(Object::Type(obj.r#type()))
    }
}
