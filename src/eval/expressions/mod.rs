use crate::{EvalResult, Evaluator, Expression, Value};

mod array;
mod function;
mod hashmap;
mod identifier;
mod if_else;
mod index;
mod infix_prefix;
mod scope;
mod r#try;
mod type_casting;
mod r#typeof;

impl Evaluator<'_> {
    pub fn evaluate_expression(&mut self, expr: Expression) -> EvalResult<Value> {
        match expr {
            Expression::Integer(val) => Value::Int(val).into(),
            Expression::Float(val) => Value::Float(val).into(),
            Expression::String(val) => Value::String(val.to_string().into()).into(),
            Expression::Char(val) => Value::Char(val).into(),
            Expression::Array { items } => self.evaluate_array_literal(items),
            Expression::HashMap { items } => self.evaluate_hashmap_literal(items),
            Expression::Boolean(val) => match val {
                true => Value::TRUE.into(),
                false => Value::FALSE.into(),
            },
            Expression::Index { left, index } => self.evaluate_index_expression(*left, *index),
            Expression::Null => Value::NULL.into(),
            Expression::Type(ty) => Value::Type(ty).into(),
            Expression::Try(expr) => self.evaluate_try_expression(*expr),
            Expression::Identifier(ident) => self.evaluate_identifier(ident),
            Expression::Infix {
                left,
                operator,
                right,
            } => self.evaluate_infix_expression(*left, operator, *right),
            Expression::Prefix { operator, right } => {
                self.evaluate_prefix_expression(operator, *right)
            }
            Expression::TypeOf(expr) => self.evaluate_typeof_expression(*expr),
            Expression::Scope { statements } => self.evaluate_scope_expression(statements),
            Expression::IfElse {
                consequences,
                else_consequence,
            } => self.evaluate_if_else_expression(consequences, else_consequence.map(|c| *c)),
            Expression::Function {
                parameters,
                body,
                return_type,
                generics,
            } => self.evaluate_function_expression(parameters, *body, return_type, generics),

            Expression::Call {
                function,
                arguments,
            } => self.evaluate_call_expression(*function, arguments),
        }
    }
}
