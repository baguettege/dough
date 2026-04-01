/// A range of byte offsets in source text.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Creates a new span from `start` to `end`.
    ///
    /// # Panics
    /// If `start > end`.
    pub(crate) fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "start must be <= end");
        Self { start, end }
    }

    pub(crate) fn start(&self) -> usize {
        self.start
    }

    pub(crate) fn end(&self) -> usize {
        self.end
    }

    /// Extends this span to also cover `other`.
    pub(crate) fn merge(self, other: Self) -> Self {
        Self::new(
            self.start.min(other.start),
            self.end.max(other.end)
        )
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Implemented by types that originated from source text.
pub(crate) trait Spanned {
    fn span(&self) -> Span;
}
