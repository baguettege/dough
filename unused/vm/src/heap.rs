pub(crate) use handle::Handle;
pub(crate) use tracer::{Trace, Tracer};

use crate::heap::gc::Gc;
use crate::heap::meta::GcBox;
use std::ptr::NonNull;

mod gc;
mod handle;
mod meta;
mod tracer;

/// A type that can be allocated on the heap.
pub(crate) trait HeapObject: Sized + Trace {}

type Root = NonNull<u8>;

/// A garbage collected heap.
#[derive(Default)]
pub(crate) struct Heap {
    roots: Vec<Root>,
}

impl Heap {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Allocates `object` on the heap, returning a handle to it.
    pub(crate) fn alloc<T: HeapObject>(&mut self, object: T) -> Handle<T> {
        let gc_box = GcBox::new(object);
        let leaked = Box::leak(Box::new(gc_box));
        let ptr = NonNull::new(leaked).unwrap();

        self.roots.push(ptr.cast());

        // SAFETY: `ptr` points to a valid, initialized `GcBox<T>`,
        // guaranteed above
        unsafe { Handle::new(ptr) }
    }

    /// Returns a reference to the object referenced by `handle`.
    ///
    /// # Panics
    /// If the handle is stale.
    pub(crate) fn get<T: HeapObject>(&self, handle: &Handle<T>) -> &T {
        let ptr = handle.ptr();
        assert!(self.roots.contains(&ptr.cast()), "stale handle");

        // SAFETY: `ptr` is valid and initialized as it exists in the root set,
        // guaranteeing that the GC has not collected it
        let gc_box = unsafe { ptr.as_ref() };

        gc_box.object()
    }

    /// Returns a mutable reference to the object referenced by `handle`.
    ///
    /// # Panics
    /// If the handle is stale.
    pub(crate) fn get_mut<T: HeapObject>(&mut self, handle: &mut Handle<T>) -> &mut T {
        let mut ptr = handle.ptr();
        assert!(self.roots.contains(&ptr.cast()), "stale handle");

        // SAFETY: `ptr` is valid and initialized as it exists in the root set,
        // guaranteeing that the GC has not collected it
        let gc_box = unsafe { ptr.as_mut() };

        gc_box.object_mut()
    }

    /// Runs a garbage collection cycle on this heap.
    pub(crate) fn gc(&mut self) {
        Gc::collect(self);
    }
}
