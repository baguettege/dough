use crate::heap::handle::Handle;
use crate::value::DoughObject;

pub(in crate::heap) enum Slot {
    Free {
        generation: u32,
        next_free: Option<usize>
    },
    Occupied {
        generation: u32,
        object: DoughObject,
        marked: bool
    }
}

impl Slot {
    pub(in crate::heap) fn new_free() -> Self {
        Self::Free {
            generation: 0,
            next_free: None,
        }
    }

    pub(in crate::heap) fn is_freed(&self) -> bool {
        match self {
            Slot::Free { .. } => true,
            Slot::Occupied { .. } => false
        }
    }

    pub(in crate::heap) fn is_occupied(&self) -> bool {
        !self.is_freed()
    }

    pub(in crate::heap) fn occupy(&mut self, object: DoughObject) {
        // SAFETY: slot is not already occupied
        match self {
            Slot::Free { generation, .. } => {
                *self = Slot::Occupied {
                    generation: *generation,
                    object,
                    marked: false
                }
            }
            Slot::Occupied { .. } => panic!("Attempted to occupy an occupied slot")
        }
    }

    pub(in crate::heap) fn free(&mut self, next_free: Option<usize>) {
        // SAFETY: slot is not already freed
        match self {
            Slot::Free { .. } => panic!("Attempted to free a freed slot"),
            Slot::Occupied { generation, .. } => {
                *self = Slot::Free {
                    generation: generation.wrapping_add(1),
                    next_free
                }
            }
        }
    }

    pub(in crate::heap) fn generation(&self) -> u32 {
        match self {
            Slot::Free { generation, .. } => *generation,
            Slot::Occupied { generation, .. } => *generation
        }
    }

    pub(in crate::heap) fn next_free(&self) -> Option<usize> {
        // SAFETY: slot is free
        match self {
            Slot::Free { next_free, .. } => *next_free,
            Slot::Occupied { .. } => panic!(
                "Attempted to obtain next free slot on an occupied slot")
        }
    }

    pub(in crate::heap) fn get_object(&self) -> &DoughObject {
        // SAFETY: slot is occupied
        match self {
            Slot::Free { .. } => panic!("Attempted to access object in a freed slot"),
            Slot::Occupied { object, .. } => object
        }
    }

    pub(in crate::heap) fn get_object_mut(&mut self) -> &mut DoughObject {
        // SAFETY: slot is occupied
        match self {
            Slot::Free { .. } => panic!("Attempted to access object in a freed slot"),
            Slot::Occupied { object, .. } => object
        }
    }

    pub(in crate::heap) fn mark(&mut self) {
        // SAFETY: slot is occupied
        match self {
            Slot::Free { .. } => panic!("Attempted to mark a freed slot"),
            Slot::Occupied { marked, .. } => {
                if *marked {
                    panic!("Attempted to mark a marked slot")
                } else {
                    *marked = true;
                }
            }
        }
    }

    pub(in crate::heap) fn unmark(&mut self) {
        // SAFETY: slot is occupied
        match self {
            Slot::Free { .. } => panic!("Attempted to unmark a freed slot"),
            Slot::Occupied { marked, .. } => {
                if !*marked {
                    panic!("Attempted to unmark an unmarked slot")
                } else {
                    *marked = false;
                }
            }
        }
    }

    pub(in crate::heap) fn is_marked(&self) -> bool {
        // SAFETY: slot is occupied
        match self {
            Slot::Free { .. } => panic!("Slot is not occupied"),
            Slot::Occupied { marked, .. } => *marked
        }
    }

    pub(in crate::heap) fn matches_handle(&self, handle: Handle) -> bool {
        self.generation() == handle.generation
    }
}