use std::rc::Rc;

pub type Identifier = Rc<str>;

#[derive(Debug, Clone, Copy)]
pub enum InfixOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equals,
    NotEquals,
    Or,
    And,
}

#[derive(Debug, Clone, Copy)]
pub enum PrefixOperator {
    Not,
    Neg,
}
