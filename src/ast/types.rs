use serde::Serialize;

use crate::{Token, expressions::Identifier};

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize)]
pub struct FunctionParameterType {
    pub ty: Type,
    pub is_constant: bool,
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize)]
pub struct TypeVarId {
    pub name: Identifier,
    pub id: u64,
}

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
    TypeVar(TypeVarId),
    HashMap {
        key: Box<Type>,
        value: Box<Type>,
    },
    Addr(Box<Type>),
    Function {
        parameters: Vec<FunctionParameterType>,
        return_type: Box<Type>,
        generics: Vec<TypeVarId>,
    },
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

    pub const fn includes_unknown(&self) -> bool {
        match self {
            Type::Unknown => true,
            Type::Array(arr_ty) => arr_ty.includes_unknown(),
            Type::HashMap { value, .. } => value.includes_unknown(),
            Type::AsValue(as_value_ty) => as_value_ty.includes_unknown(),
            Type::Addr(addr_ty) => addr_ty.includes_unknown(),
            _ => false,
        }
    }

    pub fn map<U, F: Fn(Type) -> Type>(self, f: F) -> Type {
        match self {
            Type::AsValue(ty) => Type::AsValue(f(*ty).into()),
            Type::Array(ty) => Type::Array(f(*ty).into()),
            Type::Nullable(ty) => Type::Nullable(f(*ty).into()),
            Type::Addr(ty) => Type::Addr(f(*ty).into()),
            Type::HashMap { key, value } => Type::HashMap {
                key: f(*key).into(),
                value: f(*value).into(),
            },
            x => f(x),
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
