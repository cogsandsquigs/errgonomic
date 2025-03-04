use crate::parser::{
    errors::{CustomError, Result},
    input::{Input, Underlying},
    state::State,
    Parser,
};

use super::{between, maybe};

/// Parses an input if it is whitespace (of any length), including newlines (or carriage returns).
///
/// NOTE: Will error if the input is not whitespace.
///
/// ```
/// # use errgonomic::combinators::whitespace;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::errors::DummyError;
/// let (state, parsed) = whitespace::<_, DummyError>.process("  \t\nabc".into()).unwrap();
/// assert_eq!(parsed, "  \t\n");
/// assert_eq!(state.as_input().as_inner(), "abc");
/// ```
pub fn whitespace<I: Underlying, E: CustomError>(mut state: State<I, E>) -> Result<I, Input<I>, E> {
    let mut len = 0;
    let original_input = state.as_input().fork();
    let input = state.as_input_mut();
    while let Some(c) = input.peek() {
        if !c.is_ascii_whitespace() {
            break;
        }

        len += 1;
        input.next();
    }

    Ok((state, original_input.take(len)))
}

/// Parses an input if it is whitespace (of any length), but *not* newlines (or carriage returns).
///
/// NOTE: Will error if the input is not whitespace.
///
/// ```
/// # use errgonomic::combinators::whitespace_not_newline;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::errors::DummyError;
/// let (state, parsed) = whitespace_not_newline::<_, DummyError>.process("  \t\nabc".into()).unwrap();
/// assert_eq!(parsed, "  \t");
/// assert_eq!(state.as_input().as_inner(), "\nabc");
/// ```
pub fn whitespace_not_newline<I: Underlying, E: CustomError>(
    mut state: State<I, E>,
) -> Result<I, Input<I>, E> {
    let mut len = 0;
    let original_input = state.as_input().fork();
    let input = state.as_input_mut();
    while let Some(c) = input.peek() {
        if !c.is_ascii_whitespace()
            || c == b'\n'
            || (c == b'\r' && input.peek_nth(1) == Some(b'\n'))
        {
            break;
        }

        len += 1;
        input.next();
    }

    Ok((state, original_input.take(len)))
}

/// Parses an input if it a newline(s) (or carriage returns), but *not* whitespace.
///
/// NOTE: Will error if the input is not whitespace.
///
/// ```
/// # use errgonomic::combinators::newlines;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::errors::DummyError;
/// let (state, parsed) = newlines::<_, DummyError>.process("\n\r\n  \t\nabc".into()).unwrap();
/// assert_eq!(parsed, "\n\r\n");
/// assert_eq!(state.as_input().as_inner(), "  \t\nabc");
/// ```
pub fn newlines<I: Underlying, E: CustomError>(mut state: State<I, E>) -> Result<I, Input<I>, E> {
    let mut len = 0;
    let original_input = state.as_input().fork();
    let input = state.as_input_mut();
    while let Some(c) = input.peek() {
        println!("input.peek() = {:?}", c as char);
        println!(
            "input.peek_nth(1) = {:?}",
            input.peek_nth(1).unwrap() as char
        );
        if !(c == b'\n' || (c == b'\r' && input.peek_nth(1) == Some(b'\n'))) {
            break;
        }

        len += 1;
        input.next();
    }

    Ok((state, original_input.take(len)))
}

/// Parses an input wrapped in whitespace, on both ends.
///
/// NOTE: if there is no whitespace, it will *not* error.
///
/// ```
/// # use errgonomic::combinators::{whitespace_wrapped, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// let (state, parsed): (State<&str>, Input<&str>) = whitespace_wrapped(is("abc")).process("\n\r\n  \t\nabc\t\n    \r\nasdf".into()).unwrap();
/// assert_eq!(parsed, "abc");
/// assert_eq!(state.as_input().as_inner(), "asdf");
/// ```
pub fn whitespace_wrapped<I: Underlying, E: CustomError, P: Parser<I, O, E>, O>(
    p: P,
) -> impl Parser<I, O, E> {
    between(maybe(whitespace), p, maybe(whitespace))
}

/// Parses an input wrapped in whitespace (but not newlines/carriage returns), on both ends.
///
/// NOTE: if there is no whitespace, it will *not* error.
///
/// ```
/// # use errgonomic::combinators::{whitespace_not_newline_wrapped, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// let (state, parsed): (State<&str>, Input<&str>) = whitespace_not_newline_wrapped(is("abc")).process("  \tabc\t\n    \r\nasdf".into()).unwrap();
/// assert_eq!(parsed, "abc");
/// assert_eq!(state.as_input().as_inner(), "\n    \r\nasdf");
/// ```
pub fn whitespace_not_newline_wrapped<I: Underlying, E: CustomError, P: Parser<I, O, E>, O>(
    p: P,
) -> impl Parser<I, O, E> {
    between(
        maybe(whitespace_not_newline),
        p,
        maybe(whitespace_not_newline),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{combinators::is, parser::errors::Error};

    #[test]
    fn can_parse_whitespace() {
        let (state, parsed): (State<&str>, Input<&str>) =
            whitespace.process("  \t\n".into()).unwrap();
        assert_eq!(parsed, "  \t\n");
        assert!(!state.is_err());
        assert_eq!(state.as_input().as_inner(), "");

        let (state, parsed): (State<&str>, Input<&str>) =
            whitespace.process("  \t\nabc".into()).unwrap();
        assert_eq!(parsed, "  \t\n");
        assert!(!state.is_err());
        assert_eq!(state.as_input().as_inner(), "abc");
    }

    #[test]
    fn can_parse_whitespace_not_newline() {
        let (state, parsed): (State<&str>, Input<&str>) =
            whitespace_not_newline.process("  \t\n".into()).unwrap();
        assert_eq!(parsed, "  \t");
        assert!(!state.is_err());
        assert_eq!(state.as_input().as_inner(), "\n");
    }

    #[test]
    fn can_parse_newlines() {
        let (state, parsed): (State<&str>, Input<&str>) =
            newlines.process("\n\r\n  \t\n".into()).unwrap();
        assert_eq!(parsed, "\n\r\n");
        assert!(!state.is_err());
        assert_eq!(state.as_input().as_inner(), "  \t\n");
    }

    #[test]
    fn can_parse_whitespace_wrapped() {
        let (state, parsed): (State<&str>, Input<&str>) = whitespace_wrapped(is("abc"))
            .process("\n\r\n  \t\nabc\t\n    \r\nasdf".into())
            .unwrap();
        assert_eq!(parsed, "abc");
        assert!(!state.is_err());
        assert_eq!(state.as_input().as_inner(), "asdf");

        let (state, parsed): (State<&str>, Input<&str>) =
            whitespace_wrapped(is("abc")).process("abc".into()).unwrap();
        assert_eq!(parsed, "abc");
        assert!(!state.is_err());
        assert_eq!(state.as_input().as_inner(), "");
    }
    #[test]
    fn can_parse_whitespace_not_newline_wrapped() {
        let (state, parsed): (State<&str>, Input<&str>) = whitespace_not_newline_wrapped(is("abc"))
            .process("  \tabc\t\n    \r\nasdf".into())
            .unwrap();

        assert_eq!(parsed, "abc");
        assert!(!state.is_err());
        assert_eq!(state.as_input().as_inner(), "\n    \r\nasdf");

        let (state, parsed): (State<&str>, Input<&str>) = whitespace_not_newline_wrapped(is("abc"))
            .process("abc".into())
            .unwrap();

        assert_eq!(parsed, "abc");
        assert!(!state.is_err());
        assert_eq!(state.as_input().as_inner(), "");

        let state: State<&str> = whitespace_not_newline_wrapped(is("abc"))
            .process("\n\tabc  ".into())
            .unwrap_err();

        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors()[0],
            Error::Expected {
                expected: "abc",
                found: Input::new_with_span("\n\tabc  ", 0..1)
            }
        )
    }
}
