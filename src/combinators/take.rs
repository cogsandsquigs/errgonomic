use crate::parser::{
    errors::{CustomError, Error, ErrorKind, ExpectedError, Result},
    input::{Input, Underlying},
    state::State,
    Parser,
};

/// Takes `n` elements from the input and returns them.
///
/// NOTE: When `unicode` is enabled, will take `n` unicode characters.
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
        #[cfg(not(feature = "unicode"))]
        {
            let mut taken_len = 0;
            let original_input = state.as_input().fork();
            let input = state.as_input_mut();

            for _ in 0..n {
                if input.peek().is_none() {
                    return Err(state.with_error(Error::new(
                        ErrorKind::expected(ExpectedError::Anything),
                        original_input.skip(taken_len),
                    )));
                }

                input.next(); // Update the input to the next character
                taken_len += 1; // ... and increment the matched length
            }

            Ok((state, original_input.take(taken_len)))
        }
        #[cfg(feature = "unicode")]
        {
            let mut taken_bytes_len = 0;
            let original_input = state.as_input().fork();
            let input = state.as_input_mut();

            for _ in 0..n {
                match input.peek_char() {
                    None => {
                        return Err(state.with_error(Error::new(
                            ErrorKind::expected(ExpectedError::Anything),
                            original_input.skip(taken_bytes_len),
                        )));
                    }
                    Some(c) => {
                        taken_bytes_len += c.len_utf8(); // ... and increment the matched length
                        input.next_char(); // Update the input to the next character
                    }
                }
            }

            Ok((state, original_input.take(taken_bytes_len)))
        }
    }
}

/// Takes elements from the input until a parser `until` matches. The output of `until` will be
/// included in the output. If we encounter an end-of-input before `until` matches, an error will
/// be returned.
///
/// ```
/// # use errgonomic::combinators::{take_until, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::input::Input;
/// let (state, (parsed, until)): (State<&str>, (Input<&str>, Input<&str>)) = take_until(is("world")).process("hellohellohelloworld!".into()).unwrap();
/// assert_eq!(parsed, "hellohellohello");
/// assert_eq!(until, "world");
/// assert_eq!(state.as_input().as_inner(), "!");
/// ```
pub fn take_until<I: Underlying, O2, E: CustomError, P: Parser<I, O2, E>>(
    mut until: P,
) -> impl Parser<I, (Input<I>, O2), E> {
    move |mut state: State<I, E>| -> Result<I, (Input<I>, O2), E> {
        let mut taken_len = 0;
        let original_input = state.as_input().fork();

        loop {
            match until.process(state.fork()) {
                Ok((new_state, o)) => {
                    return Ok((new_state, (original_input.take(taken_len), o)));
                }
                Err(_) => {
                    taken_len += 1;
                    state = state.with_input(original_input.skip(taken_len));
                }
            }

            // HACK: Gets around the error where if we are `take`-ing until an `eoi` matches, we
            // will always error before the `eoi` matches as we will check for a `None` first.
            // TODO: Make this faster?
            let future = state.fork().with_input(state.as_input().take(1));
            if state.as_input().peek().is_none() && until.process(future).is_err() {
                println!("input is none!");
                return Err(state.with_error(Error::new(
                    ErrorKind::expected(ExpectedError::Anything),
                    original_input.skip(taken_len),
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        combinators::is,
        parser::errors::{ErrorKind, ExpectedError},
    };

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
            state.errors(),
            &Error::new(
                ErrorKind::expected(ExpectedError::Anything),
                Input::new_with_span("hell", 4..4)
            )
        );
    }

    #[test]
    fn can_take_until() {
        let (state, (parsed, until)): (State<&str>, (Input<&str>, Input<&str>)) =
            take_until(is("world"))
                .process("hellohellohelloworld!".into())
                .unwrap();
        assert_eq!(parsed, "hellohellohello");
        assert_eq!(until, "world");
        assert_eq!(state.as_input().as_inner(), "!");
        assert!(!state.is_err());
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn can_parse_unicode_take() {
        let (state, parsed): (State<&str>, Input<&str>) =
            take(5).process("héllôhellohelloworld!".into()).unwrap();
        assert_eq!(parsed, "héllô");
        assert_eq!(state.as_input().as_inner(), "hellohelloworld!");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Input<&str>) = take(5).process("héllô".into()).unwrap();
        assert_eq!(parsed, "héllô");
        assert_eq!(state.as_input().as_inner(), "");

        let state: State<&str> = take(5).process("héll".into()).unwrap_err();
        assert!(state.is_err());
        assert_eq!(state.errors().len(), 1);
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::expected(ExpectedError::Anything),
                Input::new_with_span("héll", 5..5)
            )
        );

        let (state, parsed): (State<&str>, Input<&str>) =
            take(5).process("héllô😍".into()).unwrap();
        assert_eq!(parsed, "héllô");
        assert_eq!(state.as_input().as_inner(), "😍");
        assert!(!state.is_err());

        let (state, parsed): (State<&str>, Input<&str>) =
            take(6).process("héllô😍".into()).unwrap();
        assert_eq!(parsed, "héllô😍");
        assert_eq!(state.as_input().as_inner(), "");
        assert!(!state.is_err());
    }
}
