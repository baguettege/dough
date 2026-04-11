use bytecode::{Argc, Idx, Off};
use crate::{Error, Result};
use crate::vm::frame::Frame;
use crate::vm::Vm;

impl Vm<'_> {
    pub(super) fn op_jmp(&mut self, off: Off) -> Result<()> {
        self.frames.current_mut()?.jump(off)?;
        Ok(())
    }

    pub(super) fn op_jf(&mut self, off: Off) -> Result<()> {
        let condition: bool = pop!(self);
        if !condition {
            self.frames.current_mut()?.jump(off)?;
        }
        Ok(())
    }

    pub(super) fn op_call(&mut self, idx: Idx, argc: Argc) -> Result<()> {
        let base = self.operands.len() - argc as usize;
        let chunk = self.program
            .funcs()
            .get(idx as usize)
            .ok_or(Error::IndexOutOfBounds)?;

        let local_count = chunk.local_count();
        self.frames.push(Frame::new(base, chunk));

        self.operands.reserve(local_count);
        Ok(())
    }

    pub(super) fn op_ret(&mut self) -> Result<()> {
        let value = self.operands.pop()?;

        let base = self.frames.pop()?.base();
        self.operands.truncate(base);

        push!(self, value);
        Ok(())
    }
}
