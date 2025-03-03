use core::fmt;

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

    /// Checks if there are any errors.
    pub fn any_errs(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Return the number of errors.
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }

    /// Returns the errors.
    pub fn errors(&self) -> &[Error<I, E>] {
        &self.errors
    }

    /// Appends an error to the list of errors.
    pub fn push(&mut self, error: Error<I, E>) {
        self.errors.push(error);
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

#[cfg(feature = "fancy")]
impl<I, E> core::error::Error for Error<I, E>
where
    I: Underlying,
    E: CustomError,
{
}

#[cfg(feature = "fancy")]
impl<I, E> fmt::Display for Error<I, E>
where
    I: Underlying,
    E: CustomError,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expected { expected, found } => write!(
                f,
                "Expected `{}`, but found `{}`",
                expected,
                found.as_inner()
            ),
            Self::FoundEOI {
                expected,
                eoi_at: _,
            } => {
                write!(f, "Expected `{}`, but found end-of-input", expected,)
            }
            Self::ExpectedEOI { found } => {
                write!(f, "Expected end-of-input, but found `{}`", found.as_inner())
            }
            Self::ExpectedDec { found } => {
                write!(
                    f,
                    "Expected a decimal number, but found `{}`",
                    found.as_inner()
                )
            }
            Self::ExpectedHex { found } => {
                write!(
                    f,
                    "Expected a hexadecimal number, but found `{}`",
                    found.as_inner()
                )
            }
            Self::ExpectedAlpha { found } => {
                write!(
                    f,
                    "Expected alphabetic characters, but found `{}`",
                    found.as_inner()
                )
            }
            Self::ExpectedAlphaNum { found } => {
                write!(
                    f,
                    "Expected alphabetic or numeric characters, but found `{}`",
                    found.as_inner()
                )
            }
            Self::ExpectedWhitespace { found } => {
                write!(f, "Expected whitespace, but found `{}`", found.as_inner())
            }
            Self::ExpectedNewline { found } => {
                write!(f, "Expected newlines, but found `{}`", found.as_inner())
            }
            Self::NotExpected { found } => {
                write!(f, "Did not expect `{}`, but found it", found.as_inner())
            }
            Self::ExpectedAny { eoi_at: _ } => {
                write!(f, "Found end-of-input at")
            }
            Self::Custom { err, at: _ } => {
                write!(f, "{}", err,)
            }
        }
    }
}

/*
#[cfg(feature = "fancy")]
impl<I, E> miette::Diagnostic for Error<I, E>
where
    I: Underlying,
    E: CustomError,
{
    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self {
            Self::Expected { expected, found } => Some(Box::new(std::iter::once(
                miette::LabeledSpan::new_with_span(Some(format!("Expected {}", expected)), found),
            ))),

            _ => todo!("Need to add all the labels!"),
        }
    }
}
*/

#[cfg(feature = "fancy")]
pub trait CustomError:
    fmt::Debug + PartialEq + Eq + Clone + core::error::Error + miette::Diagnostic
{
}

#[cfg(not(feature = "fancy"))]
pub trait CustomError: fmt::Debug + PartialEq + Eq + Clone {}

#[cfg_attr(
    feature = "fancy",
    derive(Debug, PartialEq, Eq, Clone, miette::Diagnostic)
)]
#[cfg_attr(not(feature = "fancy"), derive(Debug, PartialEq, Eq, Clone))]
pub struct DummyError;

impl CustomError for DummyError {}

impl fmt::Display for DummyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dummy error! Should never be seen!")
    }
}

#[cfg(feature = "fancy")]
impl core::error::Error for DummyError {}

// #[cfg(feature = "fancy")]
// impl miette::Diagnostic for DummyError {}
