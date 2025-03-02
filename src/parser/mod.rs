pub mod errors;
pub mod input;
mod mappings;
pub mod span;
pub mod state;

use errors::{Errors, Result};
use input::Underlying;
use mappings::{Chain, Map};
use state::State;

/// The parser trait. Used to parse input.
pub trait Parser<I, O>
where
    I: Underlying,
{
    /// Processes a parser state and returns a new state.
    /// NOTE: When making parsers, this should be the function to process state and state-changes.
    fn process(&mut self, state: State<I>) -> Result<I, O>;

    /// Parses an input and returns an output.
    /// WARN: When making parsers, this should *not* be the function to process state and
    /// state-changes. Use `process` instead.
    fn parse(&mut self, input: I) -> core::result::Result<O, Errors<I>> {
        self.process(State::new(input))
            .map(|(state, output)| {
                assert!(!state.errors().any_errs());
                output
            })
            .map_err(|state| state.errors().clone())
    }

    /// Processes the output of the parser with a function.
    fn map<O2, F: Fn(O) -> O2>(self, f: F) -> Map<I, O, Self, F, O2>
    where
        Self: Sized,
    {
        Map::new(self, f)
    }

    /// Applies two parsers in sequence. Returns the output of both parsers.
    fn chain<O2, P2: Parser<I, O2>>(self, p2: P2) -> mappings::Chain<I, O, O2, Self, P2>
    where
        Self: Sized,
    {
        Chain::new(self, p2)
    }
}

impl<I, O, P> Parser<I, O> for P
where
    I: Underlying,
    P: FnMut(State<I>) -> Result<I, O>,
{
    fn process(&mut self, state: State<I>) -> Result<I, O> {
        self(state)
    }
}
