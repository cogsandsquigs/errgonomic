use super::*;
use crate::parser::input::Underlying;

/// The kind of error we are dealing with.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind<I, E = DummyError>
where
    I: Underlying,
    E: CustomError,
{
    /// No errors! We are safe!
    None,

    /// Committed error. We went down this path and can't go back.
    Committed(Box<Error<I, E>>),

    /// Expected something
    Expected(ExpectedError<I>),

    /// During `any`, if all fail, this represents all the errors at once, as a single error. So,
    /// unlike `Sequence`, it represents *every error happening at the same time*.
    All(Vec<Error<I, E>>),

    /// A sequence of errors, where each failed one after the other of them failed. Different from
    /// `Any` as it denotes errors which may be unrelated or in different parts of the parsing
    /// stage.
    Sequence(Vec<Error<I, E>>),

    /// Custom error
    Custom(E),
}

impl<I, E> ErrorKind<I, E>
where
    I: Underlying,
    E: CustomError,
{
    /// Create a new `expected` error.
    pub fn expected(expected: ExpectedError<I>) -> Self {
        Self::Expected(expected)
    }

    /// Create a new `all` error.
    pub fn all(errors: Vec<Error<I, E>>) -> Self {
        Self::All(errors)
    }

    /// Create a new `custom` error.
    pub fn custom(err: E) -> Self {
        Self::Custom(err)
    }

    /// INTERNAL: Length
    pub(super) fn len(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Committed(e) => e.len(),
            Self::Expected(_) => 1,
            Self::All(errors) => errors.iter().map(|e| e.len()).sum(),
            Self::Sequence(errors) => errors.iter().map(|e| e.len()).sum(),
            Self::Custom(_) => 1,
        }
    }
}

/// We expect the input to be a specific thing, but it wasn't.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpectedError<I: Underlying> {
    /// We expected a specific thing/string to match, but didn't get it.
    Is(I),

    /// Expected something, but *not* this.
    Not(I),

    /// Expected a digit with radix `n`
    /// NOTE: `n=10` and `n=16` specify that we want decimal or hexidecimal numbers, respectively.
    Digit(u32),

    /// Expected an alphabetic character.
    Alpha,

    /// Expected an alphabetic *or* numeric character (base 10).
    AlphaNum,

    /// Expected whitespace, including newlines
    Whitespace,

    /// Expected newlines
    Newlines,

    /// Expected whitespace, not including newlines
    WhitespaceNoNewlines,

    /// Expected nothing/end-of-input, but found something.
    Nothing,

    /// Expected something, anything, but found nothing.
    Anything,
}

impl<I, E> fmt::Display for ErrorKind<I, E>
where
    I: Underlying,
    E: CustomError,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO - Format: {:?}", self) // TODO: FORMAT!
    }
}

impl<I, E> std::error::Error for ErrorKind<I, E>
where
    I: Underlying,
    E: CustomError,
{
}
