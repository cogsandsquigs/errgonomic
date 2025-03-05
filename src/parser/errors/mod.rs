use core::fmt;

use super::{
    input::{Span, Underlying},
    state::State,
};

/// The result type for the parser.
/// NOTE: This will always return a `State` since we may want to continue parsing even if an error
/// has occurred. It is just that the `Ok` variant will contain the result of the parsing.
pub type Result<I, O, E = DummyError> = core::result::Result<(State<I, E>, O), State<I, E>>;

/// Any possible errors that could have occurred during parsing.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error<I, E = DummyError>
where
    I: Underlying,
    E: CustomError,
{
    /// The error we have
    kind: ErrorKind<I, E>,

    /// Where the error was, with the span of the input. Note that if the span is more than 1
    /// byte, then the error happened over all of those bytes.
    ///
    /// NOTE: If the underlying error contains sub-errors, then this will be the span of the
    /// sub-errors unioned together.
    from: Span,
}

impl<I, E> Error<I, E>
where
    I: Underlying,
    E: CustomError,
{
    /// Create a new `Error` object.
    pub fn new(kind: ErrorKind<I, E>, from: Span) -> Self {
        Self { kind, from }
    }

    /// Create an empty `Errors` object.
    pub fn empty(span: Span) -> Self {
        Self {
            kind: ErrorKind::None,
            from: span,
        }
    }

    /// Get the span of the error.
    pub fn span(&self) -> Span {
        self.from
    }

    /// Return the number of errors.
    pub fn len(&self) -> usize {
        self.kind.len()
    }

    /// Checks if there are any errors.
    pub fn is_empty(&self) -> bool {
        self.kind.len() == 0
    }

    /// Appends an error to the list of errors.
    pub fn push(&mut self, error: Error<I, E>) {
        match self.kind {
            ErrorKind::None => {
                *self = error;
            }
            ErrorKind::Sequence(ref mut errors) => {
                self.from.union_between(error.from);
                errors.push(error);
            }
            _ => {
                // NOTE: Cloning before err. update so that we can use the original error span in the sequence.
                let s = self.clone();
                self.from.union_between(error.from);
                self.kind = ErrorKind::Sequence(vec![s, error]);
            }
        }
    }
}

/// The kind of error we are dealing with.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind<I, E = DummyError>
where
    I: Underlying,
    E: CustomError,
{
    /// No errors! We are safe!
    None,

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
    fn len(&self) -> usize {
        match self {
            Self::None => 0,
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
