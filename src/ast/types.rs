use std::fmt::Display;

use serde::Serialize;

use crate::Token;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize)]
pub struct FunctionParameterType {
    pub ty: Type,
    pub is_constant: bool,
}

impl Display for FunctionParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let constant = if self.is_constant { "const " } else { "" };
        write!(f, "{}{}", constant, self.ty)
    }
}

pub type TypeId = usize;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize)]
pub enum Type {
    Bool,
    Char,
    Int,
    Float,
    String,
    Null,
    Void,
    Unknown,
    Never,
    AsValue(Box<Type>),
    Nullable(Box<Type>),
    Array(Box<Type>),
    TypeArg(TypeId),
    HashMap {
        key: Box<Type>,
        value: Box<Type>,
    },
    Addr(Box<Type>),
    Function {
        parameters: Vec<FunctionParameterType>,
        return_type: Box<Type>,
    },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Type as T;
        f.write_str(&match self {
            T::Bool => "bool".to_string(),
            T::Char => "char".to_string(),
            T::Int => "int".to_string(),
            T::Float => "float".to_string(),
            T::String => "str".to_string(),
            T::Null => "null".to_string(),
            T::Void => "void".to_string(),
            T::Never => "never".to_string(),
            T::Unknown => "unknown".to_string(),
            T::AsValue(ty) => format!("type[{ty}]"),
            T::Nullable(ty) => format!("{ty}?"),
            T::Addr(ty) => format!("{ty}*"),
            T::Array(ty) => format!("arr[{ty}]"),
            T::HashMap { key, value } => format!("map[{key}, {value}]"),
            T::Function {
                parameters: paramters,
                return_type,
            } => format!(
                "fn({}) -> {return_type}",
                paramters
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            T::TypeArg(t) => format!("T: id = {t}"),
        })
    }
}

impl Type {
    pub const fn from_token(token: &Token) -> Option<Type> {
        Some(match token {
            Token::TyInt => Type::Int,
            Token::TyChar => Type::Char,
            Token::TyString => Type::String,
            Token::TyFloat => Type::Float,
            Token::TyBool => Type::Bool,
            Token::Null => Type::Null,
            _ => return None,
        })
    }

    pub fn is_hashable(&self) -> bool {
        use Type::*;
        matches!(self, Int | Bool | Char | String)
    }

    pub fn satisfies(&self, other: &Type) -> bool {
        match (self, other) {
            (t1, Type::Nullable(t2)) => {
                t1.is(&Type::Null)
                    || t1.is(&Type::Void)
                    || matches!(t1, Type::Nullable(_))
                    || t1.satisfies(t2)
            }
            (Type::Nullable(_), Type::Null) => true,
            (Type::Array(arr_ty1), Type::Array(arr_ty2)) => arr_ty1.satisfies(arr_ty2),
            (
                Type::HashMap {
                    key: key1,
                    value: value1,
                },
                Type::HashMap {
                    key: key2,
                    value: value2,
                },
            ) => key1.satisfies(key2) && value1.satisfies(value2),
            (_, Type::Void) => true,
            (t1, t2) => t1.is(t2),
        }
    }

    pub fn is(&self, other: &Type) -> bool {
        self == other
    }

    pub const fn is_nullish(&self) -> bool {
        use Type::*;
        matches!(self, Null | Void | Unknown | Never)
    }

    pub fn unwrap_nullable(self) -> Type {
        match self {
            Type::Nullable(t) => *t,
            t => t,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Type;

    #[test]
    fn test_type_satisfaction() {
        let tests = [
            ((Type::Bool, Type::Bool), true),
            ((Type::Int, Type::Int), true),
            ((Type::Float, Type::Int), false),
            ((Type::String, Type::Char), false),
        ];

        for ((t1, t2), expected) in tests {
            assert_eq!(t1.satisfies(&t2), expected)
        }
    }
}
