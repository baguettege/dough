use std::rc::Rc;
use crate::bytecode::constant::Constant;
use crate::bytecode::function::Function;
use crate::instr::decoder::Decoder;
use crate::instr::Instr;
use crate::value::DoughValue;
use crate::vm::registers::Registers;

pub(super) struct Frame {
    registers: Registers,
    function: Rc<Function>,
    return_reg: u8,
    decoder: Decoder
}

impl Frame {
    pub(super) fn new(function: Rc<Function>, return_reg: u8) -> Self {
        let reg_count = function.reg_count();
        let code = function.code();

        Self {
            registers: Registers::new(reg_count),
            function,
            return_reg,
            decoder: Decoder::new(code)
        }
    }

    pub(super) fn get_reg(&self, index: usize) -> DoughValue {
        self.registers.get(index)
    }

    pub(super) fn set_reg(&mut self, index: usize, value: DoughValue) {
        self.registers.set(index, value);
    }

    pub(super) fn has_next(&self) -> bool {
        self.decoder.has_next()
    }

    pub(super) fn next_instr(&mut self) -> Instr {
        self.decoder.next_instr()
    }

    pub(super) fn jump(&mut self, offset: i16) {
        self.decoder.jump(offset)
    }

    pub(super) fn constant(&self, index: usize) -> &Constant {
        self.function.constant(index)
    }

    pub(super) fn arity(&self) -> u8 {
        self.function.arity()
    }

    pub(super) fn reg_count(&self) -> usize {
        self.function.reg_count()
    }
}