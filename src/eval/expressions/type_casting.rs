use crate::{EvalResult, Evaluator, Expression, Literal, Object, try_eval_result, types::Type};

impl Evaluator<'_> {
    pub fn evaluate_type_casting(&mut self, ty: Type, value: Expression) -> EvalResult<Object> {
        let value_obj = try_eval_result!(self.evaluate_expression(value));

        if ty == value_obj.r#type() {
            return EvalResult::Value(value_obj);
        }

        EvalResult::Value(match (ty, value_obj) {
            (Type::Int, Object::Float(v)) => Object::Int(v as i64),
            (Type::Float, Object::Int(v)) => Object::Float(v as f64),
            (Type::String, Object::Char(c)) => Object::String(Literal::from(c.to_string())),
            (Type::String, Object::Int(v)) => Object::String(Literal::from(v.to_string())),
            (Type::String, Object::Float(v)) => Object::String(Literal::from(v.to_string())),
            _ => return EvalResult::NoValue,
        })
    }
}
