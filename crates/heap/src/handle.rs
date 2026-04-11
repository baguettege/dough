use std::ptr::NonNull;
use crate::gc::GcBox;
use crate::Object;

pub struct Handle<T: Object>(NonNull<GcBox<T>>);

impl<T: Object> Handle<T> {
    pub(crate) unsafe fn new(ptr: NonNull<GcBox<T>>) -> Self {
        // SAFETY:
        // - `ptr` points to a valid, initialized, and properly aligned `T`.
        // - `ptr` must remain valid for the entire lifetime of this `Handle`.
        Self(ptr)
    }

    pub(crate) fn ptr(&self) -> NonNull<GcBox<T>> {
        self.0
    }
}
