mod error;
mod value;
mod vm;

pub use error::{Error, Result};
use bytecode::Program;

pub fn run(program: &Program) -> Result<()> {
    vm::run(program)
}
