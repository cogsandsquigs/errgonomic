use super::span::Span;

pub trait ParseInput: PartialEq + Eq + Clone {
    /// Gets the length of the input.
    fn len(&self) -> usize;

    /// Checks if the input is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Forks the input in-place, creating a new input that starts at the current head of the
    /// input. For references, it should be a shallow clone. For non-references, it should be a
    /// clone.
    fn fork(&self) -> Self;

    /// Grabs the input at a slice from the head to the tail. The `head` is inclusive, and the
    /// `tail` is not.
    fn slice(&self, span: &Span<Self>) -> Self;

    /// Takes the first `n` elements from the input, starting from the head, and returns them.
    fn take(&self, n: usize) -> Self {
        self.slice(&Span::new(0, n))
    }

    /// Skips the first `n` elements from the input, starting from the head, and returns the rest.
    fn skip(&self, n: usize) -> Self {
        self.slice(&Span::new(n, self.len()))
    }
}

impl ParseInput for &str {
    fn len(&self) -> usize {
        (self as &str).len()
    }

    fn fork(&self) -> Self {
        self
    }

    fn slice(&self, span: &Span<Self>) -> Self {
        &self[span.head()..span.tail()]
    }

    fn take(&self, n: usize) -> Self {
        &self[..n]
    }

    fn skip(&self, n: usize) -> Self {
        &self[n..]
    }
}
