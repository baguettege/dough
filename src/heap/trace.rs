use crate::heap::handle::Handle;

pub(crate) trait GcTrace {
    fn references(&self) -> Vec<Handle>;
}