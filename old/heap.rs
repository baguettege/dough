use crate::heap::handle::Handle;
use crate::heap::slot::Slot;
use crate::value::DoughObject;

pub mod handle;
mod slot;

pub struct Heap {
    slots: Vec<Slot>,
    free_head: Option<usize>
}

impl Heap {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            free_head: None
        }
    }

    pub fn alloc(&mut self, object: DoughObject) -> Handle {
        match self.free_head {
            None => {
                let index = self.slots.len();
                let mut slot = Slot::new_free();

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

    pub fn get_object(&self, handle: Handle) -> &DoughObject {
        // SAFETY: the handle index is valid & the generation matches
        let slot = &self.slots[handle.index];

        if !slot.matches_handle(handle) {
            panic!("Stale handle for generation {}", handle.generation)
        }

        slot.get_object()
    }

    pub fn get_object_mut(&mut self, handle: Handle) -> &mut DoughObject {
        // SAFETY: the handle index is valid & the generation matches
        let slot = &mut self.slots[handle.index];

        if !slot.matches_handle(handle) {
            panic!("Stale handle for generation {}", handle.generation)
        }

        slot.get_object_mut()
    }

    pub(crate) fn gc(&mut self, roots: &[Handle]) {
        // mark
        let mut worklist = roots.to_vec();

        while let Some(handle) = worklist.pop() {
            // SAFETY: the handle index is valid
            let slot = &mut self.slots[handle.index];

            if slot.is_freed() || !slot.matches_handle(handle) {
                panic!("Handle is invalid");
            }

            if slot.is_marked() {
                continue;
            }

            slot.mark();

            let children = match slot.get_object() {
                DoughObject::Array(arr) => arr.references().to_vec(),
                _ => Vec::new()
            };

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