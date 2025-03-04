use core::fmt;
use std::ops::Index;

use super::{
    input::{Input, Underlying},
    state::State,
};

/// The result type for the parser.
/// NOTE: This will always return a `State` since we may want to continue parsing even if an error
/// has occurred. It is just that the `Ok` variant will contain the result of the parsing.
pub type Result<I, O, E = DummyError> = core::result::Result<(State<I, E>, O), State<I, E>>;

/// The error type for the parser.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Errors<I, E = DummyError>
where
    I: Underlying,
    E: CustomError,
{
    /// The source of the input that produced the errors.
    source: Input<I>,

    /// All the errors that occurred during parsing.
    errors: Vec<Error<I, E>>,
}

impl<I, E> Errors<I, E>
where
    I: Underlying,
    E: CustomError,
{
    /// Create a new `Errors` object.
    pub fn new(source: Input<I>) -> Self {
        Self {
            source,
            errors: Vec::new(),
        }
    }

    /// Return the number of errors.
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Checks if there are any errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Appends an error to the list of errors.
    pub fn push(&mut self, error: Error<I, E>) {
        self.errors.push(error);
    }
}

impl<I, E> Index<usize> for Errors<I, E>
where
    I: Underlying,
    E: CustomError,
{
    type Output = Error<I, E>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.errors[index]
    }
}

/// Any possible errors that could have occurred during parsing.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error<I, E = DummyError>
where
    I: Underlying,
    E: CustomError,
{
    /// Expected a specific thing, but didn't get it.
    /// NOTE: `expected` should be the expected input, and `found` should be the remaining input.
    Expected { expected: I, found: Input<I> },

    /// Expected something, but found EOI.
    FoundEOI { expected: I, eoi_at: Input<I> },

    /// Expected the end of input, but didn't get it.
    /// NOTE: `found` should be the remaining input.
    ExpectedEOI { found: Input<I> },

    /// Expected a number, but found something else.
    ExpectedDec { found: Input<I> },

    /// Expected a hexidecimal number, but found something else.
    ExpectedHex { found: Input<I> },

    /// Expected alphabetic characters, but found something else.
    ExpectedAlpha { found: Input<I> },

    /// Expected alphabetic or numeric characters, but found something else.
    ExpectedAlphaNum { found: Input<I> },

    /// Expected whitespace.
    ExpectedWhitespace { found: Input<I> },

    /// Expected newlines.
    ExpectedNewline { found: Input<I> },

    /// Did not expect sommething, but found it.
    NotExpected { found: Input<I> },

    /// Expected anything, but found nothing/EOI.
    ExpectedAny { eoi_at: Input<I> },

    /// A custom error
    Custom { err: E, at: Input<I> },
}

pub trait CustomError: fmt::Debug + PartialEq + Eq + Clone {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DummyError;

impl core::error::Error for DummyError {}

impl fmt::Display for DummyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dummy error! Should never be seen!")
    }
}

impl CustomError for DummyError {}
