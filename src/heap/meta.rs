use std::ptr::NonNull;
use crate::heap::HeapObject;
use crate::heap::tracer::Tracer;

/// Type metadata for a heap-allocated object.
pub(crate) struct Descriptor {
    trace_fn: unsafe fn(NonNull<u8>, &mut Tracer),
    drop_fn: unsafe fn(NonNull<u8>),
}

impl Descriptor {
    pub(crate) const fn for_type<T: HeapObject>() -> &'static Self {
        unsafe fn trace_fn<T: HeapObject>(ptr: NonNull<u8>, tracer: &mut Tracer) {
            // SAFETY: caller guarantees `ptr` points to a valid,
            // initialized `GcBox<T>`
            let gc_box = unsafe { ptr.cast::<GcBox<T>>().as_ref() };
            gc_box.object.trace(tracer);
        }

        unsafe fn drop_fn<T: HeapObject>(ptr: NonNull<u8>) {
            let ptr = ptr.cast::<GcBox<T>>();

            // SAFETY: caller guarantees `ptr` points to a valid,
            // initialized `GcBox<T>`
            let _ = unsafe { Box::from_raw(ptr.as_ptr()) };
        }

        const {
            &Self {
                trace_fn: trace_fn::<T>,
                drop_fn: drop_fn::<T>
            }
        }
    }

    /// Traces all heap references held by the object at `ptr`.
    ///
    /// # Safety
    /// `ptr` must point to a valid, initialized [`GcBox<T>`] that this descriptor
    /// was created for.
    pub(crate) unsafe fn trace(&self, ptr: NonNull<u8>, tracer: &mut Tracer) {
        // SAFETY: caller guarantees `ptr` points to a valid, initialized
        // `T` that this descriptor was created for
        unsafe { (self.trace_fn)(ptr, tracer) }
    }

    /// Runs the destructor for the object at `ptr`.
    ///
    /// # Safety
    /// `ptr` must point to a valid, initialized [`GcBox<T>`] that this descriptor
    /// was created for, and the object must not have been previously
    /// dropped.
    pub(crate) unsafe fn drop(&self, ptr: NonNull<u8>) {
        // SAFETY: caller guarantees `ptr` points to a valid, initialized
        // `T` that this descriptor was created for, and it has not
        // been previously dropped
        unsafe { (self.drop_fn)(ptr) }
    }
}

/// GC bookkeeping data prepended to every heap allocation.
pub(crate) struct Header {
    is_marked: bool,
    descriptor: &'static Descriptor,
}

impl Header {
    pub(crate) fn new(descriptor: &'static Descriptor) -> Self {
        Self { is_marked: false, descriptor }
    }

    pub(crate) fn mark(&mut self) {
        self.is_marked = true;
    }

    pub(crate) fn unmark(&mut self) {
        self.is_marked = false;
    }

    pub(crate) fn is_marked(&self) -> bool {
        self.is_marked
    }

    pub(crate) fn descriptor(&self) -> &'static Descriptor {
        self.descriptor
    }
}

/// A heap-allocated object with GC bookkeeping data.
/// 
/// # Notes
/// - Stores a [`Header`] followed by the object.
/// - `#[repr(C)]` ensures that the header is always at
///   offset 0, allowing the GC to read it from a type-erased pointer.
#[repr(C)] // allows reading of `Header` without `T`
pub(crate) struct GcBox<T: HeapObject> {
    header: Header,
    object: T,
}

impl<T: HeapObject> GcBox<T> {
    pub(crate) fn new(object: T) -> Self {
        let descriptor = Descriptor::for_type::<T>();
        let header = Header::new(descriptor);
        Self { header, object }
    }

    pub(crate) fn header(&self) -> &Header {
        &self.header
    }

    pub(crate) fn header_mut(&mut self) -> &mut Header  {
        &mut self.header
    }

    pub(crate) fn object(&self) -> &T {
        &self.object
    }

    pub(crate) fn object_mut(&mut self) -> &mut T {
        &mut self.object
    }
}