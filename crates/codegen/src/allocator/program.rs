use ast::typed::{Item, Program, Static, Fn};
use crate::allocator::Allocator;
use crate::{emitter, Result};
use crate::slot::Slot;

impl Allocator {
    pub(super) fn alloc_entry(&mut self, program: &Program) -> Result<usize> {
        for item in program {
            if let Item::Static(node) = item {
                self.alloc_static(node)?;
            }
        }

        // reserve a dst register for the startup call for 'main'
        self.local_allocator.alloc()?;

        let entry_local_count = self.local_allocator.count();
        self.local_allocator.reset();
        Ok(entry_local_count)
    }

    fn alloc_static(&mut self, node: &Static) -> Result<()> {
        self.alloc_expr(node.init())?;

        let idx = self.global_counter.alloc()?;
        self.slots.insert(node, Slot::Global(idx));

        Ok(())
    }
}

impl Allocator {
    pub(super) fn alloc_fns(&mut self, program: &Program) -> Result<()> {
        for item in program {
            if let Item::Fn(node) = item {
                self.alloc_fn(node)?;
            }
        }

        Ok(())
    }

    fn alloc_fn(&mut self, node: &Fn) -> Result<()> {
        let params = node.params();

        let reg_count = params.len() + 1; // + 1 for return dst register
        // reserve the function ABI registers, return dst then params
        let dst = self.local_allocator.alloc_range(reg_count)?;
        assert_eq!(dst, emitter::RET_REG);

        for (param, reg) in params.iter().zip(dst + 1..) {
            self.slots.insert(param, Slot::Local(reg));
        }

        self.alloc_block(node.body())?;

        self.local_allocator.finish_fn();
        Ok(())
    }
}
