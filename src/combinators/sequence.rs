use crate::parser::{errors::Result, input::Underlying, state::State, Parser};

/// Parses any of the given parsers. The first parser that succeeds will be the output. Otherwise,
/// if none of the parsers succeed, the error from the last parser will be returned.
pub fn any<I: Underlying, O, L: List<I, O>>(mut ps: L) -> impl Parser<I, O> {
    move |state| ps.any(state)
}

/// Parses as many of the given parser as possible. At the first error, returns all the parsed
/// output that happened before the error. If it errors out on the first parser, it will return
/// an empty list.
pub fn many<I: Underlying, O, P: Parser<I, O>>(mut p: P) -> impl Parser<I, Vec<O>> {
    move |mut state: State<I>| -> Result<I, Vec<O>> {
        let mut results = Vec::new();

        while let Ok((new_state, o)) = p.process(state.fork()) {
            state = new_state;
            results.push(o);
        }

        Ok((state, results))
    }
}

/// Parses until a specific parser matches. The parser that is found will be included in the
/// output. If an error occurs before the parser is found, the errored state will be returned. If
/// the `until` parser matches right away, an empty list will be returned.
pub fn many_until<I: Underlying, O1, O2, P1: Parser<I, O1>, P2: Parser<I, O2>>(
    mut p: P1,
    mut until: P2,
) -> impl Parser<I, (Vec<O1>, O2)> {
    move |mut state: State<I>| -> Result<I, (Vec<O1>, O2)> {
        let mut results = Vec::new();

        loop {
            match until.process(state.fork()) {
                Err(_) => {
                    let (new_state, o) = p.process(state.fork())?;
                    state = new_state;
                    results.push(o);
                }
                Ok((new_state, o)) => return Ok((new_state, (results, o))),
            }
        }
    }
}

/// Parses a parser between two other parsers. The first parser is the opening parser, the second
/// is the parser to parse, and the third is the closing parser. The opening and closing parsers'
/// outputs are ignored.
pub fn between<I: Underlying, O1, O2, O3, P1, P2, P3>(
    open: P1,
    parser: P2,
    close: P3,
) -> impl Parser<I, O2>
where
    P1: Parser<I, O1>,
    P2: Parser<I, O2>,
    P3: Parser<I, O3>,
{
    open.then(parser).then(close).map(|((_, o), _)| o)
}

/* TRAIT IMPLEMENTATIONS NEEDED FOR ANY */
/* These are annoying and long, you can ignore*/

pub trait List<I: Underlying, O> {
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

    #[test]
    fn can_parse_between() {
        let result: (State<&str>, Input<&str>) = between(is("test"), is("123"), is("456"))
            .process("test123456789".into())
            .unwrap();

        assert_eq!(result.1, "123");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "789");
    }

    #[test]
    fn can_parse_many_once() {
        let result: (State<&str>, Vec<Input<&str>>) =
            super::many(is("test")).process("test".into()).unwrap();

        assert_eq!(result.1.len(), 1);
        assert_eq!(result.1[0], "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.input, "");
    }

    #[test]
    fn can_parse_many() {
        let result: (State<&str>, Vec<Input<&str>>) = super::many(is("test"))
            .process("testtest123".into())
            .unwrap();

        assert_eq!(result.1.len(), 2);
        assert_eq!(result.1[0], "test");
        assert_eq!(result.1[1], "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.input, "123");
    }

    #[test]
    fn can_parse_many_until() {
        let result = super::many_until(is("test"), is("123"))
            .process("testtest123".into())
            .unwrap();

        assert_eq!(result.1 .0.len(), 2);
        assert_eq!(result.1 .0[0], "test");
        assert_eq!(result.1 .0[1], "test");
        assert_eq!(result.1 .1, "123");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.input, "");
    }
}
