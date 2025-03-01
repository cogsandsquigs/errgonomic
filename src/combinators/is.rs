use crate::parser::{
    errors::Result,
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
            todo!("Need to error out here!")
        }

        let grabbed = state.input.take(self.matches.len());

        if grabbed == self.matches {
            state.input = state.input.skip(self.matches.len());
            Ok((state, grabbed))
        } else {
            todo!("Need to error out here!")
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

        todo!("Need to test error cases!");
    }
}
