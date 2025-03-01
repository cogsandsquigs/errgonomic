use core::fmt;

use super::{input::ParseInput, span::Span, state::ParserState, Parser};

pub type PResult<I, O, E> = Result<(ParserState<I, E>, O), ParserState<I, E>>;

pub trait ParseResult<I: ParseInput, O, E: ParseError> {
    /// Converts the `ParseResult` into a normal `Result`.
    fn into_result(self) -> Result<O, ErrorCollection<E, I>>;

    /// Chains a parser to the result, assuming no error occured. Returns a tuple with both of the
    /// outputs of both parsers.
    fn chain<P, O2>(self, p: P) -> PResult<I, (O, O2), E>
    where
        P: Parser<I, O2, E>;
}

impl<I: ParseInput, O, E: ParseError> ParseResult<I, O, E> for PResult<I, O, E> {
    fn into_result(self) -> Result<O, ErrorCollection<E, I>> {
        self.map_err(|e| e.errors().clone()).map(|(_, o)| o)
    }

    fn chain<P, O2>(self, mut p: P) -> PResult<I, (O, O2), E>
    where
        P: Parser<I, O2, E>,
    {
        self.and_then(|(state, o)| p.process(state).map(|(state, o2)| (state, (o, o2))))
    }
}

#[derive(Debug, Clone)]
pub enum ErrorCollection<E: ParseError, I: ParseInput> {
    /// A single error.
    Single(ErrorWrapper<E, I>),

    /// A tree of errors. This represents a tree of errors occuring during parsing, where we are
    /// parsing tree-like structures which have recursive errors. It also allows us to represent a
    /// list of errors, with trees of depth=1.
    /// NOTE: The error collection at index 0 is the "first" error occured during parsing.
    Tree(Vec<ErrorCollection<E, I>>),
}

impl<E: ParseError, I: ParseInput> ErrorCollection<E, I> {
    /// Creates a new `ErrorCollection` with a single error.
    pub fn new_single(error: ErrorWrapper<E, I>) -> Self {
        ErrorCollection::Single(error)
    }

    /// Creates a new `ErrorCollection` with a tree of errors.
    pub fn new_tree(collections: Vec<ErrorCollection<E, I>>) -> Self {
        ErrorCollection::Tree(collections)
    }

    /// Gets all the errors in the `ErrorCollection`.
    pub fn all_errors(&self) -> Vec<ErrorWrapper<E, I>> {
        match self {
            ErrorCollection::Single(e) => vec![e.clone()],
            ErrorCollection::Tree(es) => es.iter().flat_map(|e| e.all_errors()).collect(),
        }
    }

    /// Appends an error to the `ErrorCollection`.
    pub fn append(&mut self, error: ErrorWrapper<E, I>) {
        match self {
            ErrorCollection::Single(e) => {
                *self = ErrorCollection::Tree(vec![
                    ErrorCollection::Single(e.clone()),
                    ErrorCollection::Single(error),
                ]);
            }
            ErrorCollection::Tree(es) => {
                es.push(ErrorCollection::Single(error));
                *self = ErrorCollection::new_tree(es.to_vec());
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ErrorWrapper<E: ParseError, I: ParseInput> {
    pub error: E,
    pub span: Span<I>,
}

impl<E: ParseError, I: ParseInput> ErrorWrapper<E, I> {
    pub fn new(error: E, span: Span<I>) -> Self {
        ErrorWrapper { error, span }
    }
}

/// A trait representing an error that can occur during parsing. This allows for user-defined
/// parse errors.
pub trait ParseError: core::error::Error + PartialEq + Eq + Clone + fmt::Debug {}

/// The default error that occurs when the input is empty.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DefaultError {
    /// An error that occurs when the input is empty.
    #[error("Empty input")]
    EmptyInput,
}

impl ParseError for DefaultError {}
