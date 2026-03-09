mod call_stack;
mod frame;

use std::rc::Rc;
use crate::bytecode::program::Program;
use crate::bytecode::proto::Proto;
use crate::error::RuntimeError;
use crate::heap::Heap;
use crate::instr::Op;
use crate::value::DoughValue;
use crate::vm::call_stack::CallStack;
use crate::vm::frame::Frame;

macro_rules! bin_op {
    ($vm:expr, $op:tt) => {
        // todo simplify ALL of these macros, and move binary_op from DoughVm impl
    };
}

macro_rules! i_bin {
    ($vm:expr, $op:tt) => {
        $vm.binary_op(|a, b| {
            DoughValue::Int(a.as_i64() $op b.as_i64())
        })
    };
}

macro_rules! i_div {
    ($vm:expr) => {
        $vm.binary_op_fallible(|a, b| {
            let av = a.as_i64();
            let bv = b.as_i64();

            if bv == 0 {
                Err(RuntimeError::DivisionByZero)
            } else {
                let result = av / bv;
                Ok(DoughValue::Int(result))
            }
        })
    };
}

macro_rules! i_cmp {
    ($vm:expr, $op:tt) => {
        $vm.binary_op(|a, b| {
            DoughValue::Bool(a.as_i64() $op b.as_i64())
        })
    };
}

macro_rules! f_bin {
    ($vm:expr, $op:tt) => {
        $vm.binary_op(|a, b| {
            DoughValue::Float(a.as_f64() $op b.as_f64())
        })
    };
}

macro_rules! f_cmp {
    ($vm:expr, $op:tt) => {
        $vm.binary_op(|a, b| {
            DoughValue::Bool(a.as_f64() $op b.as_f64())
        })
    };
}

macro_rules! dst_src {
    ($vm:expr, |$dst:ident, $src:ident| $body:block) => {
        {
        let $dst = $vm.stack.next_u8();
        let $src = $vm.stack.next_u8();
        $body
        }
    };
}

macro_rules! cast {
    ($vm:expr, $method:ident, $typ:ty, $val:ident) => {
        dst_src!($vm, |dst, src| {
            let srcv = $vm.stack.get_value(src);
            let result = srcv.$method() as $typ;

            $vm.stack.set_value(dst, DoughValue::$val(result))
        })
    };
}

macro_rules! s_cmp {
    ($vm:expr, $op:tt) => {
        $vm.binary_op(|a, b| {
            let s1 = $vm.heap.get_object(a.as_handle()).as_str();
            let s2 = $vm.heap.get_object(b.as_handle()).as_str();
            DoughValue::Bool(s1.as_str() $op s2.as_str())
        })
    };
}

pub struct DoughVm {
    heap: Heap,
    globals: Vec<DoughValue>,
    stack: CallStack
}

impl DoughVm {
    pub fn new() -> Self {
        Self {
            heap: Heap::new(),
            globals: Vec::new(),
            stack: CallStack::new()
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), RuntimeError> {
        self.load_proto(program.get_main(), 0);
        self.dispatch_loop(program)?;

        *self = Self::new();
        Ok(())
    }

    fn load_proto(&mut self, proto: Rc<Proto>, return_reg: u8) {
        let frame = Frame::new(proto, return_reg);
        self.stack.push(frame);
    }

    fn dispatch_loop(&mut self, program: &Program)  -> Result<(), RuntimeError> {
        while !self.stack.is_empty() {
            let op = Op::from(self.stack.next_u8());

            match op {
                Op::Nop => {}

                // int
                Op::IAdd => i_bin!(self, +),
                Op::ISub => i_bin!(self, -),
                Op::IMul => i_bin!(self, *),
                Op::IDiv => i_div!(self)?,

                Op::INeg => self.unary_op(|src| DoughValue::Int(-src.as_i64())),

                Op::IEq => i_cmp!(self, ==),
                Op::INe => i_cmp!(self, !=),
                Op::ILt => i_cmp!(self, <),
                Op::ILe => i_cmp!(self, <=),
                Op::IGt => i_cmp!(self, >),
                Op::IGe => i_cmp!(self, >=),

                // float
                Op::FAdd => f_bin!(self, +),
                Op::FSub => f_bin!(self, -),
                Op::FMul => f_bin!(self, *),
                Op::FDiv => f_bin!(self, /),

                Op::FNeg => self.unary_op(|src| DoughValue::Float(-src.as_f64())),

                Op::FEq => f_cmp!(self, ==),
                Op::FNe => f_cmp!(self, !=),
                Op::FLt => f_cmp!(self, <),
                Op::FLe => f_cmp!(self, <=),
                Op::FGt => f_cmp!(self, >),
                Op::FGe => f_cmp!(self, >=),

                Op::I2F => cast!(self, as_i64, f64, Float),
                Op::F2I => cast!(self, as_f64, i64, Int),

                Op::Not => dst_src!(self, |dst, src| {
                    let result = self.stack.get_value(src).as_bool();
                    self.stack.set_value(dst, DoughValue::Bool(!result));
                }),

                Op::SEq => s_cmp!(self, ==),
                Op::SNe => s_cmp!(self, !=),

                Op::Concat => {}
                Op::LoadK => {}
                Op::LoadUnit => {}
                Op::LoadTrue => {}
                Op::LoadFalse => {}
                Op::Mov => {}
                Op::GetGlobal => {}
                Op::SetGlobal => {}
                Op::Jmp => {}
                Op::Jf => {}
                Op::Jt => {}
                Op::Call => {}
                Op::Ret => {}
                Op::NewArray => {}
                Op::ArrayLen => {}
                Op::GetIndex => {}
                Op::SetIndex => {}

                Op::_Count => panic!("invalid opcode {:?}", op)
            }
        }

        Ok(())
    }
}

// helpers
impl DoughVm {
    fn binary_op(
        &mut self,
        f: impl Fn(DoughValue, DoughValue) -> DoughValue
    ) {
        self.binary_op_fallible(|a, b| Ok(f(a, b))).unwrap();
    }

    fn binary_op_fallible(
        &mut self,
        f: impl Fn(DoughValue, DoughValue) -> Result<DoughValue, RuntimeError>
    ) -> Result<(), RuntimeError> {
        let dst = self.stack.next_u8();

        let a = self.stack.next_u8();
        let b = self.stack.next_u8();

        let av = self.stack.get_value(a);
        let bv = self.stack.get_value(b);

        let result = f(av, bv)?;
        self.stack.set_value(dst, result);

        Ok(())
    }

    fn unary_op(
        &mut self,
        f: impl Fn(DoughValue) -> DoughValue
    ) {
        let dst = self.stack.next_u8();
        let src = self.stack.next_u8();

        let srcv = self.stack.get_value(src);

        let result = f(srcv);
        self.stack.set_value(dst, result);
    }
}