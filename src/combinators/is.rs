use crate::parser::{
    errors::{Error, Result},
    input::{Input, Underlying},
    state::State,
    Parser,
};

/// Parses an input if it matches the given input. If it does, it returns the input.
/// If not, it errors out.
/// NOTE: This only matches up to the length of the matching string. If there is more input
/// after the matching string, it will be left in the parser state.
pub fn is<I: Underlying, S: Into<I>>(matches: S) -> Is<I> {
    Is {
        matches: matches.into(),
        _1: core::marker::PhantomData,
    }
}

pub struct Is<I: Underlying> {
    matches: I,
    _1: core::marker::PhantomData<I>,
}

impl<I: Underlying> Parser<I, Input<I>> for Is<I> {
    fn process(&mut self, mut state: State<I>) -> Result<I, Input<I>> {
        if state.input.len() < self.matches.len() {
            let input = state.input.fork();
            return Err(state.error(Error::Expected {
                expected: self.matches.clone(),
                found: input,
            }));
        }

        let grabbed = state.input.take(self.matches.len());

        if grabbed == self.matches {
            state.input = state.input.skip(self.matches.len());
            Ok((state, grabbed))
        } else {
            Err(state.error(Error::Expected {
                expected: self.matches.clone(),
                found: grabbed,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_with_is() {
        let result: (State<&str>, Input<&str>) = is("test").process("test".into()).unwrap();
        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "");

        let result: (State<&str>, Input<&str>) = is("test").process("test123".into()).unwrap();
        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "123");

        let result: Input<&str> = is("test").parse("test123").unwrap();
        assert_eq!(result, "test");

        let result: State<&str> = is("test").process("123test".into()).unwrap_err();
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

        let result: State<&str> = is("test").process("te".into()).unwrap_err();
        assert!(result.errors().any_errs());
        assert_eq!(result.errors().num_errors(), 1);
        assert_eq!(result.errors().errors().len(), 1);
        assert_eq!(
            result.errors().errors()[0],
            crate::parser::errors::Error::Expected {
                expected: "test",
                found: Input::new_with_span("te", (0..2).into())
            }
        );
    }
}
