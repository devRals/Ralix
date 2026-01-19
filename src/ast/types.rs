use std::fmt::Display;

use crate::Token;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Bool,
    Char,
    Int,
    Float,
    String,
    Null,
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
        // It'll grow as the type amount increases
        #[allow(clippy::match_single_binding)]
        match (self, other) {
            (t1, t2) => t1 == t2,
            // ...
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
