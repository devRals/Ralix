use std::collections::HashMap;

use crate::{EvalResult, Evaluator, HashPair, Object, expressions::HashMapItem, try_eval_result};

impl Evaluator<'_> {
    pub fn evaluate_hashmap_literal(&mut self, items: Vec<HashMapItem>) -> EvalResult<Object> {
        let mut hash_map = HashMap::new();

        for i in items {
            let key_obj = try_eval_result!(self.evaluate_expression(i.key));
            let value_obj = try_eval_result!(self.evaluate_expression(i.value));

            let hash_key = key_obj.hash_key().unwrap();

            hash_map.insert(hash_key, HashPair::from((key_obj, value_obj)));
        }

        EvalResult::Value(Object::HashMap(hash_map))
    }
}
