use bytecode::{Idx, Reg};
use crate::{Result, Error};

#[derive(Default)]
pub(super) struct LocalAllocator {
    next: Reg,
    // dense fn local counts in the top level encounter order (indexed by `Slot::Fn(idx)`)
    // relies on stable AST traversal order
    counts: Vec<usize>,
}

impl LocalAllocator {
    pub(super) fn alloc_range(&mut self, n: usize) -> Result<Reg> {
        let start = self.next;
        let count: u8 = n.try_into().map_err(|_| Error::OutOfRegisters)?;
        self.next = self.next.checked_add(count).ok_or(Error::OutOfRegisters)?;
        Ok(start)
    }

    pub(super) fn alloc(&mut self) -> Result<Reg> {
        self.alloc_range(1)
    }

    pub(super) fn count(&self) -> usize {
        self.next as usize
    }

    pub(super) fn reset(&mut self) {
        self.next = 0;
    }

    pub(super) fn finish_fn(&mut self) {
        // does not take the fn's `Slot::Fn` index due to the invariant stated on `self.counts`
        let count = self.next as usize;
        self.counts.push(count);
        self.reset();
    }

    pub(super) fn counts(self) -> Vec<usize> {
        self.counts
    }
}

#[derive(Default)]
pub(super) struct IdxCounter {
    next: Idx,
}

impl IdxCounter {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn alloc(&mut self) -> Result<Idx> {
        let idx = self.next;
        self.next = self.next.checked_add(1).ok_or(Error::OutOfIndices)?;
        Ok(idx)
    }

    pub(super) fn count(&self) -> usize {
        self.next as usize
    }
}
