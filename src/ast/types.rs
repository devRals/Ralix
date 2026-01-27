use std::fmt::Display;

use serde::Serialize;

use crate::Token;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Serialize)]
pub enum Type {
    Bool,
    Char,
    Int,
    Float,
    String,
    Null,
    Void,
    Never,
    AsValue,
    Nullable(Box<Type>),
    Addr(Box<Type>),
    Function {
        parameters: Vec<Type>,
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
            T::AsValue => "type".to_string(),
            T::Nullable(ty) => format!("{ty}?"),
            T::Addr(ty) => format!("{ty}*"),
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
        })
    }
}

impl Type {
    pub fn from_token(token: &Token) -> Option<Type> {
        Some(match token {
            Token::TyInt => Type::Int,
            Token::TyChar => Type::Char,
            Token::TyString => Type::String,
            Token::TyFloat => Type::Float,
            Token::Bool => Type::Bool,
            _ => return None,
        })
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
            (t1, t2) => t1.is(t2),
        }
    }

    pub fn is(&self, other: &Type) -> bool {
        self == other
    }

    pub fn unwrap_nullable(self) -> Type {
        match self {
            Type::Nullable(t) => *t,
            t => t.unwrap_nullable(),
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
