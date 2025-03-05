use crate::parser::{
    errors::{CustomError, Result},
    input::Underlying,
    state::State,
    Parser,
};

/// Parses as many of the given parser as possible, separated by `sep`. At the first error, returns
/// all the parsed output that happened before the error. If it errors out on the first parser, it
/// will return an empty list. The separator is dropped from the output.
///
/// If `ignore_dangling` is true, then the parser will ignore any dangling separators at the end of
/// the input, *and consume them*. If it is false, then it will return an error if there are any
/// dangling separators.
///
///```
/// # use errgonomic::combinators::{many, is, separated};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Vec<Input<&str>>) =
///     separated(is("hello"), is(","), true).process("hello,hello,hello, world!".into()).unwrap();
/// assert_eq!(parsed, vec!["hello", "hello", "hello"]);
/// assert_eq!(state.as_input().as_inner(), " world!");
///```
pub fn separated<
    I: Underlying,
    O1,
    O2,
    E: CustomError,
    P1: Parser<I, O1, E>,
    P2: Parser<I, O2, E>,
>(
    mut p: P1,
    mut sep: P2,
    ignore_dangling: bool,
) -> impl Parser<I, Vec<O1>, E> {
    move |state: State<I, E>| -> Result<I, Vec<O1>, E> {
        let mut results = Vec::new();
        let (mut state, o) = p.process(state)?;
        results.push(o);

        while let Ok((new_state, _)) = sep.process(state.fork()) {
            state = new_state;

            if ignore_dangling {
                if let Ok((new_state, o)) = p.process(state.fork()) {
                    state = new_state;
                    results.push(o);
                } else {
                    break;
                }
            } else {
                let (new_state, o) = p.process(state.fork())?;
                state = new_state;
                results.push(o);
            }
        }

        Ok((state, results))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::combinators::is;
    use crate::parser::errors::{Error, ErrorKind, ExpectedError};
    use crate::parser::input::Input;

    #[test]
    fn can_parse_separated() {
        let (state, parsed): (State<&str>, Vec<Input<&str>>) =
            separated(is("hello"), is(","), true)
                .process("hello,hello,hello, world!".into())
                .unwrap();
        assert_eq!(parsed, vec!["hello", "hello", "hello"]);
        assert_eq!(state.as_input().as_inner(), " world!");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Vec<Input<&str>>) =
            separated(is("hello"), is(","), true)
                .process("hello,hello,hello world!".into())
                .unwrap();
        assert_eq!(parsed, vec!["hello", "hello", "hello"]);
        assert_eq!(state.as_input().as_inner(), " world!");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Vec<Input<&str>>) =
            separated(is("hello"), is(","), false)
                .process("hello,hello,hello world!".into())
                .unwrap();
        assert_eq!(parsed, vec!["hello", "hello", "hello"]);
        assert_eq!(state.as_input().as_inner(), " world!");
        assert!(!state.is_err());

        let state: State<&str> = separated(is("hello"), is(","), false)
            .process("hello,hello,hello, world!".into())
            .unwrap_err();
        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(state.as_input(), &" world!");
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::expected(ExpectedError::Is("hello")),
                (18..19).into()
            )
        );
    }
}
