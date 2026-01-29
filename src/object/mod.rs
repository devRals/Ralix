use std::{fmt::Display, rc::Rc};

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

    Function(Rc<Function>),
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
            O::Function(func) => Type::Function {
                parameters: func.parameters.iter().map(|(t, _)| t.clone()).collect(),
                return_type: Box::new(func.return_type.clone()),
            },
        }
    }

    pub fn new_function(
        parameters: Vec<FunctionParameter>,
        return_type: Type,
        body: Expression,
        env: FunctionEnvironment,
    ) -> Rc<Function> {
        Rc::new(Function {
            parameters,
            return_type,
            body,
            env,
        })
    }

    pub fn is_true(&self) -> bool {
        Object::TRUE == *self
    }
}

/// Snapshot of last scope in the [`Environment`]
#[derive(Debug)]
pub struct FunctionEnvironment {
    pub items: EnvScope,
}

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<FunctionParameter>,
    return_type: Type,
    pub body: Expression,
    pub env: FunctionEnvironment,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Object as O;

        f.write_str(&match self {
            O::Boolean(val) => val.to_string(),
            O::Int(val) => val.to_string(),
            O::Char(val) => val.to_string(),
            O::Float(val) => val.to_string(),
            O::String(val) => val.to_string(),
            O::Null => "null".to_string(),
            O::Type(ty) => ty.to_string(),
            O::Address(addr) => format!("<{addr:?}>"),
            O::Function(func) => {
                let return_type = &func.return_type;
                let body = &func.body;
                let parameters = &func.parameters;
                format!(
                    "fn({}) -> {return_type}: {body}",
                    parameters
                        .iter()
                        .map(|(p_ty, p_name)| format!("{p_ty} {p_name}"))
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
