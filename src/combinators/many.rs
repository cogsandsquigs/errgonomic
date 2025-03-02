use crate::parser::{errors::Result, input::Underlying, state::State, Parser};

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
