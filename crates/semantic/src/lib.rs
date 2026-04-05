mod error;
mod scope;
mod analyzer;

pub mod symbol;

use ast::typed::Program;
use ast::untyped;

pub use error::{Error, Result};

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

pub fn analyze(program: &untyped::Program) -> Result<TypedProgram> {
    analyzer::Analyzer::new().analyze(program)
}
