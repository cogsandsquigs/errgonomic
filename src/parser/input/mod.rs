mod span;
mod underlying;

pub use span::*;
pub use underlying::*;

/// The input to the parser. Note that `Input` *never* actually deletes/shrinks the input, it only
/// just shrinks the *span* that it covers.
#[derive(Debug, Clone, Eq)]
pub struct Input<I: Underlying> {
    /// The input to the parser.
    underlying: I,

    /// The range we are currently parsing.
    /// NOTE: The `head` of the span is the byte that we output at the next `.next` call.
    /// `tail` is exclusive of the end of the span.
    span: Span,
}

impl<I: Underlying> Input<I> {
    /// Create a new `Input` object.
    pub fn new(input: I) -> Self {
        Self {
            span: Span::new(0, input.len()),
            underlying: input,
        }
    }

    /// Create a new `Input` object with a specific span.
    pub fn new_with_span<S: Into<Span>>(input: I, span: S) -> Self {
        Self {
            underlying: input,
            span: span.into(),
        }
    }

    /// Gets the underlying input.
    pub fn as_inner(&self) -> I {
        self.underlying
            .span(self.span.head(), self.span.tail())
            .expect("the span to always cover a (sub)set of the underlying input")
    }

    /// Checks if the input is empty.
    pub fn is_empty(&self) -> bool {
        self.span.is_empty() // TODO: What if the underlying is a reader and the reader is empty?
                             // Or what if the reader *isn't* empty but the span is empty, and we
                             // need to keep increasing the span to keep up with the reader?
    }

    /// Consumes a single byte of the input and returns it.
    /// TODO: Make input an iterator? But would lead to a lot of things not being accessible, i.e.
    /// accessing input methods after a `take` would be impossible.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<u8> {
        let idx = self.span.increment_head(1);
        if idx >= self.span.tail() || idx >= self.underlying.len() {
            return None;
        }
        self.underlying.byte_at(idx)
    }

    /// Consumes a character from the input and returns it.
    /// NOTE: This may consume more than one byte!
    /// WARN: Will skip over invalid unicode!
    /// TODO: Make this faster?
    #[cfg(feature = "unicode")]
    pub fn next_char(&mut self) -> Option<char> {
        let mut unicode_bytes = vec![];
        loop {
            let c = self.next()?;
            unicode_bytes.push(c);

            return match simdutf8::basic::from_utf8(&unicode_bytes) {
                Ok(c) => c.chars().next(),
                Err(_) => continue,
            };
        }
    }

    /// Peeks at the next byte (the one that would be returned by `next`) of the input without
    /// consuming it.
    pub fn peek(&self) -> Option<u8> {
        self.underlying.byte_at(self.span.head())
    }

    /// Peeks at the next character (the one that would be returned by `next_char`) of the input
    /// without consuming it.
    /// WARN: Will skip over invalid unicode!
    /// TODO: Make this faster?
    #[cfg(feature = "unicode")]
    pub fn peek_char(&self) -> Option<char> {
        let mut unicode_bytes = vec![];
        loop {
            let c = self.peek_nth(unicode_bytes.len() + 1)?;
            unicode_bytes.push(c);

            return match simdutf8::basic::from_utf8(&unicode_bytes) {
                Ok(c) => c.chars().next(),
                Err(_) => continue,
            };
        }
    }

    /// peeks at the `n`th byte of the input from the current.
    /// NOTE: `peek_nth(0) == peek_nth(1) == peek_char()`
    pub fn peek_nth(&self, n: usize) -> Option<u8> {
        if n == 0 {
            self.peek()
        } else {
            self.underlying.byte_at(self.span.head() + n - 1)
        }
    }

    /// peeks at the `n`th char of the input from the current
    /// NOTE: `peek_nth_char(0) == peek_nth_char(1) == peek_char()`
    /// WARN: Will skip over invalid unicode!
    /// TODO: Make this faster?
    #[cfg(feature = "unicode")]
    pub fn peek_nth_char(&self, n: usize) -> Option<char> {
        if n == 0 {
            return self.peek_char();
        }

        let mut unicode_bytes_all = vec![];
        let mut total_bytes_taken = 0;

        for i in 0..n {
            unicode_bytes_all.push(vec![]);
            loop {
                let c = self.peek_nth(total_bytes_taken + 1)?;
                unicode_bytes_all[i].push(c);
                total_bytes_taken += 1;

                match simdutf8::basic::from_utf8(&unicode_bytes_all[i]) {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
        }

        simdutf8::basic::from_utf8(&unicode_bytes_all[n - 1])
            .expect("to be valid utf8")
            .chars()
            .next()
    }
    /// Take a string of `n` bytes from the current head (the index of the byte that would be
    /// returned at the next `.next` call) and returns them in the input. If `n` is greater
    /// than the length of the span, it will simply return an `Input` from the current head to
    /// the end of the span.
    pub fn take(&self, n: usize) -> Input<I> {
        Input::new_with_span(self.underlying.fork(), self.span.take(n))
    }

    /// Skip `n` bytes from the current head (the index of the byte that would be returned at the
    /// next `.next` call). If `n` is greater than the length of the span, it will simply return an
    /// `Input` from the end of the span to the end of the span.
    pub fn skip(&self, n: usize) -> Input<I> {
        Input::new_with_span(self.underlying.fork(), self.span.skip(n))
    }

    /// Skips all the way to the end of the input.
    pub fn skip_all(&self) -> Input<I> {
        Input::new_with_span(self.underlying.fork(), self.span.skip(self.span.len()))
    }

    /// Forks the input, creating two separate, independent inputs.
    pub fn fork(&self) -> Input<I> {
        Input::new_with_span(self.underlying.fork(), self.span)
    }

    /// Subtracts the span of `other` from `self` and returns the remaining input.
    pub fn subtract(&self, other: &Input<I>) -> Input<I> {
        Input::new_with_span(self.underlying.fork(), self.span.subtract(other.span))
    }

    /// Joins two inputs together. Requires that the two inputs are contiguous.
    /// NOTE: Will panic of the spans are not contiguous!
    pub fn join(&self, other: &Input<I>) -> Input<I> {
        Input::new_with_span(self.underlying.fork(), self.span.union(other.span))
    }

    /// Gets the span of the input.
    pub fn span(&self) -> Span {
        self.span
    }
}

impl<I: Underlying> PartialEq for Input<I> {
    fn eq(&self, other: &Self) -> bool {
        self.underlying
            .byte_span(self.span.head(), self.span.tail())
            == other
                .underlying
                .byte_span(other.span.head(), other.span.tail())
    }
}

impl<I: Underlying> PartialEq<I> for Input<I> {
    fn eq(&self, other: &I) -> bool {
        self.underlying
            .byte_span(self.span.head(), self.span.tail())
            == other.byte_span(0, other.len())
    }
}

impl<I: Underlying> PartialEq<&I> for Input<I> {
    fn eq(&self, other: &&I) -> bool {
        self.underlying
            .byte_span(self.span.head(), self.span.tail())
            == other.byte_span(0, other.len())
    }
}

impl<I: Underlying> From<I> for Input<I> {
    fn from(input: I) -> Self {
        Self::new(input)
    }
}

impl<I: Underlying> From<&I> for Input<I> {
    fn from(input: &I) -> Self {
        Self::new(input.fork())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = Input::new("hello");
        assert_eq!(input.span.head(), 0);
        assert_eq!(input.span.tail(), 5);

        let input = Input::new("".as_bytes());
        assert_eq!(input.span.head(), 0);
        assert_eq!(input.span.tail(), 0);
    }

    #[test]
    fn test_new_with_span() {
        let input = Input::new_with_span("hello", Span::new(1, 4));
        assert_eq!(input.span.head(), 1);
        assert_eq!(input.span.tail(), 4);
    }

    #[test]
    fn test_next() {
        let mut input = Input::new("hello");
        assert_eq!(input.next(), Some(b'h'));
        assert_eq!(input.span.head(), 1);
        assert_eq!(input.next(), Some(b'e'));
        assert_eq!(input.next(), Some(b'l'));
        assert_eq!(input.next(), Some(b'l'));
        assert_eq!(input.next(), Some(b'o'));
        // Should get None after consuming all characters
        assert_eq!(input.next(), None);
        assert_eq!(input.span.head(), 5);
    }

    #[test]
    fn test_peek() {
        let mut input = Input::new("hello");
        assert_eq!(input.peek(), Some(b'h'));
        input.next(); // consume 'h'
        assert_eq!(input.peek(), Some(b'e'));
        input.next(); // consume 'e'
        assert_eq!(input.peek(), Some(b'l'));

        // Consume the rest and check we get None
        input.next(); // 'l'
        input.next(); // 'l'
        input.next(); // 'o'
        assert_eq!(input.peek(), None);
    }

    #[test]
    fn test_peek_nth() {
        let input = Input::new("hello");
        assert_eq!(input.peek_nth(0), Some(b'h'));
        assert_eq!(input.peek_nth(1), Some(b'h'));
        assert_eq!(input.peek_nth(2), Some(b'e'));
        assert_eq!(input.peek_nth(3), Some(b'l'));
        assert_eq!(input.peek_nth(4), Some(b'l'));
        assert_eq!(input.peek_nth(5), Some(b'o'));
        assert_eq!(input.peek_nth(6), None);

        let mut input = Input::new("hello");
        input.next(); // consume 'h'
        assert_eq!(input.peek_nth(0), Some(b'e'));
        assert_eq!(input.peek_nth(1), Some(b'e'));
        assert_eq!(input.peek_nth(2), Some(b'l'));
        assert_eq!(input.peek_nth(3), Some(b'l'));
        assert_eq!(input.peek_nth(4), Some(b'o'));
        assert_eq!(input.peek_nth(5), None);
    }

    #[test]
    fn test_take() {
        let input = Input::new("hello");

        // Take first 2 characters
        let taken = input.take(2);
        assert_eq!(taken.span.head(), 0);
        assert_eq!(taken.span.tail(), 2);

        // Original input should be unchanged
        assert_eq!(input.span.head(), 0);
        assert_eq!(input.span.tail(), 5);

        // Test taking after consuming some characters
        let mut input = Input::new("hello");
        input.next(); // consume 'h'
        input.next(); // consume 'e'

        // Take should start from current head position (2)
        let taken = input.take(2);
        assert_eq!(taken.span.head(), 2);
        assert_eq!(taken.span.tail(), 4);

        // Test taking more than available from current position
        let taken = input.take(10);
        assert_eq!(taken.span.head(), 2);
        assert_eq!(taken.span.tail(), 5);
    }

    #[test]
    fn test_skip() {
        let input = Input::new("hello");

        // Skip 2 characters
        let skipped = input.skip(2);
        assert_eq!(skipped.span.head(), 2);
        assert_eq!(skipped.span.tail(), 5);

        // Original input should be unchanged
        assert_eq!(input.span.head(), 0);
        assert_eq!(input.span.tail(), 5);

        // Test skip after consuming some characters
        let mut input = Input::new("hello");
        input.next(); // consume 'h'

        // Skip should start from current head position (1)
        let skipped = input.skip(2);
        assert_eq!(skipped.span.head(), 3);
        assert_eq!(skipped.span.tail(), 5);

        // Test skipping more than available
        let skipped = input.skip(10);
        assert_eq!(skipped.span.head(), 5);
        assert_eq!(skipped.span.tail(), 5);
    }

    #[test]
    fn test_chained_operations() {
        let input = Input::new("hello world");

        // Take the first 5 characters ("hello")
        let hello = input.take(5);
        assert_eq!(hello.span.head(), 0);
        assert_eq!(hello.span.tail(), 5);

        // Skip 1 from the original input (space after "hello")
        let after_space = input.skip(6); // Skip "hello "
        assert_eq!(after_space.span.head(), 6);
        assert_eq!(after_space.span.tail(), 11);

        // Test taking and then skipping
        // Take 5 ("hello") then skip 1 (skips the first char of "hello")
        let hello_minus_h = input.take(5).skip(1);
        assert_eq!(hello_minus_h.span.head(), 1);
        assert_eq!(hello_minus_h.span.tail(), 5);

        // Extracting "world" requires skipping 6 characters first
        let world = input.skip(6);
        assert_eq!(world.span.head(), 6);
        assert_eq!(world.span.tail(), 11);

        // Check we can extract "world"
        let mut world_iter = world;
        assert_eq!(world_iter.next(), Some(b'w'));
        assert_eq!(world_iter.next(), Some(b'o'));
        assert_eq!(world_iter.next(), Some(b'r'));
        assert_eq!(world_iter.next(), Some(b'l'));
        assert_eq!(world_iter.next(), Some(b'd'));
        assert_eq!(world_iter.next(), None);
    }

    #[test]
    fn test_with_bytes() {
        let bytes = b"hello";
        let mut input = Input::new(bytes.as_slice());

        assert_eq!(input.next(), Some(b'h'));
        assert_eq!(input.next(), Some(b'e'));
        assert_eq!(input.peek(), Some(b'l'));

        // Create a new input with just "ll" remaining
        let mut taken = input.take(2);
        println!("{:?}", taken);
        assert_eq!(taken.next(), Some(b'l'));
        assert_eq!(taken.next(), Some(b'l'));
        assert_eq!(taken.next(), None);
    }

    #[test]
    fn test_input_eq_input() {
        // Same content, same spans
        let input1 = Input::new("hello");
        let input2 = Input::new("hello");
        assert_eq!(input1, input2);

        // Same content, different spans
        let input1 = Input::new_with_span("hello", Span::new(1, 4));
        let input2 = Input::new_with_span("hello", Span::new(1, 4));
        assert_eq!(input1, input2);

        // Same content, different spans (should not be equal)
        let input1 = Input::new_with_span("hello", Span::new(0, 5));
        let input2 = Input::new_with_span("hello", Span::new(1, 5));
        assert_ne!(input1, input2);

        // Same spans, different content
        let input1 = Input::new("hello");
        let input2 = Input::new("world");
        assert_ne!(input1, input2);

        // Partially consumed inputs
        let mut input1 = Input::new("hello");
        input1.next();
        let input2 = Input::new_with_span("hello", Span::new(1, 5));
        assert_eq!(input1, input2);

        // Using different underlying types
        //let input1 = Input::new("hello");
        //let input2 = Input::new(b"hello".as_slice());
        //assert_eq!(input1, input2);
    }

    #[test]
    fn test_input_eq_underlying() {
        // Input equals the same string
        let input = Input::new("hello");
        assert_eq!(input, "hello");

        // Input with span equals substring
        let input = Input::new_with_span("hello", Span::new(1, 4));
        assert_eq!(input, "ell");

        // Partially consumed input equals substring
        let mut input = Input::new("hello");
        input.next();
        input.next();
        assert_eq!(input, "llo");

        // Input equals byte slice
        let input = Input::new(b"hello".as_slice());
        assert_eq!(input, b"hello".as_slice());

        // Input doesn't equal a different value
        let input = Input::new("hello");
        assert_ne!(input, "world");
    }

    #[test]
    fn test_input_eq_ref_underlying() {
        // Input equals reference to underlying
        let s = "hello";
        let input = Input::new(s);
        assert_eq!(input, &s);

        // Input with span equals reference to substring
        let s = "hello";
        let input = Input::new_with_span(s, Span::new(1, 4));
        let substring = "ell";
        assert_eq!(input, &substring);

        // Input doesn't equal reference to different value
        let s = "hello";
        let different = "world";
        let input = Input::new(s);
        assert_ne!(input, &different);
    }

    #[test]
    fn test_edge_cases() {
        // Empty inputs
        let input1 = Input::new("");
        let input2 = Input::new("");
        assert_eq!(input1, input2);
        assert_eq!(input1, "");

        // Empty span
        let input = Input::new_with_span("hello", Span::new(2, 2));
        assert_eq!(input, "");

        // Input at end of string
        let mut input = Input::new("hi");
        input.next();
        input.next();
        assert_eq!(input, "");
    }

    #[test]
    fn is_correctly_subtracting() {
        let input1 = Input::new("hello");
        let input2 = Input::new("hello");
        let subtracted = input1.subtract(&input2);
        assert_eq!(subtracted, "");
        assert_eq!(subtracted.span.head(), 0);
        assert_eq!(subtracted.span.tail(), 0);

        let input1 = Input::new("hello, world!");
        let input2 = Input::new("hello");
        assert_eq!(input1.subtract(&input2), ", world!");
        assert_eq!(input2.subtract(&input1), "");
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn gets_next_char() {
        let mut input = Input::new("hello");
        assert_eq!(input.next_char(), Some('h'));
        assert_eq!(input.next_char(), Some('e'));
        assert_eq!(input.next_char(), Some('l'));
        assert_eq!(input.next_char(), Some('l'));
        assert_eq!(input.next_char(), Some('o'));
        assert_eq!(input.next_char(), None);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn gets_next_char_with_unicode() {
        let mut input = Input::new("héllö😊");
        assert_eq!(input.next_char(), Some('h'));
        assert_eq!(input.next_char(), Some('é'));
        assert_eq!(input.next_char(), Some('l'));
        assert_eq!(input.next_char(), Some('l'));
        assert_eq!(input.next_char(), Some('ö'));
        assert_eq!(input.next_char(), Some('😊'));
        assert_eq!(input.next_char(), None);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn peeks_next_char() {
        let mut input = Input::new("hello");
        assert_eq!(input.peek_char(), Some('h'));
        assert_eq!(input.peek_char(), Some('h'));
        assert_eq!(input.next_char(), Some('h'));
        assert_eq!(input.peek_char(), Some('e'));
        assert_eq!(input.next_char(), Some('e'));
        assert_eq!(input.peek_char(), Some('l'));
        assert_eq!(input.next_char(), Some('l'));
        assert_eq!(input.next_char(), Some('l'));
        assert_eq!(input.next_char(), Some('o'));
        assert_eq!(input.peek_char(), None);
        assert_eq!(input.next_char(), None);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn peeks_next_char_with_unicode() {
        let mut input = Input::new("héllö😊");
        assert_eq!(input.peek_char(), Some('h'));
        assert_eq!(input.peek_char(), Some('h'));
        assert_eq!(input.next_char(), Some('h'));
        assert_eq!(input.peek_char(), Some('é'));
        assert_eq!(input.next_char(), Some('é'));
        assert_eq!(input.peek_char(), Some('l'));
        assert_eq!(input.next_char(), Some('l'));
        assert_eq!(input.next_char(), Some('l'));
        assert_eq!(input.peek_char(), Some('ö'));
        assert_eq!(input.next_char(), Some('ö'));
        assert_eq!(input.peek_char(), Some('😊'));
        assert_eq!(input.next_char(), Some('😊'));
        assert_eq!(input.peek_char(), None);
        assert_eq!(input.next_char(), None);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn peeks_nth_char() {
        let mut input = Input::new("hello");
        assert_eq!(input.peek_nth_char(0), Some('h'));
        assert_eq!(input.peek_nth_char(1), Some('h'));
        assert_eq!(input.peek_nth_char(2), Some('e'));
        assert_eq!(input.peek_nth_char(3), Some('l'));
        assert_eq!(input.peek_nth_char(4), Some('l'));
        assert_eq!(input.peek_nth_char(5), Some('o'));
        assert_eq!(input.peek_nth_char(6), None);
        assert_eq!(input.next_char(), Some('h'));
        assert_eq!(input.peek_nth_char(0), Some('e'));
        assert_eq!(input.peek_nth_char(1), Some('e'));
        assert_eq!(input.peek_nth_char(2), Some('l'));
    }
}
