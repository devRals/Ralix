use crate::{EvalResult, Evaluator, Literal, RuntimeError, Value};

impl Evaluator<'_> {
    pub fn evaluate_identifier(&mut self, ident: Literal) -> EvalResult<Value> {
        let value = self.ctx.get(&ident);

        match value {
            Some(exist) => match exist.copy_bits() {
                Some(copied) => copied.into(),
                None => self.ctx.get_cloned(&ident).unwrap().into(),
            },
            None => EvalResult::Err(RuntimeError::Undefined(ident)),
        }
    }
}
