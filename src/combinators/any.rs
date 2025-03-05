use crate::parser::{
    errors::{CustomError, Error, ErrorKind, Result},
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
#[inline]
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
    #[inline]
    fn any(&mut self, state: State<I, E>) -> Result<I, O, E> {
        let mut errs: Vec<Error<I, E>> = vec![];

        self.0
            .process(state.fork())
            .or_else(|e| {
                errs.push(e.errors().clone()); // TODO: Clone is bad, but don't know how to fix
                self.1.process(state.fork())
            })
            .map_err(|e| {
                errs.push(e.errors().clone());
                let span = errs
                    .iter()
                    .map(|e| e.span())
                    .fold(e.as_input().span(), |acc, x| acc.union_between(x));
                state.with_error(Error::new(ErrorKind::all(errs), span))
            })
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
    #[inline]
    fn any(&mut self, state: State<I, E>) -> Result<I, O, E> {
        let mut errs: Vec<Error<I, E>> = vec![];

        self.0
            .process(state.fork())
            .or_else(|e| {
                errs.push(e.errors().clone()); // TODO: Clone is bad, but don't know how to fix
                self.1.process(state.fork())
            })
            .or_else(|e| {
                errs.push(e.errors().clone()); // TODO: Clone is bad, but don't know how to fix
                self.2.process(state.fork())
            })
            .map_err(|e| {
                errs.push(e.errors().clone());
                let span = errs
                    .iter()
                    .map(|e| e.span())
                    .fold(e.as_input().span(), |acc, x| acc.union_between(x));
                state.with_error(Error::new(ErrorKind::all(errs), span))
            })
    }
}

impl<I, O, E, P1, P2, P3, P4> List<I, O, E> for (P1, P2, P3, P4)
where
    I: Underlying,
    E: CustomError,
    P1: Parser<I, O, E>,
    P2: Parser<I, O, E>,
    P3: Parser<I, O, E>,
    P4: Parser<I, O, E>,
{
    #[inline]
    fn any(&mut self, state: State<I, E>) -> Result<I, O, E> {
        let mut errs: Vec<Error<I, E>> = vec![];

        self.0
            .process(state.fork())
            .or_else(|e| {
                errs.push(e.errors().clone()); // TODO: Clone is bad, but don't know how to fix
                self.1.process(state.fork())
            })
            .or_else(|e| {
                errs.push(e.errors().clone()); // TODO: Clone is bad, but don't know how to fix
                self.2.process(state.fork())
            })
            .or_else(|e| {
                errs.push(e.errors().clone()); // TODO: Clone is bad, but don't know how to fix
                self.3.process(state.fork())
            })
            .map_err(|e| {
                errs.push(e.errors().clone());
                let span = errs
                    .iter()
                    .map(|e| e.span())
                    .fold(e.as_input().span(), |acc, x| acc.union_between(x));
                state.with_error(Error::new(ErrorKind::all(errs), span))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        combinators::{id, is},
        parser::{
            errors::{DummyError, Error, ExpectedError},
            input::Input,
        },
    };

    #[test]
    fn can_parse_any() {
        let (state, parsed): (State<&str>, Input<&str>) = any((is("x"), is("test")))
            .process("test123".into())
            .unwrap();
        assert_eq!(parsed, "test");
        assert_eq!(state.as_input(), &"123");
        assert!(!state.is_err());

        let parsed: Input<&str> = any((id::<_, DummyError>, is("test")))
            .parse("test123")
            .unwrap();
        assert_eq!(parsed, "test123");

        let state: State<&str> = any((is("done"), is("test")))
            .process("123test".into())
            .unwrap_err();
        assert!(state.is_err());
        println!("{:#?}", state.errors());
        assert_eq!(state.errors().len(), 2);
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::All(vec![
                    Error::new(
                        ErrorKind::Expected(ExpectedError::Is("done")),
                        (0..1).into()
                    ),
                    Error::new(
                        ErrorKind::Expected(ExpectedError::Is("test")),
                        (0..1).into()
                    ),
                ]),
                (0..7).into()
            )
        );
    }
}
