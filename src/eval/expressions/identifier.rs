use crate::{EvalResult, EvaluationError, Evaluator, Literal, Object};

impl Evaluator<'_> {
    pub fn evaluate_identifier(&mut self, ident: Literal) -> EvalResult<Object> {
        let value = self.ctx.get(&ident);

        match value {
            Some(exist) => match exist.copy_bits() {
                Some(copied) => copied.into(),
                None => self.ctx.get_cloned(&ident).unwrap().into(),
            },
            None => EvalResult::Err(EvaluationError::Undefined(ident)),
        }
    }
}
