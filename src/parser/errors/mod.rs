use super::{
    input::{Input, Underlying},
    state::State,
};

/// The result type for the parser.
/// NOTE: This will always return a `State` since we may want to continue parsing even if an error
/// has occurred. It is just that the `Ok` variant will contain the result of the parsing.
pub type Result<I, O> = core::result::Result<(State<I>, O), State<I>>;

/// The error type for the parser.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Errors<I: Underlying> {
    /// The source of the input that produced the errors.
    source: Input<I>,

    /// All the errors that occurred during parsing.
    errors: Vec<Error<I>>,
}

impl<I: Underlying> Errors<I> {
    /// Create a new `Errors` object.
    pub fn new(source: Input<I>) -> Self {
        Self {
            source,
            errors: Vec::new(),
        }
    }

    /// Checks if there are any errors.
    pub fn any_errs(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Return the number of errors.
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }

    /// Returns the errors.
    pub fn errors(&self) -> &[Error<I>] {
        &self.errors
    }

    /// Appends an error to the list of errors.
    pub fn push(&mut self, error: Error<I>) {
        self.errors.push(error);
    }
}

/// Any possible errors that could have occurred during parsing.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error<I: Underlying> {
    /// Expected a specific thing, but didn't get it.
    /// NOTE: `expected` should be the expected input, and `found` should be the remaining input.
    Expected { expected: I, found: Input<I> },

    /// Expected something, but found EOI.
    FoundEOI { expected: I, eoi_at: Input<I> },

    /// Expected a number, but found something else.
    ExpectedDecNumber { found: Input<I> },

    /// Expected a hexidecimal number, but found something else.
    ExpectedHexNumber { found: Input<I> },

    /// Expected alphabetic characters, but found something else.
    ExpectedAlpha { found: Input<I> },

    /// Expected alphabetic or numeric characters, but found something else.
    ExpectedAlphaNum { found: Input<I> },

    /// Expected the end of input, but didn't get it.
    /// NOTE: `found` should be the remaining input.
    ExpectedEOI { found: Input<I> },
}
