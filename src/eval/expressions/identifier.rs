use crate::{EvalResult, EvaluationError, Evaluator, Literal, Object};

impl Evaluator<'_> {
    pub fn evaluate_identifier(&mut self, ident: Literal) -> EvalResult<Object> {
        let value = self.ctx.get_addr(&ident);

        match value {
            Some(exist) => match Object::copy_bits(exist) {
                Some(copied) => copied.into(),
                // This object cannot copy its own bits
                // We create a deep copy
                None => self.ctx.drop(&ident).unwrap().into(),
            },
            None => EvalResult::Err(EvaluationError::Undefined(ident)),
        }
    }
}
