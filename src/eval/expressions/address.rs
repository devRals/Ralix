use crate::{EvalResult, EvaluationError, Evaluator, Literal, Object};

impl Evaluator<'_> {
    pub fn evaluate_addr_expression(&mut self, ident: Literal) -> EvalResult<Object> {
        let addr = match self.ctx.get_mut(&ident) {
            Some(o) => o,
            None => return EvalResult::Err(EvaluationError::Undefined(ident)),
        };

        Object::Address(addr as *mut Object).into()
    }
}
