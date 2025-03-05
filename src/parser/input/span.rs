use std::ops::{Range, RangeInclusive};

/// A span of input. Represents an *exclusive range* of text/input.
/// WARN: Due to the way different inputs may slice themselves, *spans should not be used with
/// inputs they were not generated from!*
/// TODO: Add a trait bound to ensure that the input is the same (probably via `PhantomData`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// The head of the span. This includes the starting character/"glyph".
    head: usize,

    /// The tail of the span. It is *exclusive* of the end, and so the last character this span
    /// represents is `tail - 1`.
    tail: usize,
}

impl Span {
    /// Creates a new span from a head and a tail.
    pub fn new(head: usize, tail: usize) -> Self {
        Span { head, tail }
    }

    /// Gets the length of the span.
    pub fn len(&self) -> usize {
        self.tail.saturating_sub(self.head)
    }

    /// Checks if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the current head of the span.
    pub fn head(&self) -> usize {
        self.head
    }

    /// Gets the current tail of the span.
    pub fn tail(&self) -> usize {
        self.tail
    }

    /// Increments the head of the span by `n`. Note that if the `head` is incremented beyond the
    /// tail, we stop incrementing. Returns the *old* head.
    pub fn increment_head(&mut self, n: usize) -> usize {
        let old_head = self.head;
        self.head = core::cmp::min(self.head + n, self.tail);
        old_head
    }

    /// Takes a span from `head` to `head + n`, inclusive. Note that if `n` is greater than or equal
    /// to the length of the tail, it will simply return a span from `head` to `tail`.
    pub fn take(&self, n: usize) -> Span {
        Span {
            head: self.head,
            tail: core::cmp::min(self.head + n, self.tail),
        }
    }

    /// Returns a span from `head + n` to `tail`, exclusive of tail. Note that if `n` is greater
    /// than or equal to the length of the tail, it will simply return an empty span.
    pub fn skip(&self, n: usize) -> Span {
        Span {
            head: core::cmp::min(self.head + n, self.tail),
            tail: self.tail,
        }
    }

    /// Checks if two spans overlap.
    pub fn is_overlapping(&self, other: Span) -> bool {
        self.head <= other.tail && other.head <= self.tail
    }

    /// Intersects two spans. Requires that the spans overlap. Otherwise, will panic.
    pub fn intersect(&self, other: Span) -> Span {
        assert!(self.is_overlapping(other), "Spans do not overlap!");

        let head = self.head.max(other.head);
        let tail = self.tail.min(other.tail);

        Span { head, tail }
    }

    /// Unions two spans. Requires that the spans are adjacent or overlapping. Otherwise, will panic.
    pub fn union(&self, other: Span) -> Span {
        assert!(self.is_overlapping(other), "Spans are not overlapping!");

        let head = self.head.min(other.head);
        let tail = self.tail.max(other.tail);

        Span { head, tail }
    }

    /// Unions two spans, such that any gaps between the spans are removed.
    pub fn union_between(&self, other: Span) -> Span {
        if self.is_overlapping(other) {
            self.union(other)
        } else {
            let head = self.head.min(other.head);
            let tail = self.tail.max(other.tail);

            Span { head, tail }
        }
    }

    /// Subtracts a span from another span. Requires that the spans overlap. Otherwise, will panic.
    pub fn subtract(&self, other: Span) -> Span {
        assert!(self.is_overlapping(other), "Spans do not overlap!");

        if self.head >= other.head && self.tail <= other.tail {
            Span {
                head: self.head,
                tail: self.head,
            }
        } else if self.head < other.head {
            Span {
                head: self.head,
                tail: self.tail.min(other.head),
            }
        } else {
            Span {
                head: self.head.max(other.tail),
                tail: self.tail,
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let span = Span::new(5, 10);
        assert_eq!(span.head, 5);
        assert_eq!(span.tail, 10);
    }

    #[test]
    fn test_len() {
        let span = Span::new(5, 10);
        assert_eq!(span.len(), 5);

        let empty_span = Span::new(7, 7);
        assert_eq!(empty_span.len(), 0);
    }

    #[test]
    fn test_is_empty() {
        let span = Span::new(5, 10);
        assert!(!span.is_empty());

        let empty_span = Span::new(7, 7);
        assert!(empty_span.is_empty());
    }

    #[test]
    fn test_head_tail_getters() {
        let span = Span::new(5, 10);
        assert_eq!(span.head(), 5);
        assert_eq!(span.tail(), 10);
    }

    #[test]
    fn test_increment_head() {
        let mut span = Span::new(5, 10);

        // Normal increment
        let old_head = span.increment_head(2);
        assert_eq!(old_head, 5);
        assert_eq!(span.head(), 7);
        assert_eq!(span.tail(), 10);

        // Increment by 0
        let old_head = span.increment_head(0);
        assert_eq!(old_head, 7);
        assert_eq!(span.head(), 7);

        // Increment past tail
        let old_head = span.increment_head(5);
        assert_eq!(old_head, 7);
        assert_eq!(span.head(), 10);
        assert_eq!(span.tail(), 10);
        assert!(span.is_empty());

        // Increment when already at tail
        let old_head = span.increment_head(1);
        assert_eq!(old_head, 10);
        assert_eq!(span.head(), 10);
        assert!(span.is_empty());
    }

    #[test]
    fn test_take() {
        let span = Span::new(5, 10);

        // Normal take
        let taken = span.take(3);
        assert_eq!(taken.head(), 5);
        assert_eq!(taken.tail(), 8);
        assert_eq!(taken.len(), 3);

        // Take 0
        let taken = span.take(0);
        assert_eq!(taken.head(), 5);
        assert_eq!(taken.tail(), 5);
        assert_eq!(taken.len(), 0);
        assert!(taken.is_empty());

        // Take more than available
        let taken = span.take(10);
        assert_eq!(taken.head(), 5);
        assert_eq!(taken.tail(), 10);
        assert_eq!(taken.len(), 5);

        // Take from empty span
        let empty_span = Span::new(7, 7);
        let taken = empty_span.take(3);
        assert_eq!(taken.head(), 7);
        assert_eq!(taken.tail(), 7);
        assert_eq!(taken.len(), 0);
        assert!(taken.is_empty());
    }

    #[test]
    fn test_skip() {
        let span = Span::new(5, 10);

        // Normal skip
        let skipped = span.skip(2);
        assert_eq!(skipped.head(), 7);
        assert_eq!(skipped.tail(), 10);
        assert_eq!(skipped.len(), 3);

        // Skip 0
        let skipped = span.skip(0);
        assert_eq!(skipped.head(), 5);
        assert_eq!(skipped.tail(), 10);
        assert_eq!(skipped.len(), 5);

        // Skip more than available
        let skipped = span.skip(10);
        assert_eq!(skipped.head(), 10);
        assert_eq!(skipped.tail(), 10);
        assert_eq!(skipped.len(), 0);
        assert!(skipped.is_empty());

        // Skip from empty span
        let empty_span = Span::new(7, 7);
        let skipped = empty_span.skip(3);
        assert_eq!(skipped.head(), 7);
        assert_eq!(skipped.tail(), 7);
        assert_eq!(skipped.len(), 0);
        assert!(skipped.is_empty());
    }

    #[test]
    fn test_chain_operations() {
        let span = Span::new(0, 10);

        // Take followed by take
        let taken1 = span.take(8);
        let taken2 = taken1.take(3);
        assert_eq!(taken2.head(), 0);
        assert_eq!(taken2.tail(), 3);

        // Skip followed by skip
        let skipped1 = span.skip(2);
        let skipped2 = skipped1.skip(3);
        assert_eq!(skipped2.head(), 5);
        assert_eq!(skipped2.tail(), 10);

        // Skip followed by take
        let skipped = span.skip(2);
        let taken = skipped.take(3);
        assert_eq!(taken.head(), 2);
        assert_eq!(taken.tail(), 5);

        // Take followed by skip
        let taken = span.take(8);
        let skipped = taken.skip(3);
        assert_eq!(skipped.head(), 3);
        assert_eq!(skipped.tail(), 8);
    }

    #[test]
    fn test_edge_cases() {
        // Create span with head > tail (should be avoided but test behavior)
        let invalid_span = Span::new(10, 5);
        assert_eq!(invalid_span.len(), 0); // Should handle this gracefully

        // Test with maximum possible values
        let max_span = Span::new(usize::MAX - 10, usize::MAX);
        assert_eq!(max_span.len(), 10);

        // Incrementing head near maximum
        let mut big_span = Span::new(usize::MAX - 10, usize::MAX);
        let old_head = big_span.increment_head(5);
        assert_eq!(old_head, usize::MAX - 10);
        assert_eq!(big_span.head(), usize::MAX - 5);
    }

    #[test]
    fn is_correctly_overlapping() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(3, 7);
        assert!(span1.is_overlapping(span2));
        assert!(span2.is_overlapping(span1));

        let span1 = Span::new(0, 5);
        let span2 = Span::new(5, 7);
        assert!(span1.is_overlapping(span2));
        assert!(span2.is_overlapping(span1));

        let span1 = Span::new(0, 5);
        let span2 = Span::new(6, 7);
        assert!(!span1.is_overlapping(span2));
        assert!(!span2.is_overlapping(span1));
    }

    #[test]
    fn is_correctly_intersecting() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(3, 7);
        assert_eq!(span1.intersect(span2), Span::new(3, 5));
        assert_eq!(span2.intersect(span1), Span::new(3, 5));

        let span1 = Span::new(0, 5);
        let span2 = Span::new(5, 7);
        assert_eq!(span1.intersect(span2), Span::new(5, 5));
        assert_eq!(span2.intersect(span1), Span::new(5, 5));
    }

    #[test]
    #[should_panic]
    fn is_panicking_on_non_overlapping_intersect() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(6, 7);
        span1.intersect(span2);
    }

    #[test]
    fn is_correctly_unioning() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(3, 7);
        assert_eq!(span1.union(span2), Span::new(0, 7));
        assert_eq!(span2.union(span1), Span::new(0, 7));

        let span1 = Span::new(0, 5);
        let span2 = Span::new(5, 7);
        assert_eq!(span1.union(span2), Span::new(0, 7));
        assert_eq!(span2.union(span1), Span::new(0, 7));
    }

    #[test]
    #[should_panic]
    fn is_panicking_on_non_overlapping_union() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(6, 7);
        span1.union(span2);
    }

    #[test]
    fn is_correctly_subtracting() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(3, 7);
        assert_eq!(span1.subtract(span2), Span::new(0, 3));
        assert_eq!(span2.subtract(span1), Span::new(5, 7));

        let span1 = Span::new(0, 5);
        let span2 = Span::new(5, 7);
        assert_eq!(span1.subtract(span2), Span::new(0, 5));
        assert_eq!(span2.subtract(span1), Span::new(5, 7));

        let span1 = Span::new(0, 5);
        let span2 = Span::new(0, 5);
        assert_eq!(span1.subtract(span2), Span::new(0, 0));
        assert_eq!(span2.subtract(span1), Span::new(0, 0));
    }

    #[test]
    #[should_panic]
    fn is_panicking_on_non_overlapping_subtract() {
        let span1 = Span::new(0, 5);
        let span2 = Span::new(6, 7);
        span1.subtract(span2);
    }

    #[test]
    fn test_union_between() {
        // Overlapping spans
        let span1 = Span::new(0, 5);
        let span2 = Span::new(3, 8);
        let union = span1.union_between(span2);
        assert_eq!(union, Span::new(0, 8));

        // Completely overlapping spans
        let span1 = Span::new(0, 10);
        let span2 = Span::new(2, 5);
        let union = span1.union_between(span2);
        assert_eq!(union, Span::new(0, 10));

        // Spans with small gap
        let span1 = Span::new(0, 5);
        let span2 = Span::new(6, 10);
        let union = span1.union_between(span2);
        assert_eq!(union, Span::new(0, 10));

        // Spans with large gap
        let span1 = Span::new(0, 5);
        let span2 = Span::new(10, 15);
        let union = span1.union_between(span2);
        assert_eq!(union, Span::new(0, 15));

        // One span completely before another
        let span1 = Span::new(0, 5);
        let span2 = Span::new(5, 10);
        let union = span1.union_between(span2);
        assert_eq!(union, Span::new(0, 10));

        // Spans in reverse order
        let span1 = Span::new(10, 15);
        let span2 = Span::new(0, 5);
        let union = span1.union_between(span2);
        assert_eq!(union, Span::new(0, 15));

        // Equal spans
        let span1 = Span::new(5, 10);
        let span2 = Span::new(5, 10);
        let union = span1.union_between(span2);
        assert_eq!(union, Span::new(5, 10));

        // One empty span
        let span1 = Span::new(0, 5);
        let span2 = Span::new(5, 5);
        let union = span1.union_between(span2);
        assert_eq!(union, Span::new(0, 5));
    }
}
