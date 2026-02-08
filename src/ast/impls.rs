use std::fmt::Display;

use crate::{
    Expression, Program, Statement,
    expressions::{HashMapItem, InfixOperator, PrefixOperator},
    types::{FunctionParameterType, Type, TypeVarId},
};

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression as E;
        f.write_str(&match self {
            E::Identifier(ident) => ident.to_string(),
            E::Integer(val) => val.to_string(),
            E::Float(val) => val.to_string(),
            E::Boolean(val) => val.to_string(),
            E::String(val) => format!("\"{val}\""),
            E::Char(val) => format!("'{val}'"),
            E::Null => "null".to_string(),
            E::Copy(ident) => format!("copy {ident}"),
            E::TypeOf(expr) => format!("typeof {expr}"),
            E::Type(ty) => format!("{ty}"),
            E::AddrOf(ident) => format!("&{ident}"),
            E::Try(expr) => format!("{expr}?"),
            E::Infix {
                left,
                operator,
                right,
            } => format!("({left} {operator} {right})"),
            E::Prefix { operator, right } => format!("{operator}{right}"),
            E::Scope { statements } => format!(
                "{{\n{}}}",
                statements
                    .iter()
                    .map(|stmt| format!("{stmt}\n"))
                    .collect::<Vec<_>>()
                    .join(";")
            ),

            E::IfElse {
                consequences,
                else_consequence,
            } => {
                return {
                    let mut first = true;
                    for (cond, cons) in consequences {
                        write!(
                            f,
                            "{}if {cond}: {cons}",
                            if first {
                                first = false;
                                ""
                            } else {
                                " else "
                            }
                        )?;
                    }

                    if let Some(else_con) = else_consequence {
                        write!(f, " else: {else_con}",)?;
                    }

                    Ok(())
                };
            }

            E::Function {
                parameters,
                return_type,
                body,
                generics,
            } => format!(
                "fn{}({}) {return_type}: {body}",
                if generics.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        "[{}]",
                        generics
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                },
                parameters
                    .iter()
                    .map(|param| format!(
                        "{}{} {}",
                        if param.is_constant { "const " } else { "" },
                        param.type_def,
                        param.name
                    ))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),

            E::Call {
                function,
                arguments,
            } => format!(
                "{function}({})",
                arguments
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),

            E::Array { items } => format!(
                "[{}]",
                items
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),

            E::HashMap { items } => format!(
                "#{{ {} }}",
                items
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),

            E::Index { left, index } => format!("{left}[{index}]"),
        })
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Binding {
                ident,
                type_annotation,
                value,
                is_constant,
            } => format!(
                "{}{} {} = {};",
                if *is_constant { "const " } else { "" },
                if let Some(ty_a) = type_annotation {
                    ty_a.to_string()
                } else {
                    "let".to_string()
                },
                ident,
                value
            ),
            Self::Expression(expr) => expr.to_string(),
            Self::Return(expr) => format!(
                "return{}",
                match expr {
                    Some(e) => format!(" {e}"),
                    None => ";".to_string(),
                }
            ),
            Self::Assign { left, value } => format!("{left} = {value}"),
        })
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.statements {
            f.write_str(&s.to_string())?;
        }

        Ok(())
    }
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            InfixOperator::Add => "+",
            InfixOperator::Subtract => "-",
            InfixOperator::Multiply => "*",
            InfixOperator::Divide => "/",
            InfixOperator::Remainder => "%",
            InfixOperator::Equals => "==",
            InfixOperator::NotEquals => "!=",
            InfixOperator::Or => "||",
            InfixOperator::And => "&&",
            InfixOperator::Less => "<",
            InfixOperator::LessEq => "<=",
            InfixOperator::Greater => ">",
            InfixOperator::GreatEq => ">=",
        })
    }
}

impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PrefixOperator::Not => "!",
            PrefixOperator::Neg => "-",
            PrefixOperator::Deref => "*",
        })
    }
}

impl Display for HashMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

impl Display for TypeVarId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl Display for FunctionParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let constant = if self.is_constant { "const " } else { "" };
        write!(f, "{}{}", constant, self.ty)
    }
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
                generics,
            } => format!(
                "fn{}({}) -> {return_type}",
                if generics.is_empty() {
                    "".to_string()
                } else {
                    format!(
                        "[{}]",
                        generics
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                },
                paramters
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            T::TypeVar(t) => t.to_string(),
        })
    }
}

impl std::ops::Index<usize> for Program {
    type Output = Statement;
    fn index(&self, index: usize) -> &Self::Output {
        &self.statements[index]
    }
}
