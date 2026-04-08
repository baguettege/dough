use ast::typed::{Item, Program, Fn};
use crate::allocator::Allocator;
use crate::allocator::counter::IdxCounter;
use crate::slot::Slot;
use crate::Result;

struct Collector<'a> {
    allocator: &'a mut Allocator,
    fn_counter: IdxCounter,
}

impl<'a> Collector<'a> {
    fn new(allocator: &'a mut Allocator) -> Self {
        let fn_counter = IdxCounter::new();
        Self { allocator, fn_counter }
    }

    fn collect(mut self, program: &Program) -> Result<()> {
        program.iter().try_for_each(|item| self.collect_item(item))
    }

    fn collect_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Fn(node) => self.collect_fn(node),
            Item::Static(_) => Ok(()), // skipped, handled in `Allocator`
        }
    }

    fn collect_fn(&mut self, node: &Fn) -> Result<()> {
        let idx = self.fn_counter.alloc()?;
        self.allocator.slots.insert(node, Slot::Fn(idx));
        Ok(())
    }
}

pub(super) fn collect(allocator: &mut Allocator, program: &Program) -> Result<()> {
    Collector::new(allocator).collect(program)
}
