use std::ops::{Range, RangeInclusive};

/// A span of input. Represents an *exclusive range* of text/input.
/// WARN: Due to the way different inputs may slice themselves, *spans should not be used with
/// inputs they were not generated from!*
/// TODO: Add a trait bound to ensure that the input is the same (probably via `PhantomData`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The head of the span. This includes the starting character/"glyph".
    pub head: usize,

    /// The tail of the span. It is *exclusive* of the end, and so the last character this span
    /// represents is `tail - 1`.
    pub tail: usize,
}

impl Span {
    /// Creates a new span from a head and a tail.
    pub fn new(head: usize, tail: usize) -> Self {
        Span { head, tail }
    }

    /// Gets the length of the span.
    pub fn len(&self) -> usize {
        self.tail - self.head
    }

    /// Checks if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Span {
            head: range.start,
            tail: range.end,
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(val: Span) -> Self {
        val.head..val.tail
    }
}

impl From<RangeInclusive<usize>> for Span {
    fn from(range: RangeInclusive<usize>) -> Self {
        Span {
            head: *range.start(),
            tail: *range.end() + 1,
        }
    }
}
