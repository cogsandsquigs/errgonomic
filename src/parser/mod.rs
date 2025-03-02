//! The parser types. This dictates how the parser is used and how it should be ran.

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
    ///
    /// ```
    /// # use errgonomic::combinators::id;
    /// # use errgonomic::parser::Parser;
    /// let (state, parsed) = id.process("test".into()).unwrap();
    /// assert_eq!(parsed, "test");
    /// assert_eq!(state.as_input().as_inner(), "");
    /// ```
    fn process(&mut self, state: State<I>) -> Result<I, O>;

    /// Parses an input and returns an output.
    /// WARN: When making parsers, this should *not* be the function to process state and
    /// state-changes. Use `process` instead.
    ///
    /// ```
    /// # use errgonomic::combinators::id;
    /// # use errgonomic::parser::Parser;
    /// let parsed = id.parse("test").unwrap();
    /// assert_eq!(parsed, "test");
    /// ```
    fn parse(&mut self, input: I) -> core::result::Result<O, Errors<I>> {
        self.process(State::new(input))
            .map(|(state, output)| {
                assert!(!state.errors().any_errs());
                output
            })
            .map_err(|state| state.errors().clone())
    }

    /// Processes the output of the parser with a function.
    /// ```
    /// # use errgonomic::combinators::decimal;
    /// # use errgonomic::parser::Parser;
    /// # use errgonomic::parser::input::Input;
    /// let parsed = decimal.map(|o: Input<&str>| o.as_inner().parse::<u32>().unwrap()).parse("123").unwrap();
    /// assert_eq!(parsed, 123);
    /// ```
    fn map<O2, F: Fn(O) -> O2>(mut self, f: F) -> impl Parser<I, O2>
    where
        Self: Sized,
    {
        move |state: State<I>| {
            self.process(state)
                .map(|(state, output)| (state, f(output)))
        }
    }

    /// Applies two parsers in sequence. Returns the output of both parsers.
    /// ```
    /// # use errgonomic::combinators::{decimal, hexadecimal};
    /// # use errgonomic::parser::Parser;
    /// let (first, second) = decimal.then(hexadecimal).parse("123abc123").unwrap();
    /// assert_eq!(first, "123");
    /// assert_eq!(second, "abc123");
    /// ```
    fn then<O2, P2: Parser<I, O2>>(mut self, mut p2: P2) -> impl Parser<I, (O, O2)>
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

    /// Applies a secondary parser depending on the output generated from the first. Like `then`,
    /// but more general and dependent on the output of the first parser.
    /// ```
    /// # use errgonomic::combinators::{decimal, hexadecimal, is, any};
    /// # use errgonomic::parser::Parser;
    /// # use errgonomic::parser::input::Input;
    /// let parsed = any((is("dec:"), is("hex:")))
    ///                           .chain(|o: &Input<&str>| {
    ///                               if o.as_inner() == "dec:" {
    ///                                   decimal
    ///                               } else {
    ///                                   hexadecimal
    ///                               }
    ///                           })
    ///                           .parse("dec:123")
    ///                           .unwrap();
    /// assert_eq!(parsed.0, "dec:");
    /// assert_eq!(parsed.1, "123");
    /// ```
    fn chain<O2, P2: Parser<I, O2>, F: Fn(&O) -> P2>(mut self, f: F) -> impl Parser<I, (O, O2)>
    where
        Self: Sized,
    {
        move |state: State<I>| {
            self.process(state).and_then(|(state, output)| {
                f(&output)
                    .process(state)
                    .map(|(state, output2)| (state, (output, output2)))
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
