use crate::{CheckerError, CheckerResult, Expression, TypeChecker, types::Type};

impl TypeChecker<'_> {
    pub fn check_index_expression(
        &mut self,
        left: &Expression,
        index: &Expression,
    ) -> CheckerResult<Type> {
        let left_ty = self.check_expression(left)?;
        let index_ty = self.check_expression(index)?;

        match (left_ty, index_ty) {
            (Type::Array(arr_ty), Type::Int) => Ok(Type::Nullable(arr_ty)),
            (Type::HashMap { key, value }, index_ty) => match index_ty.is(&key) {
                true => Ok(Type::Nullable(value)),
                false => Err(CheckerError::CannotbeIndexedBy(
                    Type::HashMap { key, value },
                    index_ty,
                )),
            },
            (t1, t2) => Err(CheckerError::CannotbeIndexedBy(t1, t2)),
        }
    }
}
