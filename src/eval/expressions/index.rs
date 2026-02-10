use std::collections::HashMap;

use crate::{
    Addr, EvalResult, EvaluationError, Evaluator, Expression, HashKey, HashPair, Object,
    try_eval_result,
};

impl Evaluator<'_> {
    pub fn evaluate_index_expression(
        &mut self,
        left: Expression,
        index: Expression,
    ) -> EvalResult<Object> {
        let left_obj = try_eval_result!(self.evaluate_expression(left));
        let index_obj = try_eval_result!(self.evaluate_expression(index));

        match (left_obj, index_obj) {
            (Object::Array(arr), Object::Int(index)) => {
                self.evaluate_array_index_expression(arr, index)
            }
            (Object::HashMap(hash_map), index) => {
                self.evaluate_hashmap_index_expression(hash_map, index)
            }
            (o1, o2) => EvalResult::Err(EvaluationError::UnsupportedIndexOperation(
                o1.r#type(),
                o2.r#type(),
            )),
        }
    }

    fn evaluate_array_index_expression(
        &mut self,
        arr: Vec<Addr>,
        index: i64,
    ) -> EvalResult<Object> {
        let Some(addr) = arr.get(index as usize) else {
            return Object::NULL.into();
        };

        self.ctx.heap.read(addr).cloned().into()
    }

    fn evaluate_hashmap_index_expression(
        &mut self,
        hash_map: HashMap<HashKey, HashPair>,
        index: Object,
    ) -> EvalResult<Object> {
        let hash_key = index.hash_key().unwrap();

        let Some((_, value_addr)) = hash_map.get(&hash_key) else {
            return Object::NULL.into();
        };

        self.ctx.heap.read(value_addr).cloned().into()
    }
}
