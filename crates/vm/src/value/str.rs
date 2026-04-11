use std::fmt;
use heap::{Trace, Tracer};

pub(crate) struct Str(String);

impl Str {
    pub(crate) fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl Trace for Str {
    fn trace(&self, _tracer: &mut Tracer) {}
}

impl heap::Object for Str {}

impl fmt::Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
