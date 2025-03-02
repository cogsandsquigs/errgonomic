mod underlying;

use super::span::Span;
use core::{cmp, fmt};
use std::fmt::Display;

pub(crate) use underlying::Underlying;

/// The input to the parser. Note that `Input` *never* actually deletes/shrinks the input, it only
/// just shrinks the *span* that it covers.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Input<I: Underlying> {
    /// The underlying input.
    underlying: I,

    /// The span that covers the input.
    span: Span,
}

impl<I: Underlying> Input<I> {
    /// Create a new `Input` object.
    pub fn new(underlying: I) -> Self {
        let span = Span::new(0, underlying.len());
        Self { underlying, span }
    }

    /// Create a new `Input` object with a specific span.
    pub fn new_with_span(underlying: I, span: Span) -> Self {
        Self { underlying, span }
    }

    /// Get the length of the input.
    pub fn len(&self) -> usize {
        self.span.len()
    }

    /// Check if the input is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Fork the input. Essentially, a transparent `Clone`. For types that are references, it
    /// should simply clone the reference. For types that are owned, it should clone the owned
    /// object.
    pub fn fork(&self) -> Self {
        Self {
            underlying: self.underlying.fork(),
            span: self.span,
        }
    }

    /// Slice the input. Panics if the slice goes beyond the current input.
    pub fn slice(&self, span: Span) -> Self {
        if self.span.head > span.head || self.span.tail < span.tail {
            panic!("Attempted to slice input beyond current input.");
        }

        Self {
            underlying: self.underlying.fork(),
            span,
        }
    }

    /// Takes just the first `n` glyphs/chars from the input. If `n` is greater than the length of
    /// the input, it will simply return all of the input.
    pub fn take(&self, n: usize) -> Self {
        self.slice(Span::new(
            self.span.head,
            cmp::min(self.span.head + n, self.span.tail),
        ))
    }

    /// Skips the first `n` glyphs/chars from the input. If `n` is greater than the length of the
    /// input, it will simply return an empty input.
    pub fn skip(&self, n: usize) -> Self {
        self.slice(Span::new(
            cmp::min(self.span.head + n, self.span.tail),
            self.span.tail,
        ))
    }

    /// Returns the inner I.
    pub fn as_inner(&self) -> I {
        self.underlying.slice(self.span)
    }
}

impl<I: Underlying> PartialEq<I> for Input<I> {
    fn eq(&self, other: &I) -> bool {
        self.underlying.slice(self.span) == *other
    }
}

impl<I: Underlying + Display> fmt::Display for Input<I> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.underlying.slice(self.span))
    }
}

impl<I: Underlying> From<I> for Input<I> {
    fn from(underlying: I) -> Self {
        Self::new(underlying)
    }
}
