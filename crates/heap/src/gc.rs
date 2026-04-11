mod meta;
mod tracer;

pub(crate) use meta::GcBox;
pub use tracer::{Tracer, Trace};

use std::ptr::NonNull;
use crate::gc::meta::Header;
use crate::Heap;

// must point to a `GcBox<T>` where `T: Object`
pub(crate) type Root = NonNull<u8>;

struct Gc<'a> {
    heap: &'a mut Heap,
}

impl<'a> Gc<'a> {
    fn new(heap: &'a mut Heap) -> Self {
        Self { heap }
    }

    fn run(mut self) {
        self.mark();
        self.sweep();
    }

    fn mark(&self) {
        let mut tracer = Tracer::new();

        // SAFETY: roots are valid ptrs to live heap allocation managed by this heap
        for root in self.heap.roots() {
            unsafe { tracer.push_root(*root) }
        }

        tracer.run();
    }

    fn sweep(&mut self) {
        // each `root` is a valid ptr to a liv heap allocation managed by this heap
        self.heap.roots_mut().retain(|&root| {
            let header = unsafe { root.cast::<Header>().as_mut() };

            if !header.is_marked() {
                unsafe { header.descriptor().drop(root) }
                false
            } else {
                header.unmark();
                true
            }
        });
    }
}

pub(crate) fn run(heap: &mut Heap) {
    Gc::new(heap).run();
}
