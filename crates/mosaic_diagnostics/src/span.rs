//! Implements [`Span`] and [`SourceId`]

use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

/// Identifies a source file.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SourceId(pub usize);

/// A byte-offset range within a source file.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub source: SourceId,
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Creates a new span covering the byte range `start..end` in the given source file.
    pub const fn new(source: SourceId, start: usize, end: usize) -> Self {
        Self { source, start, end }
    }

    /// Returns the length of this span in bytes.
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns `true` if this span covers zero bytes.
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Creates a new span that covers both `self` and `other`.
    /// Panics if the two spans come from different source files.
    pub fn merge(self, other: Self) -> Self {
        debug_assert_eq!(
            self.source, other.source,
            "cannot merge spans from different sources"
        );
        Self {
            source: self.source,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Returns a zero-width span at the start of this span.
    pub const fn start_point(&self) -> Self {
        Self {
            source: self.source,
            start: self.start,
            end: self.start,
        }
    }

    /// Returns a zero-width span at the end of this span.
    pub const fn end_point(&self) -> Self {
        Self {
            source: self.source,
            start: self.end,
            end: self.end,
        }
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({}, {}..{})", self.source.0, self.start, self.end)
    }
}

/// Wraps a value with its source location.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub const fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }

    /// Maps the inner value while preserving the span.
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            node: f(self.node),
            span: self.span,
        }
    }

    /// Returns a reference to the inner value with the same span.
    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned {
            node: &self.node,
            span: self.span,
        }
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.node
    }
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} @ {:?}", self.node, self.span)
    }
}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.node.fmt(f)
    }
}
