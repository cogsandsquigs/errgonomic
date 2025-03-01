use super::input::ParseInput;
use core::marker::PhantomData;

/// A span represents a range of input, from the head to the tail.
/// Note that this can ONLY be used for a certain input type, due to differences between
/// byte-slicing and string-slicing (over glyphs).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span<I: ParseInput> {
    _underlying: PhantomData<I>,
    head: usize,
    tail: usize,
}

impl<I: ParseInput> Span<I> {
    pub fn new(head: usize, tail: usize) -> Self {
        Span {
            _underlying: PhantomData,
            head,
            tail,
        }
    }

    pub fn len(&self) -> usize {
        self.tail - self.head
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the head of the span.
    pub fn head(&self) -> usize {
        self.head
    }

    /// Gets the tail of the span.
    /// NOTE: This is exclusive, so the last element is `tail - 1`.
    pub fn tail(&self) -> usize {
        self.tail
    }
}
