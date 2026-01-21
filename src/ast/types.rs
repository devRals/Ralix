use std::fmt::Display;

use crate::Token;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Type {
    Bool,
    Char,
    Int,
    Float,
    String,
    Null,
    Addr(Box<Type>),
    TypeAsValue(Box<Type>),
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
            T::TypeAsValue(ty) => format!("{ty} as value"),
            T::Addr(ty) => format!("{ty}*"),
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
            (Type::TypeAsValue(t1), t2) => t1.is(t2),
            (t1, Type::TypeAsValue(t2)) => t2.is(t1),
            (t1, t2) => t1.is(t2),
        }
    }

    pub fn is(&self, other: &Type) -> bool {
        self == other
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
