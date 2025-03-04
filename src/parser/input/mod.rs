mod span;
mod underlying;

pub use span::*;
pub use underlying::*;

/// The input to the parser. Note that `Input` *never* actually deletes/shrinks the input, it only
/// just shrinks the *span* that it covers.
#[derive(Debug)]
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

    /// Consumes a single `Item` of the input and returns it.
    pub fn next(&mut self) -> Option<u8> {
        let idx = self.span.increment_head(1);
        if idx >= self.span.tail() || idx >= self.underlying.len() {
            return None;
        }
        self.underlying.byte_at(idx)
    }

    /// Peeks at the next byte (the one that would be returned by `next`) of the input without
    /// consuming it.
    pub fn peek(&self) -> Option<u8> {
        self.underlying.byte_at(self.span.head())
    }

    /// Peeks at the `n`th byte of the input from the current head (the index of the byte that
    /// would be returned at the next `.next` call).
    /// NOTE: `.peek_nth(0)` is equivalent to `.peek`.
    pub fn peek_nth(&self, n: usize) -> Option<u8> {
        self.underlying.byte_at(self.span.head() + n)
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
        assert_eq!(input.peek_nth(1), Some(b'e'));
        assert_eq!(input.peek_nth(2), Some(b'l'));
        assert_eq!(input.peek_nth(3), Some(b'l'));
        assert_eq!(input.peek_nth(4), Some(b'o'));
        assert_eq!(input.peek_nth(5), None);

        let mut input = Input::new("hello");
        input.next(); // consume 'h'
        assert_eq!(input.peek_nth(0), Some(b'e'));
        assert_eq!(input.peek_nth(1), Some(b'l'));
        assert_eq!(input.peek_nth(3), Some(b'o'));
        assert_eq!(input.peek_nth(4), None);
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
}
