use crate::{
    EvalResult, EvaluationError, Evaluator, Expression, Object, expressions::Identifier,
    try_eval_result,
};

impl Evaluator<'_> {
    pub fn evaluate_assignment_statement(
        &mut self,
        left: Expression,
        value: Expression,
    ) -> EvalResult<Object> {
        let value_obj = try_eval_result!(self.evaluate_expression(value));
        match left {
            Expression::Identifier(ident) => self.evaluate_identifier_assignment(ident, value_obj),

            e => EvalResult::Err(EvaluationError::CannotAssign(e, value_obj)),
        }
    }

    fn evaluate_identifier_assignment(
        &mut self,
        ident: Identifier,
        value: Object,
    ) -> EvalResult<Object> {
        let obj = match self.ctx.get_mut(&ident) {
            Some(o) => o,
            None => return EvalResult::Err(EvaluationError::Undefined(ident)),
        };

        *obj = value;
        EvalResult::NoValue
    }
}
