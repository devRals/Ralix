use crate::{EvalResult, Evaluator, Expression, Value, try_eval_result, types::Type};

impl Evaluator<'_> {
    pub fn evaluate_type_casting(&mut self, ty: Type, value: Expression) -> EvalResult<Value> {
        let value_obj = try_eval_result!(self.evaluate_expression(value));

        if ty == value_obj.r#type(self.ctx.heap) {
            return EvalResult::Value(value_obj);
        }

        EvalResult::Value(match (ty, value_obj) {
            (Type::Int, Value::Float(v)) => Value::Int(v as i64),
            (Type::Float, Value::Int(v)) => Value::Float(v as f64),
            (Type::String, Value::Char(c)) => Value::String(c.to_string().into()),
            (Type::String, Value::Int(v)) => Value::String(v.to_string().into()),
            (Type::String, Value::Float(v)) => Value::String(v.to_string().into()),
            _ => return EvalResult::NoValue,
        })
    }
}
