mod allocator;
mod scope;
mod slot;
mod error;
mod emitter;
mod layout;

use ast::typed;
use bytecode::Program;

pub use error::{Result, Error};

pub fn compile(program: &typed::Program) -> Result<Program> {
    let layout = allocator::alloc(program)?;
    let bytecode = emitter::emit(program, &layout)?;
    Ok(bytecode)
}
