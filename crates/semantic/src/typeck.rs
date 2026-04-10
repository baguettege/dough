mod expr;
mod stmt;
mod item;
mod common;
mod ty;

use crate::{bindings, Error, Result};
use ast::typed::{Item, Program};
use ast::untyped;
use dough_core::Type;

struct TypeChecker<'a> {
    bindings: &'a bindings::Table,
    return_ty: Option<Type>,
}

impl<'a> TypeChecker<'a> {
    fn new(bindings: &'a bindings::Table) -> Self {
        Self { bindings, return_ty: None }
    }

    fn check(mut self, program: &untyped::Program) -> Result<Program> {
        let program = program
            .iter()
            .map(|item| self.check_item(item))
            .collect::<Result<Vec<_>>>()?;

        if has_main(&program) {
            Ok(program)
        } else {
            Err(Error::MissingMain)
        }
    }
}

fn has_main(program: &Program) -> bool {
    program
        .iter()
        .any(|item| match item {
            Item::Func(node) => {
                node.ident() == "main" &&
                    node.params().is_empty() &&
                    node.return_ty() == &Type::Unit
            },
            _ => false,
        })
}

pub(crate) fn check(
    program: &untyped::Program,
    bindings: &bindings::Table,
) -> Result<Program> {
    TypeChecker::new(bindings).check(program)
}
