use std::rc::Rc;
use crate::bytecode::Bytecode;
use crate::heap::handle::Handle;

pub struct Function {
    bytecode: Rc<Bytecode>,
    constants: Vec<Handle>,
    arity: u8,
    upvalues: Vec<Handle>
}

impl Function {
    pub fn new(bytecode: Rc<Bytecode>, constants: Vec<Handle>,
               arity: u8, upvalues: Vec<Handle>) -> Self {
        Self {
            bytecode,
            constants,
            arity,
            upvalues,
        }
    }

    pub fn references(&self) -> Vec<Handle> {
        self.constants.iter()
            .chain(self.upvalues.iter())
            .copied()
            .collect()
    }
// todo claude is saying some shit about FunctionProto and Closure as separate variants to this
    pub fn bytecode(&self) -> &Bytecode {
        &self.bytecode
    }

    pub fn bytecode_rc(&self) -> Rc<Bytecode> {
        Rc::clone(&self.bytecode)
    }

    pub fn constants(&self) -> &[Handle] {
        &self.constants
    }

    pub fn arity(&self) -> u8 {
        self.arity
    }

    pub fn upvalues(&self) -> &[Handle] {
        &self.upvalues
    }
}