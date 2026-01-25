use crate::{
    EvalResult, EvaluationError, Evaluator, Expression, Object,
    expressions::{InfixOperator, PrefixOperator},
    try_eval_result,
};
use std::ptr;

impl Evaluator<'_> {
    pub fn evaluate_infix_expression(
        &mut self,
        left: Expression,
        operator: InfixOperator,
        right: Expression,
    ) -> EvalResult<Object> {
        let left_obj = try_eval_result!(self.evaluate_expression(left));
        let right_obj = try_eval_result!(self.evaluate_expression(right));

        // All of the op implementions for `Object` returns an `EvalResult`
        match operator {
            InfixOperator::Add => left_obj + right_obj,
            InfixOperator::Subtract => left_obj - right_obj,
            InfixOperator::Multiply => left_obj * right_obj,
            InfixOperator::Divide => left_obj / right_obj,
            InfixOperator::Remainder => left_obj % right_obj,
            InfixOperator::Equals => Object::from(left_obj == right_obj).into(),
            InfixOperator::NotEquals => Object::from(left_obj != right_obj).into(),
            InfixOperator::Less => Object::from(left_obj < right_obj).into(),
            InfixOperator::LessEq => Object::from(left_obj <= right_obj).into(),
            InfixOperator::Greater => Object::from(left_obj > right_obj).into(),
            InfixOperator::GreatEq => Object::from(left_obj >= right_obj).into(),
            InfixOperator::Or => Self::evaluate_or(left_obj, right_obj),
            InfixOperator::And => Self::evaluate_and(left_obj, right_obj),
        }
    }

    pub fn evaluate_or(left: Object, right: Object) -> EvalResult<Object> {
        match (left, right) {
            (Object::Boolean(v1), Object::Boolean(v2)) => Object::from(v1 || v2).into(),
            (o1, o2) => EvalResult::Err(EvaluationError::UnsupportedInfixOperation(
                o1.r#type(),
                InfixOperator::Or,
                o2.r#type(),
            )),
        }
    }

    pub fn evaluate_and(left: Object, right: Object) -> EvalResult<Object> {
        match (left, right) {
            (Object::Boolean(v1), Object::Boolean(v2)) => Object::from(v1 && v2).into(),
            (o1, o2) => EvalResult::Err(EvaluationError::UnsupportedInfixOperation(
                o1.r#type(),
                InfixOperator::And,
                o2.r#type(),
            )),
        }
    }

    pub fn evaluate_prefix_expression(
        &mut self,
        operator: PrefixOperator,
        right: Expression,
    ) -> EvalResult<Object> {
        let obj = try_eval_result!(self.evaluate_expression(right));

        match operator {
            PrefixOperator::Not => !obj,
            PrefixOperator::Neg => -obj,
            PrefixOperator::Deref => self.evaluate_deref_expression(obj),
        }
    }

    pub fn evaluate_deref_expression(&mut self, obj: Object) -> EvalResult<Object> {
        match obj {
            Object::Address(addr) => unsafe { ptr::read(addr) }.into(),
            _ => unreachable!(),
        }
    }
}
