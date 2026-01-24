use serde::Serialize;

use crate::{Expression, ast::types::Type, expressions::Identifier};

#[derive(Debug, Clone, Serialize)]
pub struct Binding {
    pub ident: Identifier,
    pub type_annotation: Option<Type>,
    pub value: Expression,
}
