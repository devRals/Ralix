use crate::{EvalResult, EvaluationError, Evaluator, Literal, Object};

impl Evaluator<'_> {
    pub fn evaluate_addr_expression(&mut self, ident: Literal) -> EvalResult<Object> {
        let addr = match self.ctx.get_addr(&ident) {
            Some(o) => o,
            None => return EvalResult::Err(EvaluationError::Undefined(ident)),
        };

        Object::Address(addr as *const Object).into()
    }
}
