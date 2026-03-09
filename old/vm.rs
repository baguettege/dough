use std::rc::Rc;
use crate::bytecode::Bytecode;
use crate::bytecode::chunk::Chunk;
use crate::bytecode::proto_constant::ProtoConstant;
use crate::error::RuntimeError;
use crate::heap::handle::Handle;
use crate::heap::Heap;
use crate::value::DoughObject;
use crate::value::function::Function;
use crate::vm::frame::Frame;

mod frame;

pub struct DoughVm {
    heap: Heap,
    registers: Vec<Handle>,
    frames: Vec<Frame>
}

impl DoughVm {
    pub fn new() -> Self {
        Self {
            heap: Heap::new(),
            registers: Vec::new(),
            frames: Vec::new()
        }
    }

    pub fn run(&mut self, chunk: Chunk) -> Result<(), RuntimeError> {
        self.load(chunk);

        *self = Self::new();
        todo!();
    }

    fn load(&mut self, chunk: Chunk) {
        let (bytecode, constants, arity) = chunk.unpack();

        for constant in constants {
            let object = match constant {
                ProtoConstant::Number(n) => DoughObject::Number(n),
                ProtoConstant::Bool(b) => DoughObject::Bool(b),
                ProtoConstant::String(s) => DoughObject::String(s),
                ProtoConstant::Function(f) => {
                    let func = Function::new(
                        Rc::clone(bytecode),
                        constants,
                        arity,
                    );
                    DoughObject::Function(func)
                }
            };
        }

        todo!();
    }
}