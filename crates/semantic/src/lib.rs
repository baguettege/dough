mod error;
mod resolver;
mod typeck;
mod symbol;
mod bindings;

use ast::typed::Program;
use ast::untyped;

pub use error::{Error, Result};

pub fn analyze(program: &untyped::Program) -> Result<Program> {
    let symbols = resolver::resolve(program)?;
    let typed = typeck::check(program, &symbols)?;
    Ok(typed)
}
