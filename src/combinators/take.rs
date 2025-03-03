use crate::parser::{
    errors::{CustomError, Error},
    input::{Input, Underlying},
    state::State,
    Parser,
};

/// Takes `n` elements from the input and returns them.
///
/// NOTE: If the input is less than `n` elements, the parser will return an error.
///
/// ```
/// # use errgonomic::combinators::{take};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, parsed): (State<&str>, Input<&str>) = take(5).process("hellohellohelloworld!".into()).unwrap();
/// assert_eq!(parsed, "hello");
/// assert_eq!(state.as_input().as_inner(), "hellohelloworld!");
/// ```
pub fn take<I: Underlying, E: CustomError>(n: usize) -> impl Parser<I, Input<I>, E> {
    move |mut state: State<I, E>| {
        if state.input.len() < n {
            let found = state.input.fork();
            return Err(state.error(Error::ExpectedAny { found }));
        }

        let taken = state.input.fork().take(n);
        state.input = state.input.skip(n);
        Ok((state, taken))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_take() {
        let (state, parsed): (State<&str>, Input<&str>) =
            take(5).process("hellohellohelloworld!".into()).unwrap();

        assert_eq!(parsed, "hello");
        assert_eq!(state.as_input().as_inner(), "hellohelloworld!");
        assert!(!state.errors().any_errs());

        let (state, parsed): (State<&str>, Input<&str>) = take(5).process("hello".into()).unwrap();
        assert_eq!(parsed, "hello");
        assert_eq!(state.as_input().as_inner(), "");
        assert!(!state.errors().any_errs());

        let state: State<&str> = take(5).process("hell".into()).unwrap_err();

        assert!(state.errors().any_errs());
        assert_eq!(state.errors().num_errors(), 1);
        assert_eq!(
            state.errors().errors()[0],
            Error::ExpectedAny {
                found: "hell".into()
            }
        );
    }
}
