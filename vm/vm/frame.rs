use std::rc::Rc;
use crate::bytecode::constant::Constant;
use crate::bytecode::proto::Proto;
use crate::value::DoughValue;

pub(in crate::vm) struct Frame {
    registers: Vec<DoughValue>,
    proto: Rc<Proto>,
    return_reg: u8,
    ip: usize
}

impl Frame {
    pub(in crate::vm) fn new(proto: Rc<Proto>, return_reg: u8) -> Self {
        let reg_count = proto.reg_count();
        Self {
            registers: vec![DoughValue::Unit; reg_count],
            proto,
            return_reg,
            ip: 0
        }
    }

    pub(in crate::vm) fn get_value(&self, index: u8) -> DoughValue {
        self.registers[index as usize]
    }

    pub(in crate::vm) fn set_value(&mut self, index: u8, value: DoughValue) {
        self.registers[index as usize] = value;
    }

    pub(in crate::vm) fn get_constant(&self, index: u16) -> &Constant {
        self.proto.chunk().get_constant(index)
    }

    pub(in crate::vm) fn return_reg(&self) -> u8 {
        self.return_reg
    }

    pub(in crate::vm) fn next_u8(&mut self) -> u8 {
        let b = self.proto.chunk().read_u8(self.ip);
        self.ip += 1;
        b
    }
}