mod allocator;
mod scope;
mod slot;
mod error;
mod emitter;

use ast::typed;
use bytecode::Program;

pub use error::{Result, Error};

pub fn compile(program: &typed::Program) -> Result<Program> {
    let slots = allocator::alloc(program)?;

    todo!()
}
