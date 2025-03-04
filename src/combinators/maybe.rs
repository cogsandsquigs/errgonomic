use crate::parser::{errors::CustomError, input::Underlying, state::State, Parser};

/// Parses an input whether or not the parser is successful. If the parser is successful, the
/// output is given as `Some(output)`. If the parser is not successful, the output is `None`.
/// ```
/// # use errgonomic::combinators::{maybe, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::errors::DummyError;
/// let parsed = maybe(is::<_, DummyError>("te")).parse("test").unwrap();
/// assert_eq!(parsed.unwrap(), "te");
///
/// let parsed = maybe(is::<_, DummyError>("st")).parse("test").unwrap();
/// assert_eq!(parsed, None);
/// ```
pub fn maybe<I: Underlying, O, E: CustomError, P: Parser<I, O, E>>(
    mut p: P,
) -> impl Parser<I, Option<O>, E> {
    move |state: State<I, E>| match p.process(state.fork()) {
        Ok((new_state, o)) => Ok((new_state, Some(o))),
        Err(_) => Ok((state, None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{combinators::is, parser::input::Input};

    #[test]
    fn can_parse_maybe() {
        let (state, parsed): (State<&str>, Option<Input<&str>>) =
            maybe(is("te")).process("test".into()).unwrap();
        assert_eq!(parsed.unwrap(), "te");
        assert_eq!(state.as_input().as_inner(), "st");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Option<Input<&str>>) =
            maybe(is("st")).process("test".into()).unwrap();
        assert_eq!(parsed, None);
        assert_eq!(state.as_input().as_inner(), "test");
        assert!(!state.is_err());
    }
}
