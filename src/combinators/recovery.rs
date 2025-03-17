use crate::parser::{
    errors::{CustomError, Result},
    input::Underlying,
    state::State,
    Parser,
};

use super::take_until;

/// Recovers via "panic-mode" recovery. It's not what you think! Instead, when an error is
/// encountered in the parser `p`, it will try to recover by consuming input until it finds a
/// match on some thing, in this case the parser `until`. If no error occurs, then nothing is
/// consumed and parsing continues as normal. If an error is encountered, then after the
/// consumption it will also return `Ok`, but *still* include the error in the state! (This is why
/// we have `State` as the output for the `Err` case of `Result`).
///
/// NOTE: Will discard the consumed input. The state will also be at the end of the input, after
/// the consumed `until` parser's match.
///
/// ```
/// # use errgonomic::combinators::{panic_recover, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Option<Input<&str>>) = panic_recover(is("world"), is("world"))
///     .process("hellohellohelloworld!".into())
///     .unwrap();
/// assert!(parsed.is_none());
/// assert_eq!(state.as_input().as_inner(), "!");
/// assert!(state.is_err());
/// ```
#[inline]
pub fn panic_recover<I, O1, O2, E, P1, P2>(p: P1, until: P2) -> impl Parser<I, Option<O1>, E>
where
    I: Underlying,
    E: CustomError,
    P1: Parser<I, O1, E>,
    P2: Parser<I, O2, E>,
{
    move |state: State<I, E>| -> Result<I, Option<O1>, E> {
        match p.process(state) {
            Ok((state, o)) => Ok((state, Some(o))),
            Err(state) => match take_until(|s| until.process(s)).process(state) {
                Ok((state, _)) => Ok((state, None)),
                Err(state) => Err(state), // Something weird happened and we should tell someone
                                          // about it!
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        combinators::{eoi, is},
        parser::{
            errors::{Error, ErrorKind, ExpectedError},
            input::Input,
        },
    };

    #[test]
    fn can_panic_recover() {
        let (state, parsed): (State<&str>, Option<Input<&str>>) =
            panic_recover(is("world"), is("world"))
                .process("hellohellohelloworld!".into())
                .unwrap();
        assert!(parsed.is_none());
        assert_eq!(state.as_input().as_inner(), "!");
        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::expected(ExpectedError::Is("world")),
                Input::new_with_span("hellohellohelloworld!", 0..1)
            )
        );

        let (state, parsed): (State<&str>, Option<Input<&str>>) =
            panic_recover(is("world"), is("world"))
                .process("worldhellohelloworld!".into())
                .unwrap();
        assert_eq!(parsed.unwrap().as_inner(), "world");
        assert_eq!(state.as_input().as_inner(), "hellohelloworld!");
        assert!(!state.is_err());
        assert_eq!(state.errors().len(), 0);
    }

    #[test]
    fn can_panic_recoverto_eoi() {
        let (state, parsed): (State<&str>, Option<Input<&str>>) = panic_recover(is("world"), eoi)
            .process("hellohellohelloworld!".into())
            .unwrap();
        assert!(parsed.is_none());
        assert_eq!(state.as_input().as_inner(), "");
        assert!(state.is_err());
        println!("{:#?}", state.errors());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::expected(ExpectedError::Is("world")),
                Input::new_with_span("hellohellohelloworld!", 0..1)
            )
        );

        let (state, parsed): (State<&str>, Option<Input<&str>>) = panic_recover(is("world"), eoi)
            .process("worldhellohelloworld!".into())
            .unwrap();
        assert_eq!(parsed.unwrap().as_inner(), "world");
        assert_eq!(state.as_input().as_inner(), "hellohelloworld!");
        assert!(!state.is_err());
        assert_eq!(state.errors().len(), 0);
    }
}
