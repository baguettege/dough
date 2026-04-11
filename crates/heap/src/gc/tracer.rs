use crate::gc::meta::Header;
use crate::gc::Root;
use crate::handle::Handle;
use crate::Object;

pub struct Tracer {
    worklist: Vec<Root>,
}

impl Tracer {
    pub(crate) fn new() -> Self {
        Self { worklist: Vec::new() }
    }

    pub(crate) unsafe fn push_root(&mut self, root: Root) {
        // SAFETY: `root` points to a valid `GcBox<T>` whose layout
        // begins with `Header`, guaranteed by `#[repr(C)]`
        let header = unsafe { root.cast::<Header>().as_mut() };

        if !header.is_marked() {
            header.mark();
            self.worklist.push(root);
        }
    }

    pub fn push<T: Object>(&mut self, handle: &Handle<T>) {
        let root: Root = handle.ptr().cast::<u8>();
        // SAFETY: `handle.ptr()` is a valid ptr to a live `GcBox<T>`,
        // which can be safely treated as a type-erased root
        unsafe { self.push_root(root) }
    }

    pub(crate) fn run(mut self) {
        while let Some(root) = self.worklist.pop() {
            unsafe {
                // SAFETY: `root` points to a valid `GcBox<T>` whose layout
                // begins with `Header`, guaranteed by `#[repr(C)]`
                let header = root.cast::<Header>().as_ref();
                // SAFETY: dispatches to the type-specific trace fn for this object
                header.descriptor().trace(root, &mut self);
            }
        }
    }
}

pub trait Trace {
    fn trace(&self, tracer: &mut Tracer);
}
