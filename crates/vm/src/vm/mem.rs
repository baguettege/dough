use bytecode::{Constant, Idx};
use crate::{Error, Result};
use crate::value::{Str, Value};
use crate::vm::Vm;

impl Vm<'_> {
    pub(super) fn op_push(&mut self, idx: Idx) -> Result<()> {
        let constant = self.frames.current()?.constant(idx)?;

        let value = match constant {
            Constant::Int(v) => Value::Int(*v),
            Constant::Float(v) => Value::Float(*v),
            Constant::Bool(v) => Value::Bool(*v),
            Constant::Str(v) => {
                let handle = self.heap.alloc(Str::new(v));
                Value::Str(handle)
            },
        };

        push!(self, value);
        Ok(())
    }

    pub(super) fn op_pushu(&mut self) -> Result<()> {
        push!(self, Value::Unit);
        Ok(())
    }

    pub(super) fn op_pop(&mut self) -> Result<()> {
        self.operands.pop()?;
        Ok(())
    }

    pub(super) fn op_ldl(&mut self, idx: Idx) -> Result<()> {
        let base = self.frames.current()?.base();
        let idx = base
            .checked_add(idx as usize)
            .ok_or(Error::IndexOutOfBounds)?;

        push!(self, self.operands.get(idx)?);
        Ok(())
    }

    pub(super) fn op_stl(&mut self, idx: Idx) -> Result<()> {
        let base = self.frames.current()?.base();
        let idx = base
            .checked_add(idx as usize)
            .ok_or(Error::IndexOutOfBounds)?;

        let value = self.operands.pop()?;
        self.operands.set(idx, value)?;
        Ok(())
    }
}
