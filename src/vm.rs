mod frame;
mod call_stack;
mod registers;

use std::rc::Rc;
use crate::bytecode::function::Function;
use crate::bytecode::program::Program;
use crate::error::RuntimeError;
use crate::heap::Heap;
use crate::instr::Instr;
use crate::value::DoughValue;
use crate::vm::call_stack::CallStack;
use crate::vm::frame::Frame;
use crate::vm::registers::Registers;

type VmResult = Result<(), RuntimeError>;

pub struct DoughVm {
    heap: Heap,
    globals: Registers,
    stack: CallStack
}

impl DoughVm {
    pub fn run(program: &Program) -> VmResult {
        Self::new(program).run_inner(program)
    }

    fn new(program: &Program) -> Self {
        let global_count = program.global_count();

        Self {
            heap: Heap::new(),
            globals: Registers::new(global_count),
            stack: CallStack::new()
        }
    }

    fn run_inner(mut self, program: &Program) -> VmResult {
        self.push_function(program.main(), 0);
        self.dispatch_loop()?;
        Ok(())
    }

    fn push_function(&mut self, func: Rc<Function>, return_reg: u8) {
        let frame = Frame::new(func, return_reg);
        self.stack.push(frame);
    }

    fn dispatch_loop(&mut self) -> VmResult {
        while !self.stack.is_empty() {
            let instr = self.stack.current().next_instr();
            self.exec_instr(instr)?;
        }
        Ok(())
    }

    fn exec_instr(&mut self, instr: Instr) -> VmResult {
        let frame = self.stack.current();
        
        match instr {
            Instr::Nop { .. } => {}

            Instr::IAdd { dst, a, b } => {
                let ra = frame.get_reg(a).as_i64();
                let rb = frame.get_reg(b).as_i64();
                frame.set_reg(dst, DoughValue::Int(ra + rb));
            }
            Instr::ISub { dst, a, b } => {}
            Instr::IMul { dst, a, b } => {}
            Instr::IDiv { dst, a, b } => {}

            Instr::IEq { dst, a, b } => {}
            Instr::INe { dst, a, b } => {}
            Instr::ILt { dst, a, b } => {}
            Instr::ILe { dst, a, b } => {}
            Instr::IGt { dst, a, b } => {}
            Instr::IGe { dst, a, b } => {}

            Instr::INeg { dst, src } => {
                let rsrc = frame.get_reg(src).as_i64();
                frame.set_reg(dst, DoughValue::Int(-rsrc));
            }

            Instr::FAdd { dst, a, b } => {}
            Instr::FSub { dst, a, b } => {}
            Instr::FMul { dst, a, b } => {}
            Instr::FDiv { dst, a, b } => {}

            Instr::FEq { dst, a, b } => {}
            Instr::FNe { dst, a, b } => {}
            Instr::FLt { dst, a, b } => {}
            Instr::FLe { dst, a, b } => {}
            Instr::FGt { dst, a, b } => {}
            Instr::FGe { dst, a, b } => {}

            Instr::FNeg { dst, src } => {
                let rsrc = frame.get_reg(src).as_f64();
                frame.set_reg(dst, DoughValue::Float(-rsrc));
            }

            Instr::Not { dst, src } => {}

            Instr::SEq { dst, a, b } => {}
            Instr::SNe { dst, a, b } => {}
            Instr::Concat { dst, a, b } => {}

            Instr::I2F { dst, src } => {}
            Instr::F2I { dst, src } => {}

            Instr::Mov { dst, src } => {
                let rsrc = frame.get_reg(src);
                frame.set_reg(dst, rsrc);
            }
            Instr::LoadConst { dst, idx } => {}
            Instr::LoadUnit { dst } => {}

            Instr::GetGlobal { dst, idx } => {}
            Instr::SetGlobal { idx, src } => {}

            Instr::Jmp { off } => {}
            Instr::Jf { src, off } => {}

            Instr::Call { idx, arg_start, ret } => {}
            Instr::Ret { src } => {}

            Instr::NewArray { dst, size } => {}
            Instr::ArrayLen { dst, src } => {}
            Instr::GetIndex { dst, idx, arr } => {}
            Instr::SetIndex { idx, src, arr } => {}
        }

        todo!()
    }
}