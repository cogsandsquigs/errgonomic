use super::{
    errors::{ErrorCollection, ParseError},
    input::ParseInput,
    span::Span,
};

/// The state of the parser. This contains the input, the span, and the error collection.
/// NOTE: The input is always preserved, and never edited once given to the parser state. This is
/// to allow for rich error-handling and backtracking.
#[derive(Debug, Clone)]
pub struct ParserState<I: ParseInput, E: ParseError> {
    input: I,
    span: Span<I>,
    errors: ErrorCollection<E, I>,
}

impl<I: ParseInput, E: ParseError> ParserState<I, E> {
    /// Creates a new `ParserState` with the given input.
    pub fn new(input: I) -> Self {
        ParserState {
            span: Span::new(0, input.len()),
            input,
            errors: ErrorCollection::new_tree(vec![]),
        }
    }

    /// Returns the error collection.
    pub fn errors(&self) -> &ErrorCollection<E, I> {
        &self.errors
    }

    /// Gets the input as a shallow clone. For references, this should be a reference. For
    /// non-references, this should be a clone.
    pub fn input(&self) -> I {
        self.input.slice(&self.span)
    }

    /// Get the length of the current input.
    pub fn len(&self) -> usize {
        self.span.len()
    }

    /// Whether the input is empty.
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Forks the input in-place, creating a new input that starts at the current head of the
    /// input. For references, it should be a shallow clone. For non-references, it should be a
    /// clone.
    pub fn fork(&self) -> Self {
        Self {
            input: self.input.fork(),
            // NOTE: This is cheap b/c `span` is `Copy`. For some reason
            // Rust does not see that.
            span: self.span.clone(),
            errors: self.errors.clone(),
        }
    }

    /// Takes `n` elements from the input, starting from the head.
    pub fn take(&self, n: usize) -> Self {
        Self {
            input: self.input.fork(),
            span: Span::new(self.span.head() + n, self.span.tail()),
            errors: self.errors.clone(),
        }
    }
}
