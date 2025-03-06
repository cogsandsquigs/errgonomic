//! The parser types. This dictates how the parser is used and how it should be ran.
pub mod errors;
pub mod input;
pub mod state;

use errors::{CustomError, DummyError, Error, ErrorKind, Result};
use input::Underlying;
use state::State;

/// The parser trait. Used to parse input.
pub trait Parser<I, O, E = DummyError>
where
    I: Underlying,
    E: CustomError,
{
    /// Processes a parser state and returns a new state.
    /// NOTE: When making parsers, this should be the function to process state and state-changes.
    ///
    /// ```
    /// # use errgonomic::combinators::id;
    /// # use errgonomic::parser::Parser;
    /// # use errgonomic::parser::state::State;
    /// # use errgonomic::parser::input::Input;
    /// let (state, parsed): (State<&str>, Input<&str>) = id.process("test".into()).unwrap();
    /// assert_eq!(parsed, "test");
    /// assert_eq!(state.as_input().as_inner(), "");
    /// ```
    fn process(&mut self, state: State<I, E>) -> Result<I, O, E>;

    /// Parses an input and returns an output.
    /// WARN: When making parsers, this should *not* be the function to process state and
    /// state-changes. Use `process` instead.
    ///
    /// ```
    /// # use errgonomic::combinators::id;
    /// # use errgonomic::parser::Parser;
    /// # use errgonomic::parser::state::State;
    /// # use errgonomic::parser::input::Input;
    /// # use errgonomic::parser::errors::DummyError;
    /// let parsed = id::<_, DummyError>.parse("test").unwrap();
    /// assert_eq!(parsed, "test");
    /// ```
    #[inline]
    fn parse(&mut self, input: I) -> core::result::Result<O, Error<I, E>> {
        self.process(State::new(input))
            .map_err(|state| state.errors().clone())
            .map(|(state, output)| {
                if state.is_err() {
                    Err(state.errors().clone())
                } else {
                    Ok(output)
                }
            })?
    }

    /// Processes the output of the parser with a function.
    /// ```
    /// # use errgonomic::combinators::decimal;
    /// # use errgonomic::parser::Parser;
    /// # use errgonomic::parser::input::Input;
    /// # use errgonomic::parser::errors::DummyError;
    /// let parsed = decimal::<_, DummyError>.map(|o: Input<&str>| o.as_inner().parse::<u32>().unwrap()).parse("123").unwrap();
    /// assert_eq!(parsed, 123);
    /// ```
    #[inline]
    fn map<O2, F: Fn(O) -> O2>(mut self, f: F) -> impl Parser<I, O2, E>
    where
        Self: Sized,
    {
        move |state: State<I, E>| {
            self.process(state)
                .map(|(state, output)| (state, f(output)))
        }
    }

    /// Processes both the output and state of the parser with a function.
    ///
    /// NOTE: Passes the state as the first argument and the output as the second.
    /// NOTE: The state is owned by the function, so it can be mutated. However, this means the
    /// function needs to return the state as well in a tuple with the state and output.
    ///
    /// ```
    /// # use errgonomic::combinators::decimal;
    /// # use errgonomic::parser::Parser;
    /// # use errgonomic::parser::input::Input;
    /// # use errgonomic::parser::errors::DummyError;
    /// let parsed = decimal::<_, DummyError>.map(|o: Input<&str>| o.as_inner().parse::<u32>().unwrap()).parse("123").unwrap();
    /// assert_eq!(parsed, 123);
    /// ```
    #[inline]
    fn map_with_state<O2, F: Fn(State<I, E>, O) -> (State<I, E>, O2)>(
        mut self,
        f: F,
    ) -> impl Parser<I, O2, E>
    where
        Self: Sized,
    {
        move |state: State<I, E>| self.process(state).map(|(state, output)| f(state, output))
    }

    /// Like `map`, but processes the output with a function that returns a (std) `Result`. If it's
    /// `Ok`, parsing continues as normal. If it's `Err`, the error is returned.
    #[inline]
    fn map_res<O2, F: Fn(O) -> core::result::Result<O2, E>>(mut self, f: F) -> impl Parser<I, O2, E>
    where
        Self: Sized,
    {
        move |state: State<I, E>| {
            let orig_input = state.as_input().fork();
            self.process(state).and_then(|(state, output)| {
                f(output)
                    .map_err(|e| {
                        let input = state.as_input().fork();
                        state.fork().with_error(Error::new(
                            ErrorKind::custom(e),
                            orig_input.subtract(&input),
                        ))
                    })
                    .map(|output| (state, output))
            })
        }
    }

    /// Applies two parsers in sequence. Returns the output of both parsers.
    /// ```
    /// # use errgonomic::combinators::{decimal, hexadecimal};
    /// # use errgonomic::parser::Parser;
    /// # use errgonomic::parser::errors::DummyError;
    /// let (first, second) = decimal::<_, DummyError>.then(hexadecimal).parse("123abc123").unwrap();
    /// assert_eq!(first, "123");
    /// assert_eq!(second, "abc123");
    /// ```
    #[inline]
    fn then<O2, P2: Parser<I, O2, E>>(mut self, mut p2: P2) -> impl Parser<I, (O, O2), E>
    where
        Self: Sized,
    {
        move |state: State<I, E>| -> Result<I, (O, O2), E> {
            self.process(state).and_then(
                |(state, output1): (State<I, E>, _)| -> Result<I, (O, O2), E> {
                    p2.process(state)
                        .map(|(state, output2)| (state, (output1, output2)))
                },
            )
        }
    }

    /// Applies a secondary parser depending on the output generated from the first. Like `then`,
    /// but more general and dependent on the output of the first parser.
    /// ```
    /// # use errgonomic::combinators::{decimal, hexadecimal, is, any};
    /// # use errgonomic::parser::Parser;
    /// # use errgonomic::parser::input::Input;
    /// # use errgonomic::parser::errors::DummyError;
    /// let parsed = any((is::<_, DummyError>("dec:"), is("hex:")))
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
    #[inline]
    fn chain<O2, P2: Parser<I, O2, E>, F: Fn(&O) -> P2>(
        mut self,
        f: F,
    ) -> impl Parser<I, (O, O2), E>
    where
        Self: Sized,
    {
        move |state: State<I, E>| {
            self.process(state).and_then(|(state, output)| {
                f(&output)
                    .process(state)
                    .map(|(state, output2)| (state, (output, output2)))
            })
        }
    }

    /// Substitutes a parser's error message with a custom error message.
    ///
    /// NOTE: Replaces *all* the errors in the current state with the custom error.
    #[inline]
    fn with_err(mut self, e: E) -> impl Parser<I, O, E>
    where
        Self: Sized,
    {
        move |state: State<I, E>| {
            let orig_input = state.as_input().fork();
            self.process(state).map_err(|state| {
                let input = state.as_input().fork();
                state.with_error(Error::new(
                    ErrorKind::custom(e.clone()),
                    orig_input.subtract(&input),
                ))
            })
        }
    }
}

impl<I, O, E, P> Parser<I, O, E> for P
where
    I: Underlying,
    P: FnMut(State<I, E>) -> Result<I, O, E>,
    E: CustomError,
{
    #[inline]
    fn process(&mut self, state: State<I, E>) -> Result<I, O, E> {
        self(state)
    }
}
