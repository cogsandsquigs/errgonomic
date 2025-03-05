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
/// consumption it transparently emits the error.
///
/// NOTE: Will discard the consumed input. The state will also be at the end of the input, after
/// the consumed `until` parser's match.
///
/// ```
/// # use errgonomic::combinators::{panic_recover, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// let state: State<&str> = panic_recover(is("world"), is("world"))
///     .process("hellohellohelloworld!".into())
///     .unwrap_err();
/// assert_eq!(state.as_input().as_inner(), "!");
/// assert!(state.is_err());
/// ```
#[inline]
pub fn panic_recover<I, O1, O2, E, P1, P2>(mut p: P1, mut until: P2) -> impl Parser<I, O1, E>
where
    I: Underlying,
    E: CustomError,
    P1: Parser<I, O1, E>,
    P2: Parser<I, O2, E>,
{
    move |state: State<I, E>| -> Result<I, O1, E> {
        p.process(state).map_err(|state| {
            // NOTE: For some reason passing as a closure fixes type issues, and we can still use
            // `until` more than once.
            match take_until(|state| until.process(state)).process(state) {
                Ok((state, _)) => state,
                Err(state) => state,
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        combinators::is,
        parser::errors::{Error, ErrorKind, ExpectedError},
    };

    #[test]
    fn can_panic_recover() {
        let state: State<&str> = panic_recover(is("world"), is("world"))
            .process("hellohellohelloworld!".into())
            .unwrap_err();
        assert_eq!(state.as_input().as_inner(), "!");
        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::expected(ExpectedError::Is("world")),
                (0..1).into()
            )
        )
    }
}
