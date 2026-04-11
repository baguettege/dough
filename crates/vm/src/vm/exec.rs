use bytecode::Instr;
use crate::Result;
use crate::vm::{ControlFlow, Vm};

impl Vm<'_> {
    pub(super) fn dispatch(&mut self) -> Result<()> {
        loop {
            let instr = self.frames.current_mut()?.next()?;
            if matches!(self.exec(instr)?, ControlFlow::Halt) {
                return Ok(())
            }
        }
    }

    fn exec(&mut self, instr: Instr) -> Result<ControlFlow> {
        match instr {
            Instr::Nop => {},
            Instr::Halt => return Ok(ControlFlow::Halt),

            Instr::IAdd => self.op_iadd()?,
            Instr::ISub => self.op_isub()?,
            Instr::IMul => self.op_imul()?,
            Instr::IDiv => self.op_idiv()?,
            Instr::INeg => self.op_ineg()?,

            Instr::IEq => self.op_ieq()?,
            Instr::INe => self.op_ine()?,
            Instr::ILt => self.op_ilt()?,
            Instr::ILe => self.op_ile()?,
            Instr::IGt => self.op_igt()?,
            Instr::IGe => self.op_ige()?,

            Instr::FAdd => self.op_fadd()?,
            Instr::FSub => self.op_fsub()?,
            Instr::FMul => self.op_fmul()?,
            Instr::FDiv => self.op_fdiv()?,
            Instr::FNeg => self.op_fneg()?,

            Instr::FEq => self.op_feq()?,
            Instr::FNe => self.op_fne()?,
            Instr::FLt => self.op_flt()?,
            Instr::FLe => self.op_fle()?,
            Instr::FGt => self.op_fgt()?,
            Instr::FGe => self.op_fge()?,

            Instr::BAnd => self.op_band()?,
            Instr::BOr => self.op_bor()?,
            Instr::BNot => self.op_bnot()?,

            Instr::BEq => self.op_beq()?,
            Instr::BNe => self.op_bne()?,

            Instr::SAdd => self.op_sadd()?,
            Instr::SEq => self.op_seq()?,
            Instr::SNe => self.op_sne()?,

            Instr::Jmp(off) => self.op_jmp(off)?,
            Instr::Jf(off) => self.op_jf(off)?,

            Instr::Call(idx, argc) => self.op_call(idx, argc)?,
            Instr::Ret => self.op_ret()?,

            Instr::Push(idx) => self.op_push(idx)?,
            Instr::PushU => self.op_pushu()?,
            Instr::Pop => self.op_pop()?,

            Instr::Ldl(idx) => self.op_ldl(idx)?,
            Instr::Stl(idx) => self.op_stl(idx)?,
        }

        Ok(ControlFlow::Continue)
    }
}
