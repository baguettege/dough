use crate::heap::HeapObject;
use crate::heap::meta::GcBox;
use std::ptr::NonNull;

/// A typed handle to a value allocated on the heap.
pub(crate) struct Handle<T: HeapObject>(NonNull<GcBox<T>>);

impl<T: HeapObject> Handle<T> {
    /// Creates a new handle from a raw pointer.
    ///
    /// # Safety
    /// `ptr` must point to a valid, initialized [`GcBox<T>`].
    pub(crate) unsafe fn new(ptr: NonNull<GcBox<T>>) -> Self {
        Self(ptr)
    }

    pub(crate) fn ptr(&self) -> NonNull<GcBox<T>> {
        self.0
    }
}
