use crate::{
    CheckerResult, Expression, TypeChecker, TypeCheckerDiagnostic,
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
                O::Add
                | O::Subtract
                | O::Multiply
                | O::Divide
                | O::Remainder
                | O::BitwiseAnd
                | O::BitwiseOr
                | O::BitwiseXOr
                | O::BitShiftLeft
                | O::BitShiftRight,
                Type::Int,
            ) => Ok(Type::Int),

            (Type::String, O::Add, Type::String) => Ok(Type::String),

            (t1, O::Equals | O::NotEquals | O::Less | O::LessEq | O::Greater | O::GreatEq, t2) => {
                if !t1.is(&t2) {
                    match (t1, t2) {
                        (Type::Nullable(_), Type::Null) | (Type::Null, Type::Nullable(_)) => {
                            Ok(Type::Bool)
                        }
                        (t1, t2) => Err(TypeCheckerDiagnostic::InfixTypeMismatched(
                            t1, *operator, t2,
                        )),
                    }
                } else {
                    Ok(Type::Bool)
                }
            }

            (left_ty, operator, right_ty) => Err(TypeCheckerDiagnostic::InfixTypeMismatched(
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
            (O::Neg | O::BitwiseNot, Type::Int) => Ok(Type::Int),
            (O::AddrOf, ty) => match right {
                // You can't get address of a non-existing binding
                // because of that address of returns a nullable
                // For example from a hashmap:
                // `map[str,str] x = #{ "a": "b" }; &x["c"];`
                // there is not a value that `x` has holding with key "c"
                Expression::Index { .. } => Ok(Type::Nullable(Type::Addr(ty.into()).into())),
                _ => Ok(Type::Addr(ty.into())),
            },
            (O::Deref, target) => match target {
                Type::Addr(t) => Ok(*t),
                Type::Nullable(t) => {
                    if let Type::Addr(t) = *t {
                        Ok(Type::Nullable(t))
                    } else {
                        Err(TypeCheckerDiagnostic::PrefixTypeMismatched(O::Deref, *t))
                    }
                }
                t => Err(TypeCheckerDiagnostic::PrefixTypeMismatched(O::Deref, t)),
            },
            (op, ty) => Err(TypeCheckerDiagnostic::PrefixTypeMismatched(*op, ty)),
        }
    }
}
