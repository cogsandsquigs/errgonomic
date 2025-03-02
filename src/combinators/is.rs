use crate::parser::{
    errors::{CustomError, Error},
    input::{Input, Underlying},
    state::State,
    Parser,
};

/// Parses an input if it matches the given input. If it does, it returns the input.
/// If not, it errors out.
///
/// NOTE: This only matches up to the length of the matching string. If there is more input
/// after the matching string, it will be left in the parser state.
///
/// ```
/// # use errgonomic::combinators::is;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// let (state, parsed): (State<&str>, Input<&str>) = is("te").process("test".into()).unwrap();
/// assert_eq!(parsed, "te");
/// assert_eq!(state.as_input().as_inner(), "st");
/// ```
pub fn is<I: Underlying, E: CustomError>(matches: I) -> impl Parser<I, Input<I>, E> {
    move |mut state: State<I, E>| {
        if state.input.len() < matches.len() {
            state.input = state.input.skip(state.input.len());
            let input = state.input.fork();
            return Err(state.error(Error::FoundEOI {
                expected: matches.clone(),
                eoi_at: input,
            }));
        }

        let grabbed = state.input.take(matches.len());

        if grabbed == matches {
            state.input = state.input.skip(matches.len());
            Ok((state, grabbed))
        } else {
            Err(state.error(Error::Expected {
                expected: matches.clone(),
                found: grabbed,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_with_is() {
        let result: (State<&str>, Input<&str>) = is("test").process("test".into()).unwrap();
        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "");

        let result: (State<&str>, Input<&str>) = is("test").process("test123".into()).unwrap();
        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "123");

        let result: Input<&str> = is::<_, ()>("test").parse("test123").unwrap();
        assert_eq!(result, "test");

        let result: State<&str> = is("test").process("123test".into()).unwrap_err();
        assert!(result.errors().any_errs());
        assert_eq!(result.errors().num_errors(), 1);
        assert_eq!(result.errors().errors().len(), 1);
        assert_eq!(
            result.errors().errors()[0],
            crate::parser::errors::Error::Expected {
                expected: "test",
                found: Input::new_with_span("123test", (0..4).into())
            }
        );

        let result: State<&str> = is("test").process("te".into()).unwrap_err();
        assert!(result.errors().any_errs());
        assert_eq!(result.errors().num_errors(), 1);
        assert_eq!(result.errors().errors().len(), 1);
        assert_eq!(
            result.errors().errors()[0],
            crate::parser::errors::Error::FoundEOI {
                expected: "test",
                eoi_at: Input::new_with_span("te", (2..2).into())
            }
        );
    }
}
