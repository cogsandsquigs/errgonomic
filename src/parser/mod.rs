pub mod errors;
pub mod input;
pub mod span;
pub mod state;

use errors::{Errors, Result};
use input::Underlying;
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
    fn map<O2, F: Fn(O) -> O2>(mut self, f: F) -> impl Parser<I, O2>
    // Map<I, O, Self, F, O2>
    where
        Self: Sized,
    {
        move |state: State<I>| {
            self.process(state)
                .map(|(state, output)| (state, f(output)))
        }
    }

    /// Applies two parsers in sequence. Returns the output of both parsers.
    fn chain<O2, P2: Parser<I, O2>>(mut self, mut p2: P2) -> impl Parser<I, (O, O2)>
    where
        Self: Sized,
    {
        move |state: State<I>| {
            self.process(state).and_then(|(state, output1)| {
                p2.process(state)
                    .map(|(state, output2)| (state, (output1, output2)))
            })
        }
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
