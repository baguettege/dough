use std::rc::Rc;
use crate::bytecode::constant::Constant;

pub(crate) struct Function {
    code: Rc<Vec<u8>>,
    constants: Vec<Constant>,
    arity: u8,
    reg_count: usize
}

impl Function {
    pub(crate) fn new(
        code: Vec<u8>,
        constants: Vec<Constant>,
        arity: u8,
        reg_count: usize
    ) -> Self {
        Self { code: Rc::new(code), constants, arity, reg_count }
    }

    pub(crate) fn code(&self) -> Rc<Vec<u8>> {
        self.code.clone()
    }

    pub(crate) fn constant(&self, index: usize) -> &Constant {
        &self.constants[index]
    }

    pub(crate) fn arity(&self) -> u8 {
        self.arity
    }

    pub(crate) fn reg_count(&self) -> usize {
        self.reg_count
    }
}