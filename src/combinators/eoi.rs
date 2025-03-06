use crate::parser::{
    errors::{CustomError, Error, ErrorKind, ExpectedError, Result},
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
    if state.as_input().is_empty() {
        Ok((state, ()))
    } else {
        let input = state.as_input().fork();
        Err(state.with_error(Error::new(
            ErrorKind::Expected(ExpectedError::Nothing),
            input,
        )))
    }
}

#[cfg(test)]
mod tests {

    use crate::parser::input::Input;

    use super::*;

    #[test]
    fn can_parse_eoi() {
        let (state, _): (State<&str>, ()) = eoi("".into()).unwrap();
        assert_eq!(state.as_input(), &"");
        assert!(!state.is_err());
        assert_eq!(state.errors().len(), 0);

        let state: State<&str> = super::eoi("a".into()).unwrap_err();
        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::Expected(ExpectedError::Nothing),
                Input::new_with_span("a", 0..1)
            )
        );
    }
}
