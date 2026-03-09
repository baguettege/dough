use std::rc::Rc;
use crate::bytecode::Bytecode;
use crate::heap::handle::Handle;

pub(in crate::vm) struct Frame {
    function: Handle,
    bytecode: Rc<Bytecode>,
    base: usize,
    ip: usize
}

impl Frame {
    pub(in crate::vm) fn new(
        function: Handle, bytecode: Rc<Bytecode>, base: usize) -> Self {
        Self {
            function,
            bytecode,
            base,
            ip: 0
        }
    }

    pub(in crate::vm) fn function(&self) -> &Handle {
        &self.function
    }

    pub(in crate::vm) fn base(&self) -> usize {
        self.base
    }

    pub(in crate::vm) fn next_u8(&mut self) -> u8 {
        let b = self.bytecode.read_u8(self.ip);
        self.ip += 1;
        b
    }
    
    pub(in crate::vm) fn next_u16(&mut self) -> u16 {
        let b = self.bytecode.read_u16(self.ip);
        self.ip += 2;
        b
    }
    
    pub(in crate::vm) fn next_i8(&mut self) -> i8 {
        self.next_u8() as i8
    }

    pub(in crate::vm) fn next_i16(&mut self) -> i16 {
        self.next_u8() as i16
    }
}