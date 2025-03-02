use crate::parser::{
    errors::{CustomError, Result},
    input::Underlying,
    state::State,
    Parser,
};

/// Parses any of the given parsers. The first parser that succeeds will be the output. Otherwise,
/// if none of the parsers succeed, the error from the last parser will be returned.
///```
/// # use errgonomic::combinators::{any, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// let (state, parsed): (State<&str>, Input<&str>) = any((is("hello"), is("world"))).process("hello, world!".into()).unwrap();
/// assert_eq!(parsed, "hello");
/// assert_eq!(state.as_input().as_inner(), ", world!");
///```
#[allow(private_bounds)]
pub fn any<I: Underlying, O, E: CustomError, L: List<I, O, E>>(mut ps: L) -> impl Parser<I, O, E> {
    move |state| ps.any(state)
}

/* TRAIT IMPLEMENTATIONS NEEDED FOR ANY */
/* These are annoying and long, you can ignore*/

trait List<I: Underlying, O, E: CustomError> {
    fn any(&mut self, state: State<I, E>) -> Result<I, O, E>;
}

impl<I, O, E, P1, P2> List<I, O, E> for (P1, P2)
where
    I: Underlying,
    E: CustomError,
    P1: Parser<I, O, E>,
    P2: Parser<I, O, E>,
{
    fn any(&mut self, state: State<I, E>) -> Result<I, O, E> {
        self.0
            .process(state.fork())
            .or_else(|_| self.1.process(state))
    }
}

impl<I, O, E, P1, P2, P3> List<I, O, E> for (P1, P2, P3)
where
    I: Underlying,
    E: CustomError,
    P1: Parser<I, O, E>,
    P2: Parser<I, O, E>,
    P3: Parser<I, O, E>,
{
    fn any(&mut self, state: State<I, E>) -> Result<I, O, E> {
        self.0
            .process(state.fork())
            .or_else(|_| self.1.process(state.fork()))
            .or_else(|_| self.2.process(state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        combinators::{id, is},
        parser::input::Input,
    };

    #[test]
    fn can_parse_any() {
        let result: (State<&str>, Input<&str>) = any::<_, _, (), _>((is("x"), is("test")))
            .process("test123".into())
            .unwrap();
        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "123");

        let result: Input<&str> = any::<_, _, (), _>((id, is("test")))
            .parse("test123")
            .unwrap();
        assert_eq!(result, "test123");

        let result: State<&str> = any::<_, _, (), _>((is("done"), is("test")))
            .process("123test".into())
            .unwrap_err();
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
    }
}
