use crate::parser::{
    errors::{CustomError, Error, ErrorKind, ExpectedError, Result},
    input::{Input, Underlying},
    state::State,
    Parser,
};

use super::many_n;

/// Parses a decimal digit until it stops. If there is no decimal digit, returns an error.
///```
/// # use errgonomic::combinators::decimal_digit;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = decimal_digit.process("123abc".into()).unwrap();
/// assert_eq!(parsed, "1");
/// assert_eq!(state.as_input().as_inner(), "23abc");
///```
pub fn decimal_digit<I: Underlying, E: CustomError>(state: State<I, E>) -> Result<I, Input<I>, E> {
    let input = state.as_input().fork();
    match input.peek() {
        Some(c) if c.is_ascii_digit() => {
            let num = input.take(1);
            Ok((state.with_input(input.skip(1)), num))
        }
        _ => {
            let x = input.take(1).span();
            Err(state.with_error(Error::new(ErrorKind::expected(ExpectedError::Digit(10)), x)))
        }
    }
}

/// Parses a decimal number until it stops. If there is no decimal number, returns an error.
///```
/// # use errgonomic::combinators::decimal;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = decimal.process("123abc".into()).unwrap();
/// assert_eq!(parsed, "123");
/// assert_eq!(state.as_input().as_inner(), "abc");
///```
pub fn decimal<I: Underlying, E: CustomError>(state: State<I, E>) -> Result<I, Input<I>, E> {
    many_n(1, decimal_digit)
        .map(|xs| {
            xs.into_iter()
                .reduce(|acc, x| acc.join(&x))
                .expect("to have parsed at least one digit!")
        })
        .process(state)
}

/// Parses a hexadecimal digit until it stops. If there is no hexadecimal digit, returns an error.
///```
/// # use errgonomic::combinators::hexadecimal_digit;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = hexadecimal_digit.process("123abcdefghi".into()).unwrap();
/// assert_eq!(parsed, "1");
/// assert_eq!(state.as_input().as_inner(), "23abcdefghi");
///```
pub fn hexadecimal_digit<I: Underlying, E: CustomError>(
    state: State<I, E>,
) -> Result<I, Input<I>, E> {
    let input = state.as_input().fork();
    match state.as_input().peek() {
        Some(c) if c.is_ascii_hexdigit() => {
            let num = input.fork().take(1);
            Ok((state.with_input(input.skip(1)), num))
        }
        _ => Err(state.with_error(Error::new(
            ErrorKind::expected(ExpectedError::Digit(16)),
            input.take(1).span(),
        ))),
    }
}

/// Parses a hexadecimal number until it stops. If there is no hexadecimal number, returns an
/// error.
///```
/// # use errgonomic::combinators::hexadecimal;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = hexadecimal.process("123abcdefghi".into()).unwrap();
/// assert_eq!(parsed, "123abcdef");
/// assert_eq!(state.as_input().as_inner(), "ghi");
///```
pub fn hexadecimal<I: Underlying, E: CustomError>(state: State<I, E>) -> Result<I, Input<I>, E> {
    many_n(1, hexadecimal_digit)
        .map(|xs| {
            xs.into_iter()
                .reduce(|acc, x| acc.join(&x))
                .expect("to have parsed at least one digit!")
        })
        .process(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn can_parse_dec_digit() {
        let (state, parsed): (State<&str>, Input<&str>) =
            decimal_digit.process("123".into()).unwrap();
        assert_eq!(parsed, "1");
        assert_eq!(state.as_input(), &"23");
        assert!(!state.is_err());

        let result: State<&str> = decimal_digit.process("abc".into()).unwrap_err();
        assert!(result.is_err());
        assert_eq!(result.errors().len(), 1);
        assert_eq!(
            result.errors(),
            &Error::new(ErrorKind::expected(ExpectedError::Digit(10)), (0..1).into())
        );
    }

    #[test]
    fn can_parse_decimals() {
        let (state, parsed): (State<&str>, Input<&str>) = decimal.process("123".into()).unwrap();
        assert_eq!(parsed, "123");
        assert_eq!(state.as_input(), &"");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Input<&str>) =
            decimal.process("123.456".into()).unwrap();
        assert_eq!(parsed, "123");
        assert_eq!(state.as_input(), &".456");
        assert!(!state.is_err());

        let result: State<&str> = decimal.process("abc".into()).unwrap_err();
        assert!(result.is_err());
        assert_eq!(result.errors().len(), 1);
        assert_eq!(
            result.errors(),
            &Error::new(ErrorKind::expected(ExpectedError::Digit(10)), (0..1).into())
        );
    }

    #[test]
    fn can_parse_hex_digit() {
        let (state, parsed): (State<&str>, Input<&str>) =
            hexadecimal_digit.process("123".into()).unwrap();
        assert_eq!(parsed, "1");
        assert_eq!(state.as_input(), &"23");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Input<&str>) =
            hexadecimal_digit.process("123AbC.456".into()).unwrap();
        assert_eq!(parsed, "1");
        assert_eq!(state.as_input(), &"23AbC.456");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Input<&str>) =
            hexadecimal_digit.process("abc".into()).unwrap();
        assert_eq!(parsed, "a");
        assert_eq!(state.as_input(), &"bc");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Input<&str>) =
            hexadecimal_digit.process("BCD".into()).unwrap();
        assert_eq!(parsed, "B");
        assert_eq!(state.as_input(), &"CD");
        assert!(!state.is_err());

        let result: State<&str> = hexadecimal_digit.process("ghi".into()).unwrap_err();
        assert!(result.is_err());
        assert_eq!(result.errors().len(), 1);
        assert_eq!(
            result.errors(),
            &Error::new(ErrorKind::expected(ExpectedError::Digit(16)), (0..1).into())
        );
    }

    #[test]
    fn can_parse_hex_num() {
        let (state, process): (State<&str>, Input<&str>) =
            hexadecimal.process("123".into()).unwrap();
        assert_eq!(process, "123");
        assert_eq!(state.as_input(), &"");
        assert!(!state.is_err());

        let (state, process): (State<&str>, Input<&str>) =
            hexadecimal.process("123ABC.456".into()).unwrap();
        assert_eq!(process, "123ABC");
        assert_eq!(state.as_input(), &".456");
        assert!(!state.is_err());

        let state: State<&str> = hexadecimal.process("ghi".into()).unwrap_err();
        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors(),
            &Error::new(ErrorKind::expected(ExpectedError::Digit(16)), (0..1).into())
        );
    }
}
