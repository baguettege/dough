use crate::heap::handle::Handle;
use crate::value::DoughObject;

pub(in crate::heap) enum Slot {
    Free {
        next_free: Option<usize>,
        generation: u32
    },
    Occupied {
        object: DoughObject,
        marked: bool,
        generation: u32
    }
}

impl Slot {
    pub(in crate::heap) fn new() -> Self {
        Self::Free {
            next_free: None,
            generation: 0
        }
    }

    pub(in crate::heap) fn generation(&self) -> u32 {
        match self {
            Slot::Free { generation,  .. } => *generation,
            Slot::Occupied { generation,  .. } => *generation
        }
    }

    pub(in crate::heap) fn is_freed(&self) -> bool {
        match self {
            Slot::Free { .. } => true,
            Slot::Occupied { .. } => false,
        }
    }

    pub(in crate::heap) fn is_occupied(&self) -> bool {
        !self.is_freed()
    }

    pub(in crate::heap) fn occupy(&mut self, object: DoughObject) {
        *self = match self {
            Slot::Free { generation, .. } => Slot::Occupied {
                object,
                marked: false,
                generation: *generation
            },
            Slot::Occupied { .. } => panic!("Slot is already occupied")
        }
    }

    pub(in crate::heap) fn free(&mut self, next_free: Option<usize>) {
        *self = match self {
            Slot::Free { .. } => panic!("Slot is already freed"),
            Slot::Occupied { generation, .. } => Slot::Free {
                next_free,
                generation: generation.wrapping_add(1),
            }
        }
    }

    pub(in crate::heap) fn next_free(&self) -> Option<usize> {
        match self {
            Slot::Free { next_free, .. } => *next_free,
            Slot::Occupied { .. } => panic!("Occupied slot has no next_free")
        }
    }

    pub(in crate::heap) fn mark(&mut self) {
        match self {
            Slot::Free { .. } => panic!("Cannot mark a freed slot"),
            Slot::Occupied { marked, .. } => *marked = true
        }
    }

    pub(in crate::heap) fn unmark(&mut self) {
        match self {
            Slot::Free { .. } => panic!("Cannot unmark a freed slot"),
            Slot::Occupied { marked, .. } => *marked = false
        }
    }

    pub(in crate::heap) fn get_object(&self) -> &DoughObject {
        match self {
            Slot::Free { .. } => panic!("Freed slot has no object"),
            Slot::Occupied { object, .. } => object
        }
    }

    pub(in crate::heap) fn get_object_mut(&mut self) -> &mut DoughObject {
        match self {
            Slot::Free { .. } => panic!("Freed slot has no object"),
            Slot::Occupied { object, .. } => object
        }
    }

    pub(in crate::heap) fn is_marked(&self) -> bool {
        match self {
            Slot::Free { .. } => panic!("Freed slot cannot be marked"),
            Slot::Occupied { marked, .. } => *marked
        }
    }

    pub(in crate::heap) fn matches_handle(&self, handle: Handle) -> bool {
        self.generation() == handle.generation()
    }
}