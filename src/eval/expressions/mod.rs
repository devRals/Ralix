use crate::{EvalResult, Evaluator, Expression, Object};

mod copy;
mod identifier;
mod infix_prefix;

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
        }
    }
}
