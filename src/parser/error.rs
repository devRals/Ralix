use std::{error::Error, fmt::Display, io};

use crate::{Expression, Literal, Token, types::Type};

pub type ParserResult<N> = Result<N, ParserDiagnostic>;

#[derive(Debug)]
pub enum ParserDiagnostic {
    SyntaxError { expected: Token, got: Token },
    IsNotIdentifier(Token),
    Undefined(Literal),
    TypeMistake(Literal),
    ExpressionMistake(Token),
    IntegerParse(Literal),
    FloatParse(Literal),
    UnknownInfixOp(Literal),
    UnknownPrefixOp(Literal),
    CannotAssignTo(Expression),
    CannotBindUsing(Type),
    IsNotHashable(Type),
    FileModuleError(io::Error),
    UnacceptableConst,
}

#[derive(Debug)]
pub struct ProgramParseError {
    all: Vec<ParserDiagnostic>,
}

impl Error for ParserDiagnostic {}
impl Display for ParserDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ParserDiagnostic as E;
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
            E::CannotAssignTo(expr) => format!("Cannot assign a value to a `{expr}` expression"),
            E::UnacceptableConst => {
                "`const` keyword can only be used in binding statements".to_string()
            }
            E::CannotBindUsing(ty) => format!("Type `{ty}` cannot be used in binding statements"),
            E::IsNotHashable(ty) => format!("Type `{ty}` cannot be hashed"),
            E::FileModuleError(fme) => fme.to_string(),
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
        Vec<ParserDiagnostic>: FromIterator<U::Item>,
    {
        Self {
            all: errors.into_iter().collect(),
        }
    }
}
