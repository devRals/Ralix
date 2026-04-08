use crate::{
    EvalResult, Evaluator, Expression, RuntimeError, Value,
    expressions::{InfixOperator, PrefixOperator},
    try_eval_result,
};
use std::cmp;

impl cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(v1), Value::Int(v2)) => v1.partial_cmp(v2),
            (Value::Float(v1), Value::Float(v2)) => v1.partial_cmp(v2),
            (Value::Char(v1), Value::Char(v2)) => v1.partial_cmp(v2),
            _ => None,
        }
    }
}

enum BitShiftDirection {
    Left,
    Right,
}

impl Evaluator<'_> {
    pub fn evaluate_infix_expression(
        &mut self,
        left: Expression,
        operator: InfixOperator,
        right: Expression,
    ) -> EvalResult<Value> {
        use Value::*;

        let left_obj = try_eval_result!(self.evaluate_expression(left));
        let right_obj = try_eval_result!(self.evaluate_expression(right));

        macro_rules! impl_infix_op {
            ($op: ident => {
                $(
                    $pattern: pat => $result: expr
                ),* $(,)?
            }) => {
                match (left_obj, right_obj) {
                    $(
                        $pattern => $result
                    ),*
                    ,(o1, o2) => return EvalResult::Err(RuntimeError::UnsupportedInfixOperation(
                        o1.r#type(self.ctx.heap), InfixOperator::$op, o2.r#type(self.ctx.heap)
                    ))
                }
            };
        }

        // All of the op implementions for `Object` returns an `EvalResult`
        match operator {
            InfixOperator::Add => impl_infix_op!(Add => {
                (Int(v1), Int(v2)) => Value::from(v1 + v2),
                (Float(v1), Float(v2)) => Value::from(v1 + v2),
                (String(v1), String(v2)) => Value::from(std::string::String::from(&*v1) + &*v2),
            })
            .into(),

            InfixOperator::Subtract => impl_infix_op!(Subtract => {
                (Int(v1), Int(v2)) => Value::from(v1 - v2),
                (Float(v1), Float(v2)) => Value::from(v1 - v2)
            })
            .into(),

            InfixOperator::Multiply => impl_infix_op!(Multiply => {
                (Int(v1), Int(v2)) => Value::from(v1 * v2),
                (Float(v1), Float(v2)) => Value::from(v1 * v2)
            })
            .into(),

            InfixOperator::Divide => impl_infix_op!(Divide => {

                (Int(v1), Int(v2)) => Value::from(v1 / v2),
                (Float(v1), Float(v2)) => Value::from(v1 / v2)
            })
            .into(),
            InfixOperator::Remainder => impl_infix_op!(Remainder => {

                (Int(v1), Int(v2)) => Value::from(v1 % v2),
                (Float(v1), Float(v2)) => Value::from(v1 % v2)
            })
            .into(),
            InfixOperator::BitwiseAnd => impl_infix_op!(BitwiseAnd => {
                (Int(v1), Int(v2)) => Value::from(v1 & v2)
            })
            .into(),
            InfixOperator::BitwiseOr => impl_infix_op!(BitwiseOr => {
                (Int(v1), Int(v2)) => Value::from(v1 | v2)
            })
            .into(),
            InfixOperator::BitwiseXOr => impl_infix_op!(BitwiseXOr => {
                (Int(v1), Int(v2)) => Value::from(v1 ^ v2)
            })
            .into(),

            InfixOperator::Equals => Value::from(left_obj == right_obj).into(),
            InfixOperator::NotEquals => Value::from(left_obj != right_obj).into(),
            InfixOperator::Less => Value::from(left_obj < right_obj).into(),
            InfixOperator::LessEq => Value::from(left_obj <= right_obj).into(),
            InfixOperator::Greater => Value::from(left_obj > right_obj).into(),
            InfixOperator::GreatEq => Value::from(left_obj >= right_obj).into(),
            InfixOperator::Or => self.evaluate_or(left_obj, right_obj),
            InfixOperator::And => self.evaluate_and(left_obj, right_obj),
            InfixOperator::BitShiftLeft => {
                self.evaluate_bitshift(left_obj, right_obj, BitShiftDirection::Left)
            }
            InfixOperator::BitShiftRight => {
                self.evaluate_bitshift(left_obj, right_obj, BitShiftDirection::Right)
            }
        }
    }

    fn evaluate_bitshift(
        &self,
        left: Value,
        right: Value,
        dir: BitShiftDirection,
    ) -> EvalResult<Value> {
        let op = match dir {
            BitShiftDirection::Left => InfixOperator::BitShiftLeft,
            BitShiftDirection::Right => InfixOperator::BitShiftRight,
        };

        match (left, right) {
            (Value::Int(v1), Value::Int(v2)) => Value::Int(match dir {
                BitShiftDirection::Left => v1 << v2,
                BitShiftDirection::Right => v1 >> v2,
            })
            .into(),
            (o1, o2) => EvalResult::Err(RuntimeError::UnsupportedInfixOperation(
                o1.r#type(self.ctx.heap),
                op,
                o2.r#type(self.ctx.heap),
            )),
        }
    }

    fn evaluate_or(&self, left: Value, right: Value) -> EvalResult<Value> {
        match (left, right) {
            (Value::Boolean(v1), Value::Boolean(v2)) => Value::from(v1 || v2).into(),
            (o1, o2) => EvalResult::Err(RuntimeError::UnsupportedInfixOperation(
                o1.r#type(self.ctx.heap),
                InfixOperator::Or,
                o2.r#type(self.ctx.heap),
            )),
        }
    }

    fn evaluate_and(&self, left: Value, right: Value) -> EvalResult<Value> {
        match (left, right) {
            (Value::Boolean(v1), Value::Boolean(v2)) => Value::from(v1 && v2).into(),
            (o1, o2) => EvalResult::Err(RuntimeError::UnsupportedInfixOperation(
                o1.r#type(self.ctx.heap),
                InfixOperator::And,
                o2.r#type(self.ctx.heap),
            )),
        }
    }

    pub fn evaluate_prefix_expression(
        &mut self,
        operator: PrefixOperator,
        right: Expression,
    ) -> EvalResult<Value> {
        if let PrefixOperator::AddrOf = operator {
            return self.evaluate_addrof_expression(right);
        }

        let obj = try_eval_result!(self.evaluate_expression(right));

        macro_rules! impl_prefix_op {
            ($op: ident => {
                $(
                    $pattern: pat => $result: expr
                ),* $(,)?
            }) => {
                match obj {
                    $(
                        $pattern => $result,
                    )*
                    o => return EvalResult::Err(RuntimeError::UnsupportedPrefixOperation(PrefixOperator::$op, o.r#type(self.ctx.heap)))
                }
            };
        }

        match operator {
            PrefixOperator::Not => impl_prefix_op!(Not => {
                Value::Boolean(v) => Value::from(!v)
            })
            .into(),
            PrefixOperator::Neg => impl_prefix_op!(Neg => {
                Value::Int(v) => Value::from(-v),
                Value::Float(v) => Value::from(-v),
            })
            .into(),
            PrefixOperator::Deref => self.evaluate_deref_expression(obj),
            PrefixOperator::BitwiseNot => self.evaluate_bitwisenot(obj),
            PrefixOperator::AddrOf => unreachable!(),
        }
    }

    pub fn evaluate_bitwisenot(&mut self, value: Value) -> EvalResult<Value> {
        match value {
            Value::Int(v) => Value::from(!v).into(),
            o => EvalResult::Err(RuntimeError::UnsupportedPrefixOperation(
                PrefixOperator::BitwiseNot,
                o.r#type(self.ctx.heap),
            )),
        }
    }

    pub fn evaluate_deref_expression(&mut self, obj: Value) -> EvalResult<Value> {
        match obj {
            Value::Pointer(addr) => self.ctx.heap.read(&addr).cloned().into(),
            Value::Null => EvalResult::Value(Value::NULL),
            o => EvalResult::Err(RuntimeError::CannotBeDereferenced(o.r#type(self.ctx.heap))),
        }
    }

    pub fn evaluate_addrof_expression(&mut self, target: Expression) -> EvalResult<Value> {
        let addr = self.addr_of(target);
        match addr {
            Some(addr) => EvalResult::Value(Value::Pointer(addr)),
            None => Value::NULL.into(),
        }
    }
}
