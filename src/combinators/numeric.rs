use crate::parser::{
    errors::{Error, Result},
    input::{Input, Underlying},
    state::State,
};

/// Parses a decimal number until it stops. If there is no decimal number, returns an error.
pub fn decimal<I: Underlying>(mut state: State<I>) -> Result<I, Input<I>> {
    let mut len = 1;

    // Make sure that we have at least one digit.
    if !state.input.take(len).is_decimal() {
        let found = state.input.take(len);
        return Err(state.error(Error::ExpectedDecNumber { found }));
    }

    loop {
        len += 1;
        let num = state.input.take(len);

        if !num.is_decimal() {
            len -= 1;
            break;
        } else if len >= state.input.len() {
            break;
        }
    }

    let num = state.input.take(len);
    state.input = state.input.skip(len);
    Ok((state, num))
}

/// Parses a hexadecimal number until it stops. If there is no hexadecimal number, returns an
/// error.
pub fn hexadecimal<I: Underlying>(mut state: State<I>) -> Result<I, Input<I>> {
    let mut len = 1;

    // Make sure that we have at least one digit.
    if !state.input.take(len).is_hex() {
        let found = state.input.take(len);
        return Err(state.error(Error::ExpectedHexNumber { found }));
    }

    loop {
        len += 1;
        let num = state.input.take(len);

        if !num.is_hex() {
            len -= 1;
            break;
        } else if len >= state.input.len() {
            break;
        }
    }

    let num = state.input.take(len);
    state.input = state.input.skip(len);
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
            Error::ExpectedDecNumber {
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
            Error::ExpectedHexNumber {
                found: Input::new_with_span("ghi", (0..1).into())
            }
        );
    }
}
