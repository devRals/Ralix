use std::collections::HashMap;

use crate::{
    Addr, EvalResult, Evaluator, Expression, HashKey, HashPair, Value, expressions::PrefixOperator,
    try_eval_result,
};

impl Evaluator<'_> {
    pub fn evaluate_assignment_statement(
        &mut self,
        left: Expression,
        value: Expression,
    ) -> EvalResult<Value> {
        let value_obj = try_eval_result!(self.evaluate_expression(value));
        let left_addr = self.addr_of(left);

        if let Some(addr) = left_addr
            && let Some(o) = addr.read_mut_from(self.ctx.heap)
        {
            *o = value_obj;
        }

        EvalResult::NoValue
    }

    pub fn addr_of(&mut self, expr: Expression) -> Option<Addr> {
        match expr {
            Expression::Identifier(ident) => self.ctx.get_addr(&ident),
            Expression::Index { left, index } => self.index_addr_of(*left, *index),
            Expression::Prefix {
                operator: PrefixOperator::Deref,
                right,
            } => self.deref_addr_of(*right),
            Expression::Try(inner) => self.addr_of(*inner),

            _ => None,
        }
    }

    fn index_addr_of(&mut self, left: Expression, index: Expression) -> Option<Addr> {
        let index_obj = match self.evaluate_expression(index) {
            EvalResult::Value(v) => v,
            EvalResult::Err(_) | EvalResult::NoValue | EvalResult::Return(_) => {
                return None;
            }
        };

        let left_addr = self.addr_of(left)?;
        let left = left_addr.read_from(self.ctx.heap)?;

        match (left, index_obj) {
            (Value::HashMap(hm), index) => self.hashmap_addr_of(hm, index),
            (Value::Array(arr), Value::Int(i)) => self.eval_array_index_lhs(arr, i as usize),
            _ => None,
        }
    }

    fn deref_addr_of(&mut self, right: Expression) -> Option<Addr> {
        let right_lhs_addr = self.addr_of(right)?;
        let right_lhs = self.ctx.heap.read(&right_lhs_addr)?;

        match right_lhs {
            Value::Pointer(addr) => Some(*addr),
            _ => None,
        }
    }

    // BUG: This operation doesn't insert any value to the hash_map
    fn hashmap_addr_of(&self, hm: &HashMap<HashKey, HashPair>, index: Value) -> Option<Addr> {
        let hash_key = index.hash_key()?;

        hm.get(&hash_key).map(|(_, v)| v).cloned()
    }

    fn eval_array_index_lhs(&self, arr: &[Addr], index: usize) -> Option<Addr> {
        arr.get(index).cloned()
    }
}
