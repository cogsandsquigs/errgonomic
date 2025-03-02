use crate::parser::{errors::Result, input::Underlying, state::State, Parser};

/// Parses any of the given parsers. The first parser that succeeds will be the output. Otherwise,
/// if none of the parsers succeed, the error from the last parser will be returned.
#[allow(private_bounds)]
pub fn any<I: Underlying, O, L: List<I, O>>(mut ps: L) -> impl Parser<I, O> {
    move |state| ps.any(state)
}

/* TRAIT IMPLEMENTATIONS NEEDED FOR ANY */
/* These are annoying and long, you can ignore*/

trait List<I: Underlying, O> {
    fn any(&mut self, state: State<I>) -> Result<I, O>;
}

impl<I, O, P> List<I, O> for (P,)
where
    I: Underlying,
    P: Parser<I, O>,
{
    fn any(&mut self, state: State<I>) -> Result<I, O> {
        self.0.process(state)
    }
}

impl<I, O, P1, P2> List<I, O> for (P1, P2)
where
    I: Underlying,
    P1: Parser<I, O>,
    P2: Parser<I, O>,
{
    fn any(&mut self, state: State<I>) -> Result<I, O> {
        self.0
            .process(state.fork())
            .or_else(|_| self.1.process(state))
    }
}

impl<I, O, P1, P2, P3> List<I, O> for (P1, P2, P3)
where
    I: Underlying,
    P1: Parser<I, O>,
    P2: Parser<I, O>,
    P3: Parser<I, O>,
{
    fn any(&mut self, state: State<I>) -> Result<I, O> {
        self.0
            .process(state.fork())
            .or_else(|_| self.1.process(state.fork()))
            .or_else(|_| self.2.process(state))
    }
}

impl<I, O, P1, P2, P3, P4> List<I, O> for (P1, P2, P3, P4)
where
    I: Underlying,
    P1: Parser<I, O>,
    P2: Parser<I, O>,
    P3: Parser<I, O>,
    P4: Parser<I, O>,
{
    fn any(&mut self, state: State<I>) -> Result<I, O> {
        self.0
            .process(state.fork())
            .or_else(|_| self.1.process(state.fork()))
            .or_else(|_| self.2.process(state.fork()))
            .or_else(|_| self.3.process(state))
    }
}

impl<I, O, P1, P2, P3, P4, P5> List<I, O> for (P1, P2, P3, P4, P5)
where
    I: Underlying,
    P1: Parser<I, O>,
    P2: Parser<I, O>,
    P3: Parser<I, O>,
    P4: Parser<I, O>,
    P5: Parser<I, O>,
{
    fn any(&mut self, state: State<I>) -> Result<I, O> {
        self.0
            .process(state.fork())
            .or_else(|_| self.1.process(state.fork()))
            .or_else(|_| self.2.process(state.fork()))
            .or_else(|_| self.3.process(state.fork()))
            .or_else(|_| self.4.process(state))
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
        let result: (State<&str>, Input<&str>) = any((is("test"),)).process("test".into()).unwrap();

        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "");

        let result: (State<&str>, Input<&str>) = any((is("x"), is("test")))
            .process("test123".into())
            .unwrap();
        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "123");

        let result: Input<&str> = any((id, is("test"))).parse("test123").unwrap();
        assert_eq!(result, "test123");

        let result: State<&str> = any((is("test"),)).process("123test".into()).unwrap_err();
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
