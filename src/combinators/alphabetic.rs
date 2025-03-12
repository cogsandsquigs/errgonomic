use super::many_n;
use crate::parser::{
    errors::{CustomError, Error, ErrorKind, ExpectedError, Result},
    input::{Input, Underlying},
    state::State,
    Parser,
};

/// Parses an alphabetic character until it stops. If there is no alphabetic character, returns an error.
/// TODO: Unicode support
///
///```
/// # use errgonomic::combinators::alphabetic_char;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = alphabetic_char.process("abc123".into()).unwrap();
/// assert_eq!(parsed, "a");
/// assert_eq!(state.as_input().as_inner(), "bc123");
///```
pub fn alphabetic_char<I: Underlying, E: CustomError>(
    state: State<I, E>,
) -> Result<I, Input<I>, E> {
    let input = state.as_input().fork();
    match input.peek() {
        Some(c) if c.is_ascii_alphabetic() => {
            let num = input.take(1);
            Ok((state.with_input(input.skip(1)), num))
        }
        _ => Err(state.with_error(Error::new(
            ErrorKind::expected(ExpectedError::Digit(10)),
            input.take(1),
        ))),
    }
}

/// Parses a string of alphabetic characters until it stops. If there is no alphabetic character,
/// returns an error.
///
///```
/// # use errgonomic::combinators::alphabetic;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = alphabetic.process("abc123".into()).unwrap();
/// assert_eq!(parsed, "abc");
/// assert_eq!(state.as_input().as_inner(), "123");
///```
pub fn alphabetic<I: Underlying, E: CustomError>(state: State<I, E>) -> Result<I, Input<I>, E> {
    many_n(1, alphabetic_char)
        .map(|xs| {
            xs.into_iter()
                .reduce(|acc, x| acc.join(&x))
                .expect("to have parsed at least one character!")
        })
        .process(state)
}

/// Parses an alphanumeric character until it stops. If there is no alphanumeric character, returns an error.
/// TODO: Unicode support
///
///```
/// # use errgonomic::combinators::alphanumeric_char;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = alphanumeric_char.process("abc123".into()).unwrap();
/// assert_eq!(parsed, "a");
/// assert_eq!(state.as_input().as_inner(), "bc123");
///```
pub fn alphanumeric_char<I: Underlying, E: CustomError>(
    state: State<I, E>,
) -> Result<I, Input<I>, E> {
    let input = state.as_input().fork();
    match input.peek() {
        Some(c) if c.is_ascii_alphanumeric() => {
            let num = input.take(1);
            Ok((state.with_input(input.skip(1)), num))
        }
        _ => Err(state.with_error(Error::new(
            ErrorKind::expected(ExpectedError::Digit(10)),
            input.take(1),
        ))),
    }
}

/// Parses a string of alphanumeric characters until it stops. If there is no alphabetic character,
/// returns an error.
///
///```
/// # use errgonomic::combinators::alphanumeric;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = alphanumeric.process("abc123".into()).unwrap();
/// assert_eq!(parsed, "abc123");
/// assert_eq!(state.as_input().as_inner(), "");
///```
pub fn alphanumeric<I: Underlying, E: CustomError>(state: State<I, E>) -> Result<I, Input<I>, E> {
    many_n(1, alphanumeric_char)
        .map(|xs| {
            xs.into_iter()
                .reduce(|acc, x| acc.join(&x))
                .expect("to have parsed at least one character!")
        })
        .process(state)
}
