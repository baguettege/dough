use crate::heap::handle::Handle;
use crate::heap::slot::Slot;
use crate::value::array::DoughArray;
use crate::value::DoughObject;
use crate::value::str::DoughStr;

pub mod handle;
mod slot;

pub struct Heap {
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

    pub(crate) fn get_object(&self, handle: Handle) -> &DoughObject {
        // handle must be valid & not stale
        let slot = &self.slots[handle.index()];

        if slot.matches_handle(handle) {
            slot.get_object()
        } else {
            panic!("Stale handle {:?}", handle);
        }
    }

    pub(crate) fn get_object_mut(&mut self, handle: Handle) -> &mut DoughObject {
        // handle must be valid & not stale
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
            // handle must be valid & not stale
            let slot = &mut self.slots[handle.index()];

            if slot.is_freed() || !slot.matches_handle(handle) {
                panic!("Stale handle {:?}", handle);
            } else if slot.is_marked() {
                continue;
            }

            slot.mark();

            let children = match slot.get_object() {
                DoughObject::Array(array) => array.references(),
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

    pub fn alloc_str(&mut self, s: DoughStr) -> Handle {
        self.alloc(DoughObject::Str(s))
    }

    pub fn alloc_array(&mut self, array: DoughArray) -> Handle {
        self.alloc(DoughObject::Array(array))
    }

    pub fn get_str(&self, handle: Handle) -> &DoughStr {
        match self.get_object(handle) {
            DoughObject::Str(s) => s,
            _ => panic!("Handle {:?} does not point to a DoughStr", handle)
        }
    }

    pub fn get_str_mut(&mut self, handle: Handle) -> &mut DoughStr {
        match self.get_object_mut(handle) {
            DoughObject::Str(s) => s,
            _ => panic!("Handle {:?} does not point to a DoughStr", handle)
        }
    }

    pub fn get_array(&self, handle: Handle) -> &DoughArray {
        match self.get_object(handle) {
            DoughObject::Array(array) => array,
            _ => panic!("Handle {:?} does not point to a DoughArray", handle)
        }
    }

    pub fn get_array_mut(&mut self, handle: Handle) -> &mut DoughArray {
        match self.get_object_mut(handle) {
            DoughObject::Array(array) => array,
            _ => panic!("Handle {:?} does not point to a DoughArray", handle)
        }
    }
}