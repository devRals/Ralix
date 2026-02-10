use std::collections::HashMap;

use crate::{
    Addr, EvalResult, Evaluator, Expression, HashKey, HashPair, Object,
    expressions::PrefixOperator, try_eval_result,
};

impl Evaluator<'_> {
    pub fn evaluate_assignment_statement(
        &mut self,
        left: Expression,
        value: Expression,
    ) -> EvalResult<Object> {
        let value_obj = try_eval_result!(self.evaluate_expression(value));
        let left_addr = self.eval_lhs(left);

        if let Some(addr) = left_addr
            && let Some(o) = addr.read_mut_from(self.ctx.heap)
        {
            *o = value_obj;
        }

        EvalResult::NoValue
    }

    pub fn eval_lhs(&mut self, expr: Expression) -> Option<Addr> {
        match expr {
            Expression::Identifier(ident) => self.ctx.get_addr_cloned(&ident),
            Expression::Index { left, index } => self.eval_index_lhs(*left, *index),
            Expression::Prefix {
                operator: PrefixOperator::Deref,
                right,
            } => self.eval_deref_lhs(*right),

            _ => None,
        }
    }

    fn eval_index_lhs(&mut self, left: Expression, index: Expression) -> Option<Addr> {
        let index_obj = match self.evaluate_expression(index) {
            EvalResult::Value(v) => v,
            EvalResult::Err(_) | EvalResult::NoValue | EvalResult::Return(_) => {
                return None;
            }
        };

        let left_addr = self.eval_lhs(left)?;
        let left = left_addr.read_from(self.ctx.heap)?;

        match (left, index_obj) {
            (Object::HashMap(hm), index) => self.eval_hashmap_index_lhs(hm, index),
            (Object::Array(arr), Object::Int(i)) => self.eval_array_index_lhs(arr, i as usize),
            _ => None,
        }
    }

    fn eval_deref_lhs(&mut self, right: Expression) -> Option<Addr> {
        let right_lhs_addr = self.eval_lhs(right)?;
        let right_lhs = self.ctx.heap.read(&right_lhs_addr)?;

        match right_lhs {
            Object::Address(addr) => Some(addr.clone()),
            _ => None,
        }
    }

    // BUG: This operation doesn't insert any value to the hash_map
    fn eval_hashmap_index_lhs(
        &self,
        hm: &HashMap<HashKey, HashPair>,
        index: Object,
    ) -> Option<Addr> {
        let hash_key = index.hash_key()?;

        hm.get(&hash_key).map(|(_, v)| v).cloned()
    }

    fn eval_array_index_lhs(&self, arr: &[Addr], index: usize) -> Option<Addr> {
        arr.get(index).cloned()
    }
}
