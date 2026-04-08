use crate::{EvalResult, Evaluator, Expression, Value};

impl Evaluator<'_> {
    pub fn evaluate_try_expression(&mut self, expr: Expression) -> EvalResult<Value> {
        let obj = self.evaluate_expression(expr);

        match obj {
            EvalResult::Value(v) => {
                if v.is_null() {
                    EvalResult::Return(Some(v))
                } else {
                    v.into()
                }
            }

            v => v,
        }
    }
}
