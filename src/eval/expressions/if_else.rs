use crate::{
    EvalResult, Evaluator, Expression, Object, expressions::IfConsequence, try_eval_result,
};

type ElseConsequence = Expression;

impl Evaluator<'_> {
    pub fn evaluate_if_else_expression(
        &mut self,
        consequences: Vec<IfConsequence>,
        else_consequence: Option<ElseConsequence>,
    ) -> EvalResult<Object> {
        for (condition, consequence) in consequences {
            let condition_obj = try_eval_result!(self.evaluate_expression(condition));

            if Object::is_true(&condition_obj) {
                let cons_obj = try_eval_result!(self.evaluate_expression(consequence));
                return EvalResult::Value(cons_obj);
            }
        }

        if let Some(else_cons) = else_consequence {
            let else_cons_obj = try_eval_result!(self.evaluate_expression(else_cons));
            return EvalResult::Value(else_cons_obj);
        }

        EvalResult::NoValue
    }
}
