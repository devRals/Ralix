use std::{error::Error, fmt::Display};

use crate::{Literal, Token};

pub type ParserResult<N> = Result<N, ParserError>;

#[derive(Debug, Clone)]
pub enum ParserError {
    SyntaxError { expected: Token, got: Token },
    IsNotIdentifier(Token),
    Undefined(Literal),
    TypeMistake(Literal),
    ExpressionMistake(Token),
    IntegerParse(Literal),
    FloatParse(Literal),
    UnknownInfixOp(Literal),
    UnknownPrefixOp(Literal),
}

#[derive(Debug)]
pub struct ProgramParseError {
    all: Vec<ParserError>,
}

impl Error for ParserError {}
impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParserError as E;
        f.write_str(&match self {
            E::SyntaxError { expected, got } => {
                format!("Syntax Error: Expected `{expected}`, but found `{got}` instead")
            }
            E::IsNotIdentifier(got) => {
                format!("Syntax Error: Expected `<identifier>`, but found `{got}` instead")
            }
            E::Undefined(lit) => format!("`{lit}` is not found in the current scope"),
            E::TypeMistake(lit) => format!("`{lit}` cannot be use as a type definition"),
            E::ExpressionMistake(tok) => {
                format!("Expected an expression but `{tok}` is not avaliable to create expressions")
            }
            E::IntegerParse(int_lit) => format!("`{int_lit}` can't parse into a integer value"),
            E::FloatParse(float_parse) => {
                format!("`{float_parse}` can't parse into a floating point number value")
            }
            E::UnknownInfixOp(op) => {
                format!("`{op}` cannot be used as an opeartor in infix expressions")
            }
            E::UnknownPrefixOp(op) => {
                format!("`{op}` cannot be used as an opeartor in prefix expressions")
            }
        })
    }
}

impl Error for ProgramParseError {}
impl Display for ProgramParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.all {
            writeln!(f, "{e}")?;
        }

        Ok(())
    }
}

impl ProgramParseError {
    pub fn new<U: IntoIterator>(errors: U) -> Self
    where
        Vec<ParserError>: FromIterator<U::Item>,
    {
        Self {
            all: errors.into_iter().collect(),
        }
    }
}
