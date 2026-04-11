use bytecode::{Chunk, Constant, Idx, Instr, Off};
use crate::vm::cursor::Cursor;
use crate::{Error, Result};

pub(super) struct Frame<'a> {
    base: usize,
    cursor: Cursor<'a>,
    constants: &'a [Constant],
}

impl<'a> Frame<'a> {
    pub(super) fn new(base: usize, chunk: &'a Chunk) -> Self {
        let cursor = Cursor::new(chunk.code());
        let constants = chunk.constants();
        Self { base, cursor, constants }
    }

    pub(super) fn base(&self) -> usize {
        self.base
    }

    pub(super) fn next(&mut self) -> Result<Instr> {
        self.cursor.next()
    }

    pub(super) fn jump(&mut self, off: Off) -> Result<()> {
        self.cursor.jump(off)
    }

    pub(super) fn constant(&self, idx: Idx) -> Result<&Constant> {
        self.constants
            .get(idx as usize)
            .ok_or(Error::IndexOutOfBounds)
    }
}

pub(super) struct Stack<'a>(Vec<Frame<'a>>);

impl<'a> Stack<'a> {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn push(&mut self, frame: Frame<'a>) {
        self.0.push(frame);
    }

    pub(super) fn pop(&mut self) -> Result<Frame<'a>> {
        self.0
            .pop()
            .ok_or(Error::StackUnderflow)
    }

    pub(super) fn current(&self) -> Result<&Frame<'a>> {
        self.0
            .last()
            .ok_or(Error::StackUnderflow)
    }

    pub(super) fn current_mut(&mut self) -> Result<&mut Frame<'a>> {
        self.0
            .last_mut()
            .ok_or(Error::StackUnderflow)
    }
}
