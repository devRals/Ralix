use crate::{EvalResult, Evaluator, Expression, Object, try_eval_result};

impl Evaluator<'_> {
    pub fn evaluate_array_literal(&mut self, items: Vec<Expression>) -> EvalResult<Object> {
        let mut values = Vec::new();
        for i in items {
            let item_obj = try_eval_result!(self.evaluate_expression(i));

            values.push(item_obj);
        }

        EvalResult::Value(Object::Array(values))
    }
}
