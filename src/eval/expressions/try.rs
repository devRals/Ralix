use crate::{EvalResult, Evaluator, Expression, Object};

impl Evaluator<'_> {
    pub fn evaluate_try_expression(&mut self, expr: Expression) -> EvalResult<Object> {
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
