use std::ptr::NonNull;
use crate::handle::Handle;
use crate::{gc, Object};
use crate::gc::{GcBox, Root};

#[derive(Default)]
pub struct Heap {
    roots: Vec<Root>,
}

impl Heap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc<T: Object>(&mut self, object: T) -> Handle<T> {
        let gc_box = GcBox::new(object);
        let ptr = Box::into_raw(Box::new(gc_box));
        let ptr = NonNull::new(ptr).unwrap();

        self.roots.push(ptr.cast());
        // SAFETY: `ptr` points to a freshly allocated `GcBox<T>`
        unsafe { Handle::new(ptr) }
    }

    pub fn with<T, R, F>(&mut self, handle: &Handle<T>, f: F) -> R
    where
        T: Object,
        F: FnOnce(&mut T) -> R,
    {
        let mut ptr = handle.ptr();
        assert!(self.roots.contains(&ptr.cast()), "stale handle");

        // SAFETY: `handle.ptr()` is a valid ptr to a valid `GcBox<T>` managed by the heap
        let object = unsafe { ptr.as_mut().object_mut() };
        f(object)
    }

    pub fn gc(&mut self) {
        gc::run(self);
    }
}

impl Heap {
    pub(crate) fn roots(&self) -> &[Root] {
        &self.roots
    }

    pub(crate) fn roots_mut(&mut self) -> &mut Vec<Root> {
        &mut self.roots
    }
}
