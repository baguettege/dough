use crate::gc::Root;
use crate::gc::tracer::Tracer;
use crate::Object;

pub(crate) struct Descriptor {
    trace_fn: unsafe fn(Root, &mut Tracer),
    drop_fn: unsafe fn(Root),
}

impl Descriptor {
    pub(crate) const fn for_type<T: Object>() -> &'static Descriptor {
        unsafe fn trace_fn<T: Object>(root: Root, tracer: &mut Tracer) {
            // SAFETY: `root` points to a valid, initialized, properly aligned `GcBox<T>`
            let gc_box = unsafe { root.cast::<GcBox<T>>().as_ref() };
            gc_box.object.trace(tracer);
        }

        unsafe fn drop_fn<T: Object>(root: Root) {
            let ptr = root.cast::<GcBox<T>>();
            // SAFETY:
            // - `root` points to a valid, initialized, properly aligned `GcBox<T>`
            // - this method has not been called previously on `root`
            let _ = unsafe { Box::from_raw(ptr.as_ptr()) };
        }

        const {
            &Self {
                trace_fn: trace_fn::<T>,
                drop_fn: drop_fn::<T>
            }
        }
    }

    pub(crate) unsafe fn trace(&self, root: Root, tracer: &mut Tracer) {
        // SAFETY: same as `trace_fn` in `Self::for_type`
        unsafe { (self.trace_fn)(root, tracer) }
    }

    pub(crate) unsafe fn drop(&self, root: Root) {
        // SAFETY: same as `drop_fn` in `Self::for_type`
        unsafe { (self.drop_fn)(root) }
    }
}

pub(crate) struct Header {
    is_marked: bool,
    descriptor: &'static Descriptor,
}

impl Header {
    pub(crate) fn new(descriptor: &'static Descriptor) -> Self {
        Self { is_marked: false, descriptor }
    }

    pub(crate) fn is_marked(&self) -> bool {
        self.is_marked
    }

    pub(crate) fn descriptor(&self) -> &'static Descriptor {
        self.descriptor
    }

    pub(crate) fn mark(&mut self) {
        self.is_marked = true;
    }

    pub(crate) fn unmark(&mut self) {
        self.is_marked = false;
    }
}

#[repr(C)]
pub(crate) struct GcBox<T: Object> {
    header: Header,
    object: T,
}

impl<T: Object> GcBox<T> {
    pub(crate) fn new(object: T) -> Self {
        let header = Header::new(Descriptor::for_type::<T>());
        Self { header, object }
    }

    pub(crate) fn object_mut(&mut self) -> &mut T {
        &mut self.object
    }
}
