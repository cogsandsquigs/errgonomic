use crate::parser::span::Span;
use core::fmt::Debug;

/// Anything that implements `Underlying` can be used as an `Input` to the parser.
pub trait Underlying: PartialEq + Eq + Debug + Clone {
    /// Get the length of the input.
    fn len(&self) -> usize;

    /// Check if the input is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Slice the input.
    fn slice(&self, span: Span) -> Self;

    /// Fork the input. Essentially, a transparent `Clone`. For types that are references, it
    /// should simply clone the reference. For types that are owned, it should clone the owned
    /// object.
    fn fork(&self) -> Self;
}

impl Underlying for &str {
    fn len(&self) -> usize {
        (self as &str).len()
    }

    fn slice(&self, span: Span) -> Self {
        &self[span.head..span.tail]
    }

    fn fork(&self) -> Self {
        self
    }
}

impl Underlying for &[u8] {
    fn len(&self) -> usize {
        (self as &[u8]).len()
    }

    fn slice(&self, span: Span) -> Self {
        &self[span.head..span.tail]
    }

    fn fork(&self) -> Self {
        self
    }
}
