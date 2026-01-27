use std::fmt::Display;

use crate::{
    Expression, Node, NodeV, Program, Statement,
    expressions::{InfixOperator, PrefixOperator},
};

impl Node for Expression {
    fn downcast(self) -> NodeV {
        NodeV::Expression(self)
    }
}
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
            } => format!(
                "fn({}) {return_type}: {body}",
                parameters
                    .iter()
                    .map(|(p_ty, p_name)| format!("{p_ty} {p_name}"))
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
        })
    }
}

impl Node for Statement {
    fn downcast(self) -> super::NodeV {
        NodeV::Statement(self)
    }
}
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Binding {
                ident,
                type_annotation,
                value,
            } => format!(
                "{} {} = {};",
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

impl Node for Program {
    fn downcast(self) -> NodeV {
        NodeV::Program(self)
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
