use crate::{EvalResult, Evaluator, Expression, Object, try_eval_result};

impl Evaluator<'_> {
    pub fn evaluate_assignment_statement(
        &mut self,
        left: Expression,
        value: Expression,
    ) -> EvalResult<Object> {
        let value_obj = try_eval_result!(self.evaluate_expression(value));
        let left_obj = self.eval_lhs(left);

        if let Some(o) = left_obj {
            let old_v = o.clone();
            *o = value_obj;

            return old_v.into();
        }

        EvalResult::NoValue
    }

    pub fn eval_lhs(&mut self, expr: Expression) -> Option<&mut Object> {
        match expr {
            Expression::Identifier(ident) => self.ctx.get_mut(&ident),
            Expression::Index { left, index } => self.eval_index_lhs(*left, *index),

            _ => None,
        }
    }

    fn eval_index_lhs(&mut self, left: Expression, index: Expression) -> Option<&mut Object> {
        let index_obj = match self.evaluate_expression(index) {
            EvalResult::Value(v) => v,
            EvalResult::Err(_) | EvalResult::NoValue | EvalResult::Return(_) => {
                return None;
            }
        };
        let left = self.eval_lhs(left)?;

        match (left, index_obj) {
            (Object::HashMap(hm), i) => hm.get_mut(&i.hash_key()?).map(|(_, v)| v),
            (Object::Array(arr), Object::Int(i)) => arr.get_mut(i as usize),
            _ => None,
        }
    }
}
