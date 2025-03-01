pub mod errors;
pub mod input;
pub mod span;
pub mod state;

use errors::{ErrorCollection, PResult, ParseError, ParseResult};
use input::ParseInput;
use state::ParserState;

pub trait Parser<I: ParseInput, O, E: ParseError> {
    /// Processes the parser state and returns the result.
    /// NOTE: This *should* be used when chaining parsers together, as it passes the state around.
    fn process(&mut self, state: ParserState<I, E>) -> PResult<I, O, E>;

    /// Parses according to the rules defined in this `Parser`.
    /// WARN: When making parsers, this should *not* be called! This is to process the *full input*
    /// and return a result. It does not pass state around, so it's not useful for chaining
    /// parsers.
    fn parse(&mut self, input: I) -> Result<O, ErrorCollection<E, I>> {
        self.process(ParserState::new(input)).into_result()
    }
}

impl<I: ParseInput, O, E: ParseError, P> Parser<I, O, E> for P
where
    P: FnMut(ParserState<I, E>) -> PResult<I, O, E>,
{
    fn process(&mut self, state: ParserState<I, E>) -> PResult<I, O, E> {
        self(state)
    }
}
