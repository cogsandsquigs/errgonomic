use crate::parser::{
    errors::{CustomError, Result},
    input::Underlying,
    state::State,
    Parser,
};

/// Parses as many of the given parser as possible. At the first error, returns all the parsed
/// output that happened before the error. If it errors out on the first parser, it will return
/// an empty list.
///```
/// # use errgonomic::combinators::{many, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Vec<Input<&str>>) = many(is("hello")).process("hellohellohello, world!".into()).unwrap();
/// assert_eq!(parsed, vec!["hello", "hello", "hello"]);
/// assert_eq!(state.as_input().as_inner(), ", world!");
///```
pub fn many<I: Underlying, O, E: CustomError, P: Parser<I, O, E>>(
    mut p: P,
) -> impl Parser<I, Vec<O>, E> {
    move |mut state: State<I, E>| -> Result<I, Vec<O>, E> {
        let mut results = Vec::new();

        while let Ok((new_state, o)) = p.process(state.fork()) {
            state = new_state;
            results.push(o);
        }

        Ok((state, results))
    }
}

/// Parses `n` of the given parser as possible. At the first error, returns all the parsed
/// output that happened before the error. If it errors out before `n`, an error will be returned.
///```
/// # use errgonomic::combinators::{many_n, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::errors::DummyError;
/// let (state, parsed): (State<&str>, Vec<Input<&str>>) = many_n(1, is("hello")).process("hellohello, world!".into()).unwrap();
/// assert_eq!(parsed, vec!["hello"]);
/// assert_eq!(state.as_input().as_inner(), "hello, world!");
///
/// let (state, parsed) = many_n(2, is::<_, DummyError>("hello")).process("hellohello, world!".into()).unwrap();
/// assert_eq!(parsed, vec!["hello", "hello"]);
/// assert_eq!(state.as_input().as_inner(), ", world!");
///
/// let state = many_n(3, is::<_, DummyError>("hello")).process("hellohello, world!".into()).unwrap_err();
/// assert!(state.errors().any_errs());
/// assert_eq!(state.as_input().as_inner(), ", world!");
///```
pub fn many_n<I: Underlying, O, E: CustomError, P: Parser<I, O, E>>(
    n: usize,
    mut p: P,
) -> impl Parser<I, Vec<O>, E> {
    move |mut state: State<I, E>| -> Result<I, Vec<O>, E> {
        let mut results = Vec::new();

        for _ in 0..n {
            match p.process(state.fork()) {
                Ok((new_state, o)) => {
                    state = new_state;
                    results.push(o);
                }
                Err(e) => return Err(e),
            }
        }

        Ok((state, results))
    }
}

/// Parses between `m` and  `n` of the given parser as possible. At the first error, returns all
/// the parsed output that happened before the error. If it errors out before `n`, an error will
/// be returned.
///```
/// # use errgonomic::combinators::{many_m_n, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::errors::DummyError;
/// let state = many_m_n(1, 2, is::<_, DummyError>("hello")).process(", world!".into()).unwrap_err();
/// assert!(state.errors().any_errs());
/// assert_eq!(state.as_input().as_inner(), ", world!");
///
/// let (state, parsed) = many_m_n(1, 2, is::<_, DummyError>("hello")).process("hello, world!".into()).unwrap();
/// assert_eq!(parsed, vec!["hello"]);
/// assert_eq!(state.as_input().as_inner(), ", world!");
///
/// let (state, parsed) = many_m_n(1, 2, is::<_, DummyError>("hello")).process("hellohello, world!".into()).unwrap();
/// assert_eq!(parsed, vec!["hello", "hello"]);
/// assert_eq!(state.as_input().as_inner(), ", world!");
///
/// let (state, parsed) = many_m_n(1, 2, is::<_, DummyError>("hello")).process("hellohellohello, world!".into()).unwrap();
/// assert_eq!(parsed, vec!["hello", "hello"]);
/// assert_eq!(state.as_input().as_inner(), "hello, world!");
///```
pub fn many_m_n<I: Underlying, O, E: CustomError, P: Parser<I, O, E>>(
    m: usize,
    n: usize,
    mut p: P,
) -> impl Parser<I, Vec<O>, E> {
    move |mut state: State<I, E>| -> Result<I, Vec<O>, E> {
        let mut results = Vec::new();

        for _ in 0..m {
            match p.process(state.fork()) {
                Ok((new_state, o)) => {
                    state = new_state;
                    results.push(o);
                }
                Err(e) => return Err(e),
            }
        }

        for _ in m..n {
            match p.process(state.fork()) {
                Ok((new_state, o)) => {
                    state = new_state;
                    results.push(o);
                }
                Err(_) => break,
            }
        }

        Ok((state, results))
    }
}

/// Parses until a specific parser matches. The parser that is found will be included in the
/// output. If an error occurs before the parser is found, the errored state will be returned. If
/// the `until` parser matches right away, an empty list will be returned.
///```
/// # use errgonomic::combinators::{many_until, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, (Vec<Input<&str>>, Input<&str>))
///     = many_until(is("hello"), is(", world!")).process("hellohellohello, world! Hi!".into()).unwrap();
/// assert_eq!(parsed.0, vec!["hello", "hello", "hello"]);
/// assert_eq!(parsed.1, ", world!");
/// assert_eq!(state.as_input().as_inner(), " Hi!");
///```
pub fn many_until<
    I: Underlying,
    O1,
    O2,
    E: CustomError,
    P1: Parser<I, O1, E>,
    P2: Parser<I, O2, E>,
>(
    mut p: P1,
    mut until: P2,
) -> impl Parser<I, (Vec<O1>, O2), E> {
    move |mut state: State<I, E>| -> Result<I, (Vec<O1>, O2), E> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{combinators::is, parser::input::Input};

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
    fn can_parse_many_n() {
        let (state, parsed): (State<&str>, Vec<Input<&str>>) =
            many_n(1, is("test")).process("testtest123".into()).unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], "test");
        assert!(!state.errors().any_errs());
        assert_eq!(state.input, "test123");

        let (state, parsed): (State<&str>, Vec<Input<&str>>) =
            many_n(2, is("test")).process("testtest123".into()).unwrap();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], "test");
        assert_eq!(parsed[1], "test");
        assert!(!state.errors().any_errs());
        assert_eq!(state.input, "123");

        let state: State<&str> = super::many_n(3, is("test"))
            .process("testtest123".into())
            .unwrap_err();

        assert!(state.errors().any_errs());
        assert_eq!(state.errors().num_errors(), 1);
        assert_eq!(
            state.errors().errors()[0],
            crate::parser::errors::Error::FoundEOI {
                expected: "test",
                eoi_at: Input::new_with_span("testtest123", (11..11).into())
            }
        );
    }

    #[test]
    fn can_parse_many_m_n() {
        let state: State<&str> = many_m_n(1, 2, is("hello"))
            .process(", world!".into())
            .unwrap_err();
        assert!(state.errors().any_errs());
        assert_eq!(state.as_input().as_inner(), ", world!");
        assert_eq!(
            state.errors().errors()[0],
            crate::parser::errors::Error::Expected {
                expected: "hello",
                found: Input::new_with_span(", world!", (0..5).into())
            }
        );

        let (state, parsed): (State<&str>, Vec<Input<&str>>) = many_m_n(1, 2, is("hello"))
            .process("hello, world!".into())
            .unwrap();
        assert_eq!(parsed, vec!["hello"]);
        assert!(!state.errors().any_errs());
        assert_eq!(state.as_input().as_inner(), ", world!");

        let (state, parsed): (State<&str>, Vec<Input<&str>>) = many_m_n(1, 2, is("hello"))
            .process("hellohello, world!".into())
            .unwrap();
        assert_eq!(parsed, vec!["hello", "hello"]);
        assert!(!state.errors().any_errs());
        assert_eq!(state.as_input().as_inner(), ", world!");

        let (state, parsed): (State<&str>, Vec<Input<&str>>) = many_m_n(1, 2, is("hello"))
            .process("hellohellohello, world!".into())
            .unwrap();
        assert_eq!(parsed, vec!["hello", "hello"]);
        assert!(!state.errors().any_errs());
        assert_eq!(state.as_input().as_inner(), "hello, world!");
    }

    #[test]
    fn can_parse_many_until() {
        let result: (State<&str>, (_, _)) = many_until(is("test"), is("123"))
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
