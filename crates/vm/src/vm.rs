use crate::vm::frame::Frame;
use crate::Result;
use bytecode::Program;
use heap::Heap;

macro_rules! push {
    ($self:expr, $value:expr) => {
        $self.operands.push($value)
    };
}

macro_rules! pop {
    ($self:expr) => {
        $self.operands.pop()?.try_into()?
    };
}

mod frame;
mod cursor;
mod operand;
mod arith;
mod cmp;
mod logic;
mod flow;
mod mem;
mod exec;

enum ControlFlow {
    Continue,
    Halt,
}

struct Vm<'a> {
    program: &'a Program,
    frames: frame::Stack<'a>,
    operands: operand::Stack,
    heap: Heap,
}

impl<'a> Vm<'a> {
    fn new(program: &'a Program) -> Self {
        let frames = frame::Stack::new();
        let operands = operand::Stack::new();
        let heap = Heap::new();
        Self { program, frames, operands, heap }
    }

    fn run(mut self) -> Result<()> {
        self.setup();
        self.dispatch()
    }

    fn setup(&mut self) {
        let entry = self.program.entry();
        let frame = Frame::new(0, entry);
        self.frames.push(frame);
    }
}

pub(crate) fn run(program: &Program) -> Result<()> {
    Vm::new(program).run()
}
