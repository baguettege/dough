use crate::heap::handle::Handle;
use crate::heap::trace::GcTrace;

#[derive(Debug)]
pub struct DoughStr(String);

impl DoughStr {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl GcTrace for DoughStr {
    fn references(&self) -> Vec<Handle> {
        Vec::new()
    }
}