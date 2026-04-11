use crate::{Error, Result};
use crate::value::Value;

pub(super) struct Stack(Vec<Value>);

impl Stack {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn len(&self) -> usize {
        self.0.len()
    }

    pub(super) fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    pub(super) fn pop(&mut self) -> Result<Value> {
        self.0
            .pop()
            .ok_or(Error::StackUnderflow)
    }

    pub(super) fn get(&self, idx: usize) -> Result<Value> {
        self.0
            .get(idx)
            .copied()
            .ok_or(Error::IndexOutOfBounds)
    }

    pub(super) fn set(&mut self, idx: usize, value: Value) -> Result<()> {
        self.0
            .get_mut(idx)
            .map(|val| *val = value)
            .ok_or(Error::IndexOutOfBounds)
    }

    pub(super) fn reserve(&mut self, n: usize) {
        self.0.resize(self.0.len() + n, Value::Unit);
    }

    pub(super) fn truncate(&mut self, base: usize) {
        self.0.truncate(base);
    }
}
