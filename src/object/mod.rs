use std::{
    collections::HashMap,
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::{
    Expression,
    expressions::FunctionParameter,
    types::{FunctionParameterType, Type, TypeVarId},
};
mod environment;
mod heap;

pub use environment::*;
pub use heap::*;

pub type HashKey = u64;

pub type HashPair = (Addr, Addr);

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Int(i64),
    Char(char),
    String(Rc<String>),
    Float(f64),
    Boolean(bool),
    Type(Type),
    Address(Addr),
    Array(Vec<Addr>),
    HashMap(HashMap<HashKey, HashPair>),
    Null,

    Function(Rc<Function>),
}

macro_rules! hash {
    (  $v: expr ) => {{
        let mut hasher = DefaultHasher::new();
        $v.hash(&mut hasher);
        hasher.finish()
    }};
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
            O::Array(items) => Type::Array(
                items
                    .first()
                    .map(|i| i.r#type())
                    .unwrap_or(Type::Unknown)
                    .into(),
            ),
            Object::HashMap(hm) => {
                let (k, v) = hm
                    .values()
                    .next()
                    .map(|(k, v)| (k.r#type(), v.r#type()))
                    .unwrap_or((Type::Unknown, Type::Unknown));

                Type::HashMap {
                    key: k.into(),
                    value: v.into(),
                }
            }
            O::Type(t) => Type::AsValue(t.clone().into()),
            O::Address(t) => Type::Addr(t.r#type().into()),
            O::Function(func) => Type::Function {
                parameters: func
                    .parameters
                    .iter()
                    .map(|param| FunctionParameterType {
                        is_constant: param.is_constant,
                        ty: param.type_def.clone(),
                    })
                    .collect(),
                return_type: Box::new(func.return_type.clone()),
                generics: func.generics.clone(),
            },
        }
    }

    pub fn hash_key(&self) -> Option<HashKey> {
        Some(match self {
            Object::Boolean(v) => hash!(v),
            Object::Int(v) => hash!(v),
            Object::String(v) => hash!(v),
            Object::Char(v) => hash!(v),
            _ => return None,
        })
    }

    pub fn new_function(
        parameters: Vec<FunctionParameter>,
        return_type: Type,
        body: Expression,
        env: FunctionEnvironment,
        generics: Vec<TypeVarId>,
    ) -> Rc<Function> {
        Rc::new(Function {
            parameters,
            return_type,
            body,
            env,
            generics,
        })
    }

    pub fn is_true(&self) -> bool {
        Object::TRUE == *self
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Object::Null)
    }
}

/// Snapshot of last scope in the [`Environment`]
#[derive(Debug, PartialEq)]
pub struct FunctionEnvironment {
    pub items: EnvScope,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub parameters: Vec<FunctionParameter>,
    pub return_type: Type,
    pub body: Expression,
    pub env: FunctionEnvironment,
    pub generics: Vec<TypeVarId>,
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let return_type = &self.return_type;
        let body = &self.body;
        let parameters = &self
            .parameters
            .iter()
            .map(|param| {
                format!(
                    "{}{} {}",
                    if param.is_constant { "const " } else { "" },
                    param.type_def,
                    param.name
                )
            })
            .collect::<Vec<_>>()
            .join("\x1b[0m, ");
        let generics = if self.generics.is_empty() {
            "".to_string()
        } else {
            format!(
                "[{}]",
                self.generics
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        write!(f, "fn{generics}({parameters}) -> {return_type}: {body}")
    }
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
            O::Function(func) => func.to_string(),
            O::Array(items) => format!(
                "[{}]",
                items
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            O::HashMap(hm) => format!("#{{ {} }}", {
                hm.values()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            }),
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
impl_from!(String, String);

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        match value {
            true => Object::TRUE,
            false => Object::FALSE,
        }
    }
}
