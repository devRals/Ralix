use crate::{EvalResult, Evaluator, Expression, Value, try_eval_result};

impl Evaluator<'_> {
    pub fn evaluate_array_literal(&mut self, items: Vec<Expression>) -> EvalResult<Value> {
        let mut values = Vec::new();
        for i in items {
            let item_obj = try_eval_result!(self.evaluate_expression(i));

            let addr = self.ctx.heap.alloc(item_obj);
            values.push(addr);
        }

        EvalResult::Value(Value::Array(values))
    }
}
