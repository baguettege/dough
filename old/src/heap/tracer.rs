use crate::heap::handle::Handle;
use crate::heap::meta::Header;
use crate::heap::{HeapObject, Root};

/// Worklist based tracer for the mark phase of the GC.
#[derive(Default)]
pub(crate) struct Tracer {
    worklist: Vec<Root>,
}

impl Tracer {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Pushes the object referenced by `handle` onto the worklist
    /// if it has not already been marked.
    pub(crate) fn push<T: HeapObject>(&mut self, handle: &Handle<T>) {
        let ptr = handle.ptr().cast::<u8>();
        // SAFETY: `ptr` points to a valid, initialized `GcBox<T>`
        // guaranteed by `handle.ptr()`
        unsafe { self.push_root(ptr) }
    }

    /// Pushes a type-erased pointer onto the worklist if it has
    /// not already been marked.
    ///
    /// # Safety
    /// `ptr` points to a valid, initialized [`GcBox<T>`] for some `T: HeapObject`.
    pub(crate) unsafe fn push_root(&mut self, ptr: Root) {
        // SAFETY: caller guarantees `ptr` points to a valid,
        // initialized `GcBox<T>` and `Header` is guaranteed
        // to be at offset 0 by `#[repr(C)]`
        let header = unsafe { ptr.cast::<Header>().as_mut() };

        if !header.is_marked() {
            header.mark();
            self.worklist.push(ptr.cast());
        }
    }

    /// Traces all enqueued objects until all reachable objects are marked.
    pub(crate) fn run(mut self) {
        while let Some(ptr) = self.worklist.pop() {
            unsafe {
                // SAFETY: `ptr` points to a valid, initialized `GcBox<T>`
                // and `Header` is guaranteed to be at offset 0 by `#[repr(C)]`
                let header = ptr.cast::<Header>().as_ref();

                // SAFETY: `ptr` points to a valid, initialized `GcBox<T>`
                // guaranteed by `push`, which only pushes valid `GcBox<T>`
                // pointers onto `worklist`
                header.descriptor().trace(ptr, &mut self)
            }
        }
    }
}

/// Implemented by heap objects to expose their heap references to the GC.
pub(crate) trait Trace {
    fn trace(&self, tracer: &mut Tracer);
}
