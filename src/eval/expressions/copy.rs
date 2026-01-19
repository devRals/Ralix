use crate::{EvalResult, EvaluationError, Evaluator, Literal, Object};

impl Evaluator<'_> {
    pub fn evaluate_copy_expression(&mut self, ident: Literal) -> EvalResult<Object> {
        let value = self.ctx.get_cloned(&ident);

        match value {
            Some(exist) => EvalResult::Value(exist),
            None => EvalResult::Err(EvaluationError::Undefined(ident)),
        }
    }
}
