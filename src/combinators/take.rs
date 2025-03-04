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
        /*
                if state.input.len() < n {
                    let eoi_at = state.input.fork();
                    return Err(state.error(Error::ExpectedAny { eoi_at }));
                }

                let taken = state.input.fork().take(n);
                state.input = state.input.skip(n);
                Ok((state, taken))
        */
        let mut taken_len = 0;
        let original_input = state.as_input().fork();
        let input = state.as_input_mut();

        for _ in 0..n {
            if input.peek().is_none() {
                return Err(state.with_error(Error::ExpectedAny {
                    eoi_at: original_input.skip(taken_len),
                }));
            }

            input.next(); // Update the input to the next character
            taken_len += 1; // ... and increment the matched length
        }

        Ok((state, original_input.take(taken_len)))
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
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Input<&str>) = take(5).process("hello".into()).unwrap();
        assert_eq!(parsed, "hello");
        assert_eq!(state.as_input().as_inner(), "");
        assert!(!state.is_err());

        let state: State<&str> = take(5).process("hell".into()).unwrap_err();

        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors()[0],
            Error::ExpectedAny {
                eoi_at: Input::new_with_span("hell", 4..4)
            }
        );
    }
}
