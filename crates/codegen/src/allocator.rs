mod collector;
mod counter;
mod common;
mod stmt;
mod expr;
mod program;

use ast::typed::Program;
use crate::allocator::counter::{IdxCounter, LocalAllocator};
use crate::{slot, Result};
use crate::layout::Layout;

#[derive(Default)]
struct Allocator {
    slots: slot::Table,
    global_counter: IdxCounter,
    local_allocator: LocalAllocator,
}

impl Allocator {
    fn new() -> Self {
        Self::default()
    }

    fn alloc(mut self, program: &Program) -> Result<Layout> {
        collector::collect(&mut self, program)?;

        let entry_local_count = self.alloc_entry(program)?;
        let global_count = self.global_counter.count();

        self.alloc_fns(program)?;
        let local_counts = self.local_allocator.counts();

        Ok(Layout::new(self.slots, global_count, entry_local_count, local_counts))
    }
}

pub(crate) fn alloc(program: &Program) -> Result<Layout> {
    Allocator::new().alloc(program)
}
