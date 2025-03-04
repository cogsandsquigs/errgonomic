mod underlying;

use super::span::Span;

pub(crate) use underlying::Underlying;

/// The input to the parser. Note that `Input` *never* actually deletes/shrinks the input, it only
/// just shrinks the *span* that it covers.
#[derive(Debug, Clone)]
pub struct Input<I: Underlying> {
    /// The underlying iterator input.
    underlying: I,

    /// The saved buffer of input
    seen: Vec<I::Item>,

    /// The pointer of the next input to return on `next`. If it's greater than `span.len()`,
    /// that means we need to call `next`.
    seen_ptr: usize,

    /// The span that covers the input.
    span: Span,
}

impl<I: Underlying> Input<I> {
    pub fn new<N: Into<I>>(input: N) -> Input<I> {
        Self {
            underlying: input.into(),
            seen: Vec::new(),
            seen_ptr: 0,
            span: Span::new(0, 0),
        }
    }

    pub fn new_with_span<N: Into<I>>(input: N, span: Span) -> Input<I> {
        Self {
            underlying: input.into(),
            seen: Vec::new(),
            seen_ptr: 0,
            span,
        }
    }

    /// Consumes a single `Item` of the input and returns it.
    pub fn next(&mut self) -> Option<I::Item> {
        if self.seen_ptr < self.seen.len() {
            self.seen_ptr += 1;
            Some(self.seen[self.seen_ptr - 1])
        } else {
            let n = self.underlying.next()?;
            self.seen.push(n);
            self.seen_ptr += 1;
            Some(n)
        }
    }

    /// Peek at the next `Item` of the input without consuming it.
    pub fn peek(&mut self) -> Option<&I::Item> {
        if self.seen_ptr < self.seen.len() {
            Some(&self.seen[self.seen_ptr])
        } else {
            let n = self.underlying.next()?;
            self.seen.push(n);
            Some(&n)
        }
    }
}

impl<I: Underlying> From<I> for Input<I> {
    fn from(input: I) -> Self {
        Self {
            underlying: input,
            seen: Vec::new(),
            seen_ptr: 0,
            span: Span::new(0, 0),
        }
    }
}

/*
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
    pub fn slice(self, span: Span) -> Self {
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
    pub fn take(self, n: usize) -> Self {
        let span = self.span;
        self.slice(Span::new(span.head, cmp::min(span.head + n, span.tail)))
    }

    /// Skips the first `n` glyphs/chars from the input. If `n` is greater than the length of the
    /// input, it will simply return an empty input.
    pub fn skip(self, n: usize) -> Self {
        let span = self.span;
        self.slice(Span::new(cmp::min(span.head + n, span.tail), span.tail))
    }

    /// Checks if the input itself is a string of digits or not.
    pub fn is_decimal(&self) -> bool {
        self.underlying.slice(self.span).is_decimal()
    }

    /// Checks if the input is a hex
    pub fn is_hex(&self) -> bool {
        self.underlying.slice(self.span).is_hex()
    }

    /// Checks if the input is whitespace, but *not* newlines.
    pub fn is_whitespace_not_newline(&self) -> bool {
        self.underlying.slice(self.span).is_whitespace_not_newline()
    }

    /// Checks if the input is newlines, but *not* whitespace.
    pub fn is_newline(&self) -> bool {
        self.underlying.slice(self.span).is_newline()
    }

    /// Checks if the input is whitespace, including newlines.
    pub fn is_whitespace(&self) -> bool {
        self.underlying.slice(self.span).is_whitespace()
    }

    /// Returns the inner I.
    pub fn as_inner(&self) -> I {
        self.underlying.slice(self.span)
    }

    /// Intersects this input with another input. If the two inputs don't intersect, it will return
    /// an empty input.
    pub fn intersect(self, other: &Self) -> Self {
        let head = cmp::max(self.span.head, other.span.head);
        let tail = cmp::min(self.span.tail, other.span.tail);

        if head >= tail {
            return Self {
                underlying: self.underlying.fork(),
                span: Span::new(0, 0),
            };
        }

        Self {
            underlying: self.underlying.fork(),
            span: Span::new(head, tail),
        }
    }

    /// Subtracts another input from this input. If the two inputs don't intersect, it will return
    /// the original input.
    pub fn subtract(self, other: &Self) -> Self {
        let head = cmp::max(self.span.head, other.span.head);
        let tail = cmp::min(self.span.tail, other.span.tail);

        if head >= tail {
            return self.clone(); // No overlap, return original
        }

        // If the intersection is at the start or end of the span, adjust the spans accordingly.
        if self.span.head < head {
            return Self {
                underlying: self.underlying.fork(),
                span: Span::new(self.span.head, head),
            };
        }

        if self.span.tail > tail {
            return Self {
                underlying: self.underlying.fork(),
                span: Span::new(tail, self.span.tail),
            };
        }

        // If the subtraction is not simple (removing the intersection in the middle),
        // it may require splitting or complex handling, which you can add based on your use case.
        // For now, we can return the original input, assuming simple subtraction.
        self
    }
}

impl<'a, I: Underlying + 'a> Input<I> {
    /// Returns a reference to the inner I.
    pub fn as_inner_ref(&'a self) -> &'a I {
        &self.underlying
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
*/
