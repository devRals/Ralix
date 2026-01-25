use std::fmt::Display;

use crate::{Expression, Literal, expressions::FunctionParameter, types::Type};
mod environment;

pub use environment::*;

#[derive(Debug, Clone)]
pub enum Object {
    Int(i64),
    Char(char),
    String(Literal),
    Float(f64),
    Boolean(bool),
    Type(Type),
    Address(*const Object),
    Null,

    Function {
        parameters: Vec<FunctionParameter>,
        return_type: Type,
        body: Expression,
        env: Environment,
    },
}

impl Object {
    pub const TRUE: Self = Object::Boolean(true);
    pub const FALSE: Self = Object::Boolean(false);
    pub const NULL: Self = Object::Null;

    pub fn r#type(&self) -> Type {
        use Object as O;
        match self {
            O::Boolean(_) => Type::Bool,
            O::Char(_) => Type::Char,
            O::Int(_) => Type::Int,
            O::Float(_) => Type::Float,
            O::Null => Type::Null,
            O::String(_) => Type::String,
            O::Type(_) => Type::AsValue,
            O::Address(t) => Type::Addr(Box::new(unsafe { (**t).clone().r#type() })),
            O::Function {
                parameters,
                return_type,
                ..
            } => Type::Function {
                parameters: parameters.iter().map(|(t, _)| t.clone()).collect(),
                return_type: Box::new(return_type.clone()),
            },
        }
    }

    pub fn is_true(&self) -> bool {
        Object::TRUE == *self
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Object as O;

        let clear = "\x1b[0m";

        f.write_str(&match self {
            O::Boolean(val) => format!("\x1b[36m{val}{clear}"),
            O::Int(val) => format!("\x1b[93m{val}{clear}"),
            O::Char(val) => format!("\x1b[94m{val}{clear}"),
            O::Float(val) => format!("\x1b[93m{val}{clear}"),
            O::String(val) => format!("\x1b[32m{val}{clear}"),
            O::Null => format!("\x1b[90mnull{clear}"),
            O::Type(ty) => format!("\x1b[92m{ty}{clear}"),
            O::Address(addr) => format!("\x1b[90m<{addr:?}>{clear}"),
            O::Function {
                parameters,
                return_type,
                body,
                ..
            } => {
                format!(
                    "\x1b[93mfn{clear}({}{clear}) \x1b[92m{return_type}{clear}: {body}",
                    parameters
                        .iter()
                        .map(|(p_ty, p_name)| format!("\x1b[92m{p_ty} \x1b[0m{p_name}"))
                        .collect::<Vec<_>>()
                        .join("\x1b[0m, ")
                )
            }
        })
    }
}

macro_rules! impl_from {
    ( $ty: ty, $wrapper: ident ) => {
        impl From<$ty> for Object {
            fn from(value: $ty) -> Self {
                Object::$wrapper(value.into())
            }
        }
    };
}

impl_from!(i64, Int);
impl_from!(f64, Float);
impl_from!(char, Char);
impl_from!(&str, String);
impl_from!(String, String);

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        match value {
            true => Object::TRUE,
            false => Object::FALSE,
        }
    }
}
