mod item;
mod stmt;
mod expr;
mod resolve;
mod common;

use crate::symbol::Symbol;
use crate::{Error, Result};
use crate::{scope, symbol, TypedProgram};
use ast::types::Ident;
use ast::untyped;
use dough_core::Type;

#[derive(Default)]
pub(crate) struct Analyzer {
    table: symbol::Table,
    stack: scope::Stack,
    return_ty: Option<Type>,
}

impl Analyzer {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn analyze(mut self, program: &untyped::Program) -> Result<TypedProgram> {
        for item in program {
            self.register(item)?;
        }

        self.validate_main()?;
        
        let typed = program
            .iter()
            .map(|item| self.analyze_item(item))
            .collect::<Result<Vec<_>>>()?;

        Ok(TypedProgram::new(typed, self.table))
    }

    fn validate_main(&self) -> Result<()> {
        match self.table.lookup(&Ident::new("main")) {
            Some(Symbol::Fn { params, return_ty }) => {
                (params.is_empty() && *return_ty == Type::Unit)
                    .then(|| ())
                    .ok_or(Error::InvalidMain)
            },
            _ => Err(Error::MissingMain),
        }
    }

    fn register(&mut self, item: &untyped::Item) -> Result<()> {
        match item {
            untyped::Item::Fn { ident, params, return_ty, .. } => {
                let params = params
                    .iter()
                    .map(|param| resolve::ty(param.ty()))
                    .collect::<Result<Vec<_>>>()?;
                let return_ty = return_ty
                    .as_ref()
                    .map(|ty| resolve::ty(ty))
                    .transpose()?
                    .unwrap_or(Type::Unit);

                let symbol = Symbol::Fn { params, return_ty };
                self.table.insert(ident.clone(), symbol);
            },
            untyped::Item::Static { ident, ty, .. } => {
                let ty = resolve::ty(ty)?;
                let symbol = Symbol::Global(ty);
                self.table.insert(ident.clone(), symbol);
            },
        }

        Ok(())
    }
}
