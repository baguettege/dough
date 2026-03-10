use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Span {
    start: usize,
    end: usize
}

impl Span {
    pub(crate) fn merge(a: Span, b: Span) -> Self {
        Self {
            start: a.start.min(b.start),
            end: a.end.max(b.end)
        }
    }

    pub(crate) fn new(start: usize, end: usize) -> Self {
        Self {
            start: start.min(end),
            end: start.max(end)
        }
    }

    pub(crate) fn start(&self) -> usize {
        self.start
    }

    pub(crate) fn end(&self) -> usize {
        self.end
    }

    pub(crate) fn merge_with(&self, other: Span) -> Self {
        Self::merge(*self, other)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

pub(crate) trait Spanned {
    fn span(&self) -> Span;
}