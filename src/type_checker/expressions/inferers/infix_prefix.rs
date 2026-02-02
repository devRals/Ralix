use crate::{
    CheckerError, CheckerResult, Expression, TypeChecker,
    expressions::{InfixOperator, PrefixOperator},
    types::Type,
};

impl TypeChecker<'_> {
    pub fn check_infix_expression(
        &mut self,
        left: &Expression,
        operator: &InfixOperator,
        right: &Expression,
    ) -> CheckerResult<Type> {
        let left_ty = self.check_expression(left)?;
        let right_ty = self.check_expression(right)?;

        use InfixOperator as O;
        match (left_ty, operator, right_ty) {
            (Type::Bool, O::Or | O::And, Type::Bool) => Ok(Type::Bool),
            (
                Type::Float,
                O::Add | O::Subtract | O::Multiply | O::Divide | O::Remainder,
                Type::Float,
            ) => Ok(Type::Float),

            (
                Type::Int,
                O::Add | O::Subtract | O::Multiply | O::Divide | O::Remainder,
                Type::Int,
            ) => Ok(Type::Int),

            (Type::String, O::Add, Type::String) => Ok(Type::String),

            (t1, O::Equals | O::NotEquals | O::Less | O::LessEq | O::Greater | O::GreatEq, t2) => {
                if !t1.is(&t2) {
                    Err(CheckerError::InfixTypeMismatched(t1, *operator, t2))
                } else {
                    Ok(Type::Bool)
                }
            }

            (left_ty, operator, right_ty) => Err(CheckerError::InfixTypeMismatched(
                left_ty, *operator, right_ty,
            )),
        }
    }

    pub fn check_prefix_expression(
        &mut self,
        operator: &PrefixOperator,
        right: &Expression,
    ) -> CheckerResult<Type> {
        let right_ty = self.check_expression(right)?;

        use PrefixOperator as O;
        match (operator, right_ty) {
            (O::Not, Type::Bool) => Ok(Type::Bool),
            (O::Neg, Type::Float) => Ok(Type::Float),
            (O::Neg, Type::Int) => Ok(Type::Int),
            (O::Deref, Type::Addr(t)) => Ok(*t),
            (op, ty) => Err(CheckerError::PrefixTypeMismatched(*op, ty)),
        }
    }
}
