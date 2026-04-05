//! Performs semantic analysis on an untyped AST, producing a typed
//! AST and a symbol table.
//! 
//! Analysis runs in 2 passes:
//! 1. Collect all top-level declarations into the symbol table to allow
//!    for forward declarations.
//! 2. Walk function bodies to resolve and check types.

mod error;
mod scope;
mod analyzer;

pub mod symbol;

use ast::typed::Program;
use ast::untyped;

pub use error::{Error, Result};

/// The output of semantic analysis - a fully typed AST with the
/// top-level symbol table.
/// 
/// A [`Symbol::Fn`] is guaranteed to exist under the ident `main`
/// in the symbol table, with no parameters, and a return type of [`Type::Unit`].
#[derive(Debug)]
pub struct TypedProgram {
    program: Program,
    table: symbol::Table,
}

impl TypedProgram {
    fn new(program: Program, table: symbol::Table) -> Self {
        Self { program, table }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn table(&self) -> &symbol::Table {
        &self.table
    }
}

/// Analyzes `program`, returning a [`TypedProgram`].
pub fn analyze(program: &untyped::Program) -> Result<TypedProgram> {
    analyzer::Analyzer::new().analyze(program)
}
