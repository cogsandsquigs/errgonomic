use crate::parser::{
    errors::{CustomError, Error, Result},
    input::{Input, Underlying},
    state::State,
};

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
pub fn decimal<I: Underlying, E: CustomError>(mut state: State<I, E>) -> Result<I, Input<I>, E> {
    let mut len = 1;

    // Make sure that we have at least one digit.
    if !state.input.fork().take(len).is_decimal() {
        let found = state.input.fork().take(len);
        return Err(state.error(Error::ExpectedDec { found }));
    }

    loop {
        len += 1;
        let num = state.input.fork().take(len);

        if !num.is_decimal() {
            len -= 1;
            break;
        } else if len >= state.input.fork().len() {
            break;
        }
    }

    let num = state.input.fork().take(len);
    state.input = state.input.skip(len);
    Ok((state, num))
}

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
pub fn decimal_digit<I: Underlying, E: CustomError>(
    mut state: State<I, E>,
) -> Result<I, Input<I>, E> {
    let num = state.input.fork().take(1);

    if !num.is_decimal() {
        return Err(state.error(Error::ExpectedDec { found: num }));
    }

    state.input = state.input.fork().skip(1);
    Ok((state, num))
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
pub fn hexadecimal<I: Underlying, E: CustomError>(
    mut state: State<I, E>,
) -> Result<I, Input<I>, E> {
    let mut len = 1;

    // Make sure that we have at least one digit.
    if !state.input.fork().take(len).is_hex() {
        let found = state.input.fork().take(len);
        return Err(state.error(Error::ExpectedHex { found }));
    }

    loop {
        len += 1;
        let num = state.input.fork().take(len);

        if !num.is_hex() {
            len -= 1;
            break;
        } else if len >= state.input.len() {
            break;
        }
    }

    let num = state.input.fork().take(len);
    state.input = state.input.skip(len);
    Ok((state, num))
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
    mut state: State<I, E>,
) -> Result<I, Input<I>, E> {
    let num = state.input.fork().take(1);

    if !num.is_hex() {
        return Err(state.error(Error::ExpectedHex { found: num }));
    }

    state.input = state.input.fork().skip(1);
    Ok((state, num))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn can_parse_dec_num() {
        let result: (State<&str>, Input<&str>) = decimal.process("123".into()).unwrap();
        assert_eq!(result.1, "123");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.input, "");

        let result: (State<&str>, Input<&str>) = decimal.process("123.456".into()).unwrap();
        assert_eq!(result.1, "123");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.input, ".456");

        let result: State<&str> = decimal.process("abc".into()).unwrap_err();
        assert!(result.errors().any_errs());
        assert_eq!(result.errors().num_errors(), 1);
        assert_eq!(
            result.errors().errors()[0],
            Error::ExpectedDec {
                found: Input::new_with_span("abc", (0..1).into())
            }
        );
    }

    #[test]
    fn can_parse_hex_num() {
        let result: (State<&str>, Input<&str>) = hexadecimal.process("123".into()).unwrap();
        assert_eq!(result.1, "123");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.input, "");

        let result: (State<&str>, Input<&str>) = hexadecimal.process("123ABC.456".into()).unwrap();
        assert_eq!(result.1, "123ABC");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.input, ".456");

        let result: State<&str> = hexadecimal.process("ghi".into()).unwrap_err();
        assert!(result.errors().any_errs());
        assert_eq!(result.errors().num_errors(), 1);
        assert_eq!(
            result.errors().errors()[0],
            Error::ExpectedHex {
                found: Input::new_with_span("ghi", (0..1).into())
            }
        );
    }
}
