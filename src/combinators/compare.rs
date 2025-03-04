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
        let mut matches_input = Input::new(matches.fork());
        let input = state.as_input_mut();
        let original_input = input.fork();
        let mut matched_len = 0;

        while let Some(match_c) = matches_input.next() {
            if let Some(input_c) = input.peek() {
                if input_c != match_c {
                    return Err(state.with_error(Error::Expected {
                        expected: matches.fork(),
                        found: original_input.take(matched_len + 1),
                    }));
                }
            } else {
                return Err(state.with_error(Error::FoundEOI {
                    expected: matches.fork(),
                    eoi_at: original_input.skip(matched_len),
                }));
            }

            input.next(); // Update the input to the next character
            matched_len += 1; // ... and increment the matched length
        }

        Ok((state, original_input.take(matched_len)))
    }
}

/// Inverts the result of the parser. That is to say, if the parser is successful, it will return
/// an error with the output. If the parser is not successful, it will return the state as-is. If
/// the parser consumes any input, it will return the state before the input was consumed.
///
/// NOTE: When this returns an error, the state input is not consumed.
///
/// ```
/// # use errgonomic::combinators::{is, not};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// let (state, _): (State<&str>, ()) = not(is("st")).process("test".into()).unwrap();
/// assert_eq!(state.as_input().as_inner(), "test");
/// ```
pub fn not<I: Underlying, O, E: CustomError, P: Parser<I, O, E>>(
    mut p: P,
) -> impl Parser<I, (), E> {
    move |state: State<I, E>| match p.process(state.fork()) {
        Ok((new_state, _)) => {
            let found = state.as_input().fork().subtract(new_state.as_input());
            Err(state.with_error(Error::NotExpected { found }))
        }
        Err(_) => Ok((state, ())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{errors::DummyError, Parser};

    #[test]
    fn can_parse_with_is() {
        let (state, parsed): (State<&str>, Input<&str>) =
            is("test").process("test".into()).unwrap();
        assert_eq!(parsed, "test");
        assert_eq!(state.as_input(), &"");
        assert!(!state.is_err());
        assert_eq!(state.errors().len(), 0);

        let (state, parsed): (State<&str>, Input<&str>) =
            is("test").process("test123".into()).unwrap();
        assert_eq!(parsed, "test");
        assert_eq!(state.as_input(), &"123");
        assert!(!state.is_err());
        assert_eq!(state.errors().len(), 0);

        let result: Input<&str> = is::<_, DummyError>("test").parse("test123").unwrap();
        assert_eq!(result, "test");

        let state: State<&str> = is("test").process("123test".into()).unwrap_err();
        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors()[0],
            Error::Expected {
                expected: "test",
                found: Input::new_with_span("1", 0..1)
            }
        );

        let result: State<&str> = is("test").process("te".into()).unwrap_err();
        assert!(result.is_err());
        assert_eq!(result.errors().len(), 1);
        assert_eq!(
            result.errors()[0],
            Error::FoundEOI {
                expected: "test",
                eoi_at: Input::new_with_span("", 0..0)
            }
        );
    }

    #[test]
    fn can_parse_not() {
        let state: State<&str> = not(is("te")).process("test".into()).unwrap_err();
        assert_eq!(state.as_input(), &"test");
        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors()[0],
            Error::NotExpected {
                found: Input::new_with_span("test", 0..2)
            }
        );

        let (state, _): (State<&str>, _) = not(is("st")).process("test".into()).unwrap();
        assert_eq!(state.as_input(), &"test");
        assert!(!state.is_err());
    }
}
