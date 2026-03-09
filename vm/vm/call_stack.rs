use crate::bytecode::constant::Constant;
use crate::value::DoughValue;
use crate::vm::frame::Frame;

pub(in crate::vm) struct CallStack {
    frames: Vec<Frame>
}

impl CallStack {
    pub(in crate::vm) fn new() -> Self {
        Self { frames: Vec::new() }
    }

    fn current_frame(&self) -> &Frame {
        self.frames.last().expect("empty call stack")
    }

    fn current_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().expect("empty call stack")
    }

    pub(in crate::vm) fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub(in crate::vm) fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub(in crate::vm) fn pop(&mut self) -> Frame {
        self.frames.pop().expect("call stack underflow")
    }

    pub(in crate::vm) fn get_value(&self, index: u8) -> DoughValue {
        self.current_frame().get_value(index)
    }

    pub(in crate::vm) fn set_value(&mut self, index: u8, value: DoughValue) {
        self.current_frame_mut().set_value(index, value);
    }

    pub(in crate::vm) fn get_constant(&self, index: u16) -> &Constant {
        self.current_frame().get_constant(index)
    }

    pub(in crate::vm) fn next_u8(&mut self) -> u8 {
        self.current_frame_mut().next_u8()
    }

    pub(in crate::vm) fn next_u16(&mut self) -> u16 {
        let hi = self.next_u8() as u16;
        let lo = self.next_u8() as u16;
        (hi << 8) | lo
    }

    pub(in crate::vm) fn next_u32(&mut self) -> u32 {
        let hi = self.next_u16() as u32;
        let lo = self.next_u16() as u32;
        (hi << 16) | lo
    }

    pub(in crate::vm) fn next_i16(&mut self) -> i16 {
        self.next_u16() as i16
    }

    pub(in crate::vm) fn next_reg(&mut self) -> DoughValue {
        let reg = self.next_u8();
        self.get_value(reg)
    }
}