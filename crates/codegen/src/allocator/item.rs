use ast::typed::{Item, Static, Fn};
use crate::allocator::Allocator;
use crate::slot::Slot;
use crate::Result;

impl Allocator {
    pub(super) fn alloc_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Fn(node) => self.alloc_fn(node),
            Item::Static(node) => self.alloc_static(node),
        }
    }

    fn alloc_fn(&mut self, node: &Fn) -> Result<()> {
        let params = node.params();

        let reg_count = params.len() + 1; // + 1 for ret dst register
        let dst = self.local_counter.alloc_range(reg_count)?;

        for (param, reg) in params.iter().zip(dst + 1..) {
            self.slots.insert(param, Slot::Local(reg));
        }

        self.alloc_block(node.body())?;

        self.local_counter.finish_fn();
        Ok(())
    }

    fn alloc_static(&mut self, node: &Static) -> Result<()> {
        let idx = self.global_counter.alloc()?;
        let slot = Slot::Global(idx);
        self.slots.insert(node, slot);
        Ok(())
    }
}
