use heap::Handle;
use crate::vm::Vm;
use crate::{Error, Result};
use crate::value::{Str, Value};

macro_rules! int_bin_op {
    ($self:expr, $op:tt) => {
        {
            let rhs: i64 = pop!($self);
            let lhs: i64 = pop!($self);
            push!($self, $crate::value::Value::Int(lhs $op rhs));
            Ok(())
        }
    };
}

macro_rules! float_bin_op {
    ($self:expr, $op:tt) => {
        {
            let rhs: f64 = pop!($self);
            let lhs: f64 = pop!($self);
            push!($self, $crate::value::Value::Float(lhs $op rhs));
            Ok(())
        }
    };
}

impl Vm<'_> {
    pub(super) fn op_iadd(&mut self) -> Result<()> {
        int_bin_op!(self, +)
    }

    pub(super) fn op_isub(&mut self) -> Result<()> {
        int_bin_op!(self, -)
    }

    pub(super) fn op_imul(&mut self) -> Result<()> {
        int_bin_op!(self, *)
    }

    pub(super) fn op_idiv(&mut self) -> Result<()> {
        let rhs: i64 = pop!(self);
        let lhs: i64 = pop!(self);

        if rhs != 0 {
            push!(self, Value::Int(lhs / rhs));
            Ok(())
        } else {
            Err(Error::DivisionByZero)
        }
    }

    pub(super) fn op_ineg(&mut self) -> Result<()> {
        let value: i64 = pop!(self);
        push!(self, Value::Int(-value));
        Ok(())
    }

    pub(super) fn op_fadd(&mut self) -> Result<()> {
        float_bin_op!(self, +)
    }

    pub(super) fn op_fsub(&mut self) -> Result<()> {
        float_bin_op!(self, -)
    }

    pub(super) fn op_fmul(&mut self) -> Result<()> {
        float_bin_op!(self, *)
    }

    pub(super) fn op_fdiv(&mut self) -> Result<()> {
        float_bin_op!(self, /)
    }

    pub(super) fn op_fneg(&mut self) -> Result<()> {
        let value: f64 = pop!(self);
        push!(self, Value::Float(-value));
        Ok(())
    }

    pub(super) fn op_sadd(&mut self) -> Result<()> {
        let rhs: Handle<Str> = pop!(self);
        let lhs: Handle<Str> = pop!(self);

        let rhs = self.heap.with(
            rhs, |obj| obj.as_str().to_string());
        let result = self.heap.with(
            lhs, |obj| format!("{}{}", obj.as_str(), rhs));

        let handle = self.heap.alloc(Str::new(result));
        push!(self, Value::Str(handle));

        Ok(())
    }
}
