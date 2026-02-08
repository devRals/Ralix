use crate::{CheckerResult, Expression, TypeChecker, types::Type};

mod inferers;

impl TypeChecker<'_> {
    pub fn check_expression(&mut self, expr: &Expression) -> CheckerResult<Type> {
        use Expression as E;

        match expr {
            E::String(_) => Ok(Type::String),
            E::Char(_) => Ok(Type::Char),
            E::Float(_) => Ok(Type::Float),
            E::Integer(_) => Ok(Type::Int),
            E::Boolean(_) => Ok(Type::Bool),
            E::Type(ty) => Ok(Type::AsValue(Box::new(ty.clone()))),
            E::Null => Ok(Type::Null),
            E::Identifier(ident) => self.infer_identifier(ident),
            E::Copy(ident) => self.infer_copy_expression(ident),
            E::TypeOf(expr) => self.check_typeof_expression(expr),
            E::AddrOf(ident) => self.infer_addrof_expression(ident),
            E::Scope { statements } => self.infer_scope_expression(statements),
            E::Infix {
                left,
                operator,
                right,
            } => self.check_infix_expression(left, operator, right),
            E::Prefix { operator, right } => self.check_prefix_expression(operator, right),
            E::IfElse {
                consequences,
                else_consequence,
            } => self.infer_if_else_expression(consequences, else_consequence.as_deref()),
            E::Function {
                return_type,
                body,
                parameters,
                generics,
            } => self.check_function_expression(parameters, body, return_type, generics),
            E::Call {
                function,
                arguments,
            } => self.check_call_expression(function, arguments),
            E::Array { items } => self.check_array_literal(items),
            E::HashMap { items } => self.check_hashmap_literal(items),
            E::Index { left, index } => self.check_index_expression(left, index),
            E::Try(expr) => self.check_try_expression(expr),
        }
    }
}
