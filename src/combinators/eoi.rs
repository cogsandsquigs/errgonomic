use crate::parser::{
    errors::{CustomError, Error, Result},
    input::Underlying,
    state::State,
};

/// Parses an end of input.
/// ```
/// # use errgonomic::combinators::eoi;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::errors::DummyError;
/// assert_eq!(eoi::<_, DummyError>.parse("").unwrap(), ());
/// ```
pub fn eoi<I: Underlying, E: CustomError>(state: State<I, E>) -> Result<I, (), E> {
    if state.input.is_empty() {
        Ok((state, ()))
    } else {
        let input = state.input.fork();
        Err(state.error(Error::ExpectedEOI { found: input }))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::input::Input;

    use super::*;

    #[test]
    fn can_parse_eoi() {
        let result: (State<&str>, ()) = super::eoi("".into()).unwrap();
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "");

        let result: State<&str> = super::eoi("a".into()).unwrap_err();
        assert!(result.errors().any_errs());
        assert_eq!(result.errors().num_errors(), 1);
        assert_eq!(result.errors().errors().len(), 1);
        assert_eq!(
            result.errors().errors()[0],
            Error::ExpectedEOI {
                found: Input::new_with_span("a", (0..1).into())
            }
        );
    }
}
