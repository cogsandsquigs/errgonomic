mod custom;
mod kinds;

use super::{
    input::{Input, Underlying},
    state::State,
};

pub use custom::*;
pub use kinds::*;

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
    from: Input<I>,
}

impl<I, E> Error<I, E>
where
    I: Underlying,
    E: CustomError,
{
    /// Create a new `Error` object.
    pub fn new<F: Into<Input<I>>>(kind: ErrorKind<I, E>, from: F) -> Self {
        Self {
            kind,
            from: from.into(),
        }
    }

    /// Create an empty `Errors` object.
    pub fn empty<F: Into<Input<I>>>(from: F) -> Self {
        Self {
            kind: ErrorKind::None,
            from: from.into(),
        }
    }

    /// Commit on this error.
    pub fn commit(self) -> Self {
        let new_from = self.from.fork();
        if matches!(self.kind, ErrorKind::Committed(_)) {
            self
        } else {
            let new_kind = ErrorKind::Committed(Box::new(self));
            Error::new(new_kind, new_from)
        }
    }

    /// Check if it's committed.
    pub fn is_committed(&self) -> bool {
        matches!(self.kind, ErrorKind::Committed(_))
    }

    /// Get where the error is from.
    pub fn from(&self) -> Input<I> {
        self.from.fork()
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
                self.from.join_between(&error.from);
                errors.push(error);
            }
            _ => {
                // NOTE: Cloning before err. update so that we can use the original error span in the sequence.
                let s = self.clone();
                self.from.join_between(&error.from);
                self.kind = ErrorKind::Sequence(vec![s, error]);
            }
        }
    }
}
