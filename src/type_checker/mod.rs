mod error;
mod expressions;
pub mod module_cache;
mod statements;

pub use error::*;
pub use module_cache::*;
use std::collections::HashMap;

use crate::{
    Program, SymbolTable,
    expressions::Identifier,
    types::{Type, TypeVarId},
};

struct FunctionContext {
    return_type: Type,
}

/// Holds module names
type ModuleTrace = Vec<Identifier>;

pub struct TypeChecker<'a> {
    symbol_table: &'a mut SymbolTable,
    module_cache: &'a mut ModuleCache,
    module_trace: &'a mut ModuleTrace,
    fn_trace: Vec<FunctionContext>,
    typevar_bindings: HashMap<TypeVarId, Type>,
    self_module: Module,
}

impl<'a> TypeChecker<'a> {
    pub fn new(
        symbol_table: &'a mut SymbolTable,
        module_cache: &'a mut ModuleCache,
        module_trace: &'a mut ModuleTrace,
    ) -> Self {
        TypeChecker {
            symbol_table,
            module_cache,
            module_trace,
            fn_trace: Vec::new(),
            typevar_bindings: HashMap::new(),
            self_module: Module {
                name: "self".into(),
                exports: HashMap::new(),
            },
        }
    }
}

impl TypeChecker<'_> {
    pub fn check_program(&mut self, program: &Program) -> Result<(), ProgramCheckError> {
        let mut errors = Vec::new();

        for stmt in &program.statements {
            match self.check_statement(stmt) {
                Ok(_) => {}
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(ProgramCheckError::new(errors))
        }
    }

    pub fn enter_function(&mut self, fn_return_ty: Type) {
        self.fn_trace.push(FunctionContext {
            return_type: fn_return_ty,
        });
    }

    pub fn leave_function(&mut self) {
        self.fn_trace.pop();
    }

    pub fn current_fn_return_type(&self) -> &Type {
        self.fn_trace
            .last()
            .map(|stack| &stack.return_type)
            .unwrap_or(&Type::Void)
    }

    pub fn unify_typevar(&mut self, t1: Type, t2: Type) -> CheckerResult<Type> {
        let t1 = self.resolve_typevar(t1);
        let t2 = self.resolve_typevar(t2);

        match (t1, t2) {
            (Type::TypeVar(id1), Type::TypeVar(id2)) if id1.id == id2.id => Ok(Type::TypeVar(id1)),

            (Type::TypeVar(id), t) => self.bind_typevar(id, t),
            (t, Type::TypeVar(id)) => self.bind_typevar(id, t),

            (Type::Array(arr_ty1), Type::Array(arr_ty2)) => {
                Ok(Type::Array(self.unify_typevar(*arr_ty1, *arr_ty2)?.into()))
            }
            (Type::HashMap { key: k1, value: v1 }, Type::HashMap { key: k2, value: v2 }) => {
                let key = self.unify_typevar(*k1, *k2)?.into();
                let value = self.unify_typevar(*v1, *v2)?.into();

                Ok(Type::HashMap { key, value })
            }
            (Type::Addr(addr_ty1), Type::Addr(addr_ty2)) => {
                Ok(Type::Addr(self.unify_typevar(*addr_ty1, *addr_ty2)?.into()))
            }

            (Type::Nullable(t1), Type::Nullable(t2)) => {
                Ok(Type::Nullable(self.unify_typevar(*t1, *t2)?.into()))
            }
            (Type::Nullable(t1), t2) => Ok(Type::Nullable(self.unify_typevar(*t1, t2)?.into())),
            (t1, Type::Nullable(t2)) => Ok(Type::Nullable(self.unify_typevar(t1, *t2)?.into())),

            (Type::AsValue(t1), Type::AsValue(t2)) => {
                Ok(Type::AsValue(self.unify_typevar(*t1, *t2)?.into()))
            }

            (t1, t2) if t1 == t2 => Ok(t1),
            (t1, t2) => Err(TypeCheckerDiagnostic::Unsatisfied(t2, t1)),
        }
    }

    pub fn bind_typevar(&mut self, id: TypeVarId, t: Type) -> CheckerResult<Type> {
        if occurs(id.clone(), &t) {
            return Err(TypeCheckerDiagnostic::InfiniteType);
        }

        self.typevar_bindings.insert(id, t.clone());
        Ok(t)
    }

    pub fn resolve_typevar(&self, ty: Type) -> Type {
        match ty {
            Type::TypeVar(id) => {
                if let Some(bound) = self.typevar_bindings.get(&id) {
                    self.resolve_typevar(bound.clone())
                } else {
                    Type::TypeVar(id)
                }
            }
            other => other,
        }
    }
}

fn occurs(id: TypeVarId, ty: &Type) -> bool {
    match ty {
        Type::TypeVar(other) => *other == id,
        Type::Array(arr_ty) => occurs(id, arr_ty),
        Type::HashMap { key, value } => occurs(id.clone(), key) || occurs(id, value),
        Type::Addr(addr_ty) => occurs(id, addr_ty),
        _ => false,
    }
}
