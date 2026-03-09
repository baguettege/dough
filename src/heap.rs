use crate::heap::handle::Handle;
use crate::heap::slot::Slot;
use crate::heap::trace::GcTrace;
use crate::value::DoughObject;

pub mod handle;
mod slot;
pub(crate) mod trace;

pub(crate) struct Heap {
    slots: Vec<Slot>,
    free_head: Option<usize>
}

impl Heap {
    pub(crate) fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_head: None
        }
    }

    pub(crate) fn alloc(&mut self, object: DoughObject) -> Handle {
        match self.free_head {
            None => {
                let index = self.slots.len();
                let mut slot = Slot::new();

                slot.occupy(object);
                self.slots.push(slot);

                Handle::new(index, 0)
            }
            Some(index) => {
                let slot = &mut self.slots[index];
                self.free_head = slot.next_free();

                slot.occupy(object);

                Handle::new(index, slot.generation())
            }
        }
    }

    pub(crate) fn deref(&self, handle: Handle) -> &DoughObject {
        let slot = &self.slots[handle.index()];

        if slot.matches_handle(handle) {
            slot.get_object()
        } else {
            panic!("Stale handle {:?}", handle);
        }
    }

    pub(crate) fn deref_mut(&mut self, handle: Handle) -> &mut DoughObject {
        let slot = &mut self.slots[handle.index()];

        if slot.matches_handle(handle) {
            slot.get_object_mut()
        } else {
            panic!("Stale handle {:?}", handle);
        }
    }

    pub(crate) fn gc(&mut self, roots: &[Handle]) {
        // mark
        let mut worklist = roots.to_vec();

        while let Some(handle) = worklist.pop() {
            let slot = &mut self.slots[handle.index()];

            if slot.is_freed() || !slot.matches_handle(handle) {
                panic!("Stale handle {:?}", handle);
            } else if slot.is_marked() {
                continue;
            }

            slot.mark();

            let children = slot.get_object().references();

            worklist.extend(children);
        }

        // sweep
        for (index, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_freed() {
                continue;
            }

            if slot.is_marked() {
                slot.unmark();
            } else {
                slot.free(self.free_head);
                self.free_head = Some(index);
            }
        }
    }
}