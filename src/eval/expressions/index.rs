use crate::{EvalResult, EvaluationError, Evaluator, Expression, Object, try_eval_result};

impl Evaluator<'_> {
    pub fn evaluate_index_expression(
        &mut self,
        left: Expression,
        index: Expression,
    ) -> EvalResult<Object> {
        let left_obj = try_eval_result!(self.evaluate_expression(left));
        let index_obj = try_eval_result!(self.evaluate_expression(index));

        match (left_obj, index_obj) {
            (Object::Array(arr), Object::Int(index)) => arr
                .get(index as usize)
                .cloned()
                .unwrap_or(Object::NULL)
                .into(),
            (Object::HashMap(hash_map), index) => {
                let hash_key = index.hash_key().unwrap();

                hash_map
                    .get(&hash_key)
                    .cloned()
                    .map(|(_, v)| v)
                    .unwrap_or(Object::NULL)
                    .into()
            }
            (o1, o2) => EvalResult::Err(EvaluationError::UnsupportedIndexOperation(
                o1.r#type(),
                o2.r#type(),
            )),
        }
    }
}
