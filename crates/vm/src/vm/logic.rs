use crate::Result;
use crate::value::Value;
use crate::vm::Vm;

macro_rules! bool_bin_op {
    ($self:expr, $op:tt) => {
        {
            let rhs: bool = $self.operands.pop()?.try_into()?;
            let lhs: bool = $self.operands.pop()?.try_into()?;
            let result = Value::Bool(lhs $op rhs);
            $self.operands.push(result);
            Ok(())
        }
    };
}

impl Vm<'_> {
    pub(super) fn op_band(&mut self) -> Result<()> {
        bool_bin_op!(self, &&)
    }

    pub(super) fn op_bor(&mut self) -> Result<()> {
        bool_bin_op!(self, ||)
    }

    pub(super) fn op_bnot(&mut self) -> Result<()> {
        let value: bool = self.operands.pop()?.try_into()?;
        let result = Value::Bool(!value);
        self.operands.push(result);
        Ok(())
    }
}
