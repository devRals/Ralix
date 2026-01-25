use crate::{EvalResult, Evaluator, Expression, Object};

mod address;
mod copy;
mod function;
mod identifier;
mod if_else;
mod infix_prefix;
mod scope;
mod r#typeof;

impl Evaluator<'_> {
    pub fn evaluate_expression(&mut self, expr: Expression) -> EvalResult<Object> {
        match expr {
            Expression::Integer(val) => Object::Int(val).into(),
            Expression::Float(val) => Object::Float(val).into(),
            Expression::String(val) => Object::String(val).into(),
            Expression::Char(val) => Object::Char(val).into(),
            Expression::Boolean(val) => match val {
                true => Object::TRUE.into(),
                false => Object::FALSE.into(),
            },
            Expression::Null => Object::NULL.into(),
            Expression::Type(ty) => Object::Type(ty).into(),
            Expression::Identifier(ident) => self.evaluate_identifier(ident),
            Expression::Infix {
                left,
                operator,
                right,
            } => self.evaluate_infix_expression(*left, operator, *right),
            Expression::Prefix { operator, right } => {
                self.evaluate_prefix_expression(operator, *right)
            }
            Expression::Copy(ident) => self.evaluate_copy_expression(ident),
            Expression::TypeOf(expr) => self.evaluate_typeof_expression(*expr),
            Expression::AddrOf(ident) => self.evaluate_addr_expression(ident),
            Expression::Scope { statements } => self.evaluate_scope_expression(statements),
            Expression::IfElse {
                consequences,
                else_consequence,
            } => self.evaluate_if_else_expression(consequences, else_consequence.map(|c| *c)),
            Expression::Function {
                parameters,
                body,
                return_type,
            } => self.evaluate_function_expression(parameters, *body, return_type),
        }
    }
}
