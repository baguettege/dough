mod collector;
mod counter;
mod item;
mod common;
mod stmt;

use ast::typed::Program;
use crate::allocator::counter::{IdxCounter, LocalCounter};
use crate::{slot, Result};

pub(crate) struct Allocation {
    slots: slot::Table,
    global_count: usize,
    local_counts: Vec<usize>, // indexed by `Slot::Fn(idx)`
}

impl Allocation {
    fn new(slots: slot::Table, global_count: usize, local_counts: Vec<usize>) -> Self {
        Self { slots, global_count, local_counts }
    }

    pub(crate) fn slots(&self) -> &slot::Table {
        &self.slots
    }

    pub(crate) fn global_count(&self) -> usize {
        self.global_count
    }

    pub(crate) fn local_counts(&self) -> &[usize] {
        &self.local_counts
    }
}

#[derive(Default)]
struct Allocator {
    slots: slot::Table,
    global_counter: IdxCounter,
    local_counter: LocalCounter,
}

impl Allocator {
    fn new() -> Self {
        Self::default()
    }

    fn alloc(mut self, program: &Program) -> Result<Allocation> {
        collector::collect(&mut self, program)?;
        program.iter().try_for_each(|item| self.alloc_item(item))?;

        let global_count = self.global_counter.count();
        let local_counts = self.local_counter.counts();
        Ok(Allocation::new(self.slots, global_count, local_counts))
    }
}

pub(crate) fn alloc(program: &Program) -> Result<Allocation> {
    Allocator::new().alloc(program)
}
