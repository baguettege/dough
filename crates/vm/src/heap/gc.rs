use crate::heap::Heap;
use crate::heap::meta::Header;
use crate::heap::tracer::Tracer;

/// Garbage collector for a [`Heap`].
pub(crate) struct Gc<'a> {
    heap: &'a mut Heap,
}

impl<'a> Gc<'a> {
    /// Runs a full mark and sweep collection cycle on `heap`.
    pub(crate) fn collect(heap: &'a mut Heap) {
        Self::new(heap).collect_inner();
    }
    
    fn new(heap: &'a mut Heap) -> Self {
        Self { heap }
    }

    fn collect_inner(mut self) {
        self.mark();
        self.sweep();
    }
    
    /// Marks all objects reachable from the root set.
    fn mark(&self) {
        let mut tracer = Tracer::default();

        for ptr in self.heap.roots.iter() {
            // SAFETY: `ptr` points to a valid, initialized `GcBox<T>`
            // guaranteed by `Heap::alloc` which only inserts valid
            // pointers into `roots`
            unsafe { tracer.push_root(*ptr); }
        }

        tracer.run();
    }

    /// Drops all unmarked objects and removes them from the root set
    /// and unmarks all surviving objects.
    fn sweep(&mut self) {
        self.heap.roots.retain(|ptr| {
            // SAFETY: `ptr` points to a valid, initialized `GcBox<T>`
            // guaranteed by `Heap::alloc` which only inserts valid
            // pointers into `roots`
            let header = unsafe { ptr.cast::<Header>().as_mut() };

            if !header.is_marked() {
                unsafe { header.descriptor().drop(*ptr) }
                false
            } else {
                header.unmark();
                true
            }
        });
    }
}
