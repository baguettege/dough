use crate::vm::frame::Frame;

pub(super) struct CallStack {
    frames: Vec<Frame>
}

impl CallStack {
    pub(super) fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub(super) fn push(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub(super) fn pop(&mut self) -> Frame {
        self.frames.pop().expect("call stack underflow")
    }

    pub(super) fn current(&mut self) -> &mut Frame {
        self.frames.last_mut().expect("call stack underflow")
    }

    pub(super) fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    pub(super) fn with_current<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Frame)
    {
        let frame = self.current();
        f(frame);
    }
}