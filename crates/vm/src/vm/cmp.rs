use crate::vm::Vm;
use crate::Result;

macro_rules! int_cmp_op {
    ($self:expr, $op:tt) => {
        {
            let rhs: i64 = pop!($self);
            let lhs: i64 = pop!($self);
            push!($self, $crate::value::Value::Bool(lhs $op rhs));
            Ok(())
        }
    };
}

macro_rules! float_cmp_op {
    ($self:expr, $op:tt) => {
        {
            let rhs: f64 = pop!($self);
            let lhs: f64 = pop!($self);
            push!($self, $crate::value::Value::Bool(lhs $op rhs));
            Ok(())
        }
    };
}

macro_rules! bool_cmp_op {
    ($self:expr, $op:tt) => {
        {
            let rhs: bool = pop!($self);
            let lhs: bool = pop!($self);
            push!($self, $crate::value::Value::Bool(lhs $op rhs));
            Ok(())
        }
    };
}

macro_rules! str_cmp_op {
    ($self:expr, $op:tt) => {
        {
            let rhs: heap::Handle<$crate::value::Str> = pop!($self);
            let lhs: heap::Handle<$crate::value::Str> = pop!($self);

            let rhs = $self.heap.with(
                &rhs, |obj| obj.as_str().to_string());
            let result = $self.heap.with(
                &lhs, |obj| obj.as_str() == rhs);

            push!($self, $crate::value::Value::Bool(result));
            Ok(())
        }
    };
}

impl Vm<'_> {
    pub(super) fn op_ieq(&mut self) -> Result<()> {
        int_cmp_op!(self, ==)
    }

    pub(super) fn op_ine(&mut self) -> Result<()> {
        int_cmp_op!(self, !=)
    }

    pub(super) fn op_ilt(&mut self) -> Result<()> {
        int_cmp_op!(self, <)
    }

    pub(super) fn op_ile(&mut self) -> Result<()> {
        int_cmp_op!(self, <=)
    }

    pub(super) fn op_igt(&mut self) -> Result<()> {
        int_cmp_op!(self, >)
    }

    pub(super) fn op_ige(&mut self) -> Result<()> {
        int_cmp_op!(self, >=)
    }

    pub(super) fn op_feq(&mut self) -> Result<()> {
        float_cmp_op!(self, ==)
    }

    pub(super) fn op_fne(&mut self) -> Result<()> {
        float_cmp_op!(self, !=)
    }

    pub(super) fn op_flt(&mut self) -> Result<()> {
        float_cmp_op!(self, <)
    }

    pub(super) fn op_fle(&mut self) -> Result<()> {
        float_cmp_op!(self, <=)
    }

    pub(super) fn op_fgt(&mut self) -> Result<()> {
        float_cmp_op!(self, >)
    }

    pub(super) fn op_fge(&mut self) -> Result<()> {
        float_cmp_op!(self, >=)
    }

    pub(super) fn op_beq(&mut self) -> Result<()> {
        bool_cmp_op!(self, ==)
    }

    pub(super) fn op_bne(&mut self) -> Result<()> {
        bool_cmp_op!(self, !=)
    }

    pub(super) fn op_seq(&mut self) -> Result<()> {
        str_cmp_op!(self, ==)
    }

    pub(super) fn op_sne(&mut self) -> Result<()> {
        str_cmp_op!(self, !=)
    }
}
