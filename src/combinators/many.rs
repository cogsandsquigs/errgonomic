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
}
