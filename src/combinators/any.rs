use crate::parser::{
    errors::{CustomError, Error, ErrorKind, Result},
    input::Underlying,
    state::State,
    Parser,
};
use eval_macro::eval;

/// Parses any of the given parsers. The first parser that succeeds will be the output. Otherwise,
/// if none of the parsers succeed, the error from the last parser will be returned.
///```
/// # use errgonomic::combinators::{any, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// let (state, parsed): (State<&str>, Input<&str>) = any((is("hello"), is("world"))).process("hello, world!".into()).unwrap();
/// assert_eq!(parsed, "hello");
/// assert_eq!(state.as_input().as_inner(), ", world!");
///```
#[inline]
#[allow(private_bounds)]
pub fn any<I: Underlying, O, E: CustomError, L: List<I, O, E>>(mut ps: L) -> impl Parser<I, O, E> {
    move |state| ps.any(state)
}

/// Allows for errors to be committed on, if an error does occur. This means that in certain cases
/// where an error occurs during a branch of parsing, if we "went down" it far enough that we can
/// definately say that we are supposed to be on this branch, *and* an error occured, then when we
/// go back up the branch we only return those errors.
#[inline]
pub fn commit<I: Underlying, O, E: CustomError, P: Parser<I, O, E>>(
    mut p: P,
) -> impl Parser<I, O, E> {
    move |state| match p.process(state) {
        Ok(x) => Ok(x),
        Err(e) => Err(e.commit()),
    }
}

/* TRAIT IMPLEMENTATIONS NEEDED FOR ANY */
/* These are annoying and long, you can ignore*/

trait List<I: Underlying, O, E: CustomError> {
    fn any(&mut self, state: State<I, E>) -> Result<I, O, E>;
}

// Magic macro magic that makes the impl. of `List` for (nearly!) all tuples of parsers.
// See: https://crates.io/crates/eval-macro
eval! {
    const UP_TO: usize = 20; // NOTE: The maximum size of the parser-tuples we want to implement.

    for n in 1..=UP_TO {
        let parser_generics = (1..=n)
            .into_iter()
            .map(|i| format!("P{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        // NOTE: Leading comma so that this also works for the single-tuple
        let parser_tuple = format!("({parser_generics},)");
        let parser_defs = (1..=n)
            .into_iter()
            .map(|i| format!("P{i}: Parser<I, O, E>"))
            .collect::<Vec<_>>()
            .join(",\n");
        let processing = (0..n)
            .into_iter()
            .map(|i| format!("match self.{i}.process(state.fork()) {{
                Ok(x) => return Ok(x),
                Err(e) if e.errors().is_committed() => return Err(e),
                Err(e) => errs.push(e.errors().clone()),
            }};"))
            .collect::<Vec<_>>()
            .join("\n\n");

        output! {
            impl<I, O, E, {{parser_generics}}> List<I, O, E> for {{parser_tuple}}
            where
                I: Underlying,
                E: CustomError,
                {{parser_defs}}
            {
                #[inline]
                fn any(&mut self, state: State<I, E>) -> Result<I, O, E> {
                    let mut errs: Vec<Error<I, E>> = vec![];

                    {{processing}}

                    let input = errs
                        .iter()
                        .map(|err| err.from())
                        .reduce(|acc, x| acc.join_between(&x))
                        .expect("There to be at least 1 error");

                    Err(state.with_error(Error::new(ErrorKind::all(errs), input)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        combinators::{id, is},
        parser::{
            errors::{DummyError, Error, ExpectedError},
            input::Input,
        },
    };

    #[test]
    fn can_parse_any() {
        let (state, parsed): (State<&str>, Input<&str>) = any((is("x"), is("test")))
            .process("test123".into())
            .unwrap();
        assert_eq!(parsed, "test");
        assert_eq!(state.as_input(), &"123");
        assert!(!state.is_err());

        let parsed: Input<&str> = any((id::<_, DummyError>, is("test")))
            .parse("test123")
            .unwrap();
        assert_eq!(parsed, "test123");

        let state: State<&str> = any((is("done"), is("test")))
            .process("123test".into())
            .unwrap_err();
        assert!(state.is_err());
        println!("{:#?}", state.errors());
        assert_eq!(state.errors().len(), 2);
        assert_eq!(
            state.errors(),
            &Error::new(
                ErrorKind::All(vec![
                    Error::new(
                        ErrorKind::Expected(ExpectedError::Is("done")),
                        Input::new_with_span("123test", 0..1)
                    ),
                    Error::new(
                        ErrorKind::Expected(ExpectedError::Is("test")),
                        Input::new_with_span("123test", 0..1)
                    ),
                ]),
                Input::new_with_span("123test", 0..1)
            )
        );
    }

    #[test]
    fn test_basic_commit() {
        // Test successful parsing - commit shouldn't affect successful results
        let (state, parsed): (State<&str>, Input<&str>) =
            commit(is("test")).process("test123".into()).unwrap();
        assert_eq!(parsed, "test");
        assert_eq!(state.as_input(), &"123");
        assert!(!state.is_err());
    }

    #[test]
    fn test_commit_error() {
        // Test that a committed error is properly marked
        let error_state: State<&str> = commit(is("test")).process("xyz123".into()).unwrap_err();

        assert!(error_state.is_err());
        assert!(error_state.errors().is_committed());
        // Verify the error kind is still preserved
        assert_eq!(
            error_state.errors(),
            &Error::new(
                ErrorKind::Committed(Box::new(Error::new(
                    ErrorKind::Expected(ExpectedError::Is("test")),
                    Input::new_with_span("xyz123", 0..1)
                ))),
                Input::new_with_span("xyz123", 0..1)
            )
        );
    }

    #[test]
    fn test_commit_with_any() {
        // This tests the interaction between commit and any
        // If the first parser in 'any' is committed and fails,
        // it should short-circuit and return just that error

        let error_state: State<&str> = any((
            commit(is("hello")), // This will fail and be committed
            is("xyz"),           // This would succeed but shouldn't be tried
        ))
        .process("abc123".into())
        .unwrap_err();

        assert!(error_state.is_err());
        assert!(error_state.errors().is_committed());

        // Should only have one error (from the committed parser)
        // rather than collecting errors from both parsers
        assert_eq!(error_state.errors().len(), 1);

        // The error should be about failing to match "hello"
        assert_eq!(
            error_state.errors(),
            &Error::new(
                ErrorKind::Committed(Box::new(Error::new(
                    ErrorKind::Expected(ExpectedError::Is("hello")),
                    Input::new_with_span("abc123", 0..1)
                ))),
                Input::new_with_span("abc123", 0..1)
            )
        );
    }

    #[test]
    fn test_nested_commit() {
        // Test nesting commit parsers
        let error_state: State<&str> = commit(commit(is("test")))
            .process("xyz123".into())
            .unwrap_err();

        assert!(error_state.is_err());
        assert!(error_state.errors().is_committed());
    }

    #[test]
    fn test_commit_with_complex_parser() {
        // Test commit with a more complex parser setup
        // For example with sequence or other combinators

        // This simulates a parser that expects "test" followed by "123"
        let complex_parser = move |state| {
            is("test")
                .then(is("123"))
                .map(|(a, b)| a.join(&b))
                .process(state)
        };

        // Test when first part matches but second fails
        let error_state: State<&str> = commit(complex_parser)
            .process("test456".into())
            .unwrap_err();

        assert!(error_state.is_err());
        assert!(error_state.errors().is_committed());

        // The error should be about failing to match "123" after "test"
        assert_eq!(
            error_state.errors(),
            &Error::new(
                ErrorKind::Committed(Box::new(Error::new(
                    ErrorKind::Expected(ExpectedError::Is("123")),
                    Input::new_with_span("test456", 4..5)
                ))),
                Input::new_with_span("test456", 4..5)
            )
        );

        // The error position should be at the start of "456", not at the start of the input
        assert_eq!(error_state.as_input(), &"456");
    }

    #[test]
    fn test_any_with_one_committed_error() {
        // Test that when using 'any', a committed error takes precedence over non-committed errors

        // First parser will fail with a committed error
        let committed_parser = commit(is("hello"));
        // Second parser will fail with a non-committed error
        let normal_parser = is("world");

        let error_state: State<&str> = any((committed_parser, normal_parser))
            .process("xyz123".into())
            .unwrap_err();

        assert!(error_state.is_err());
        // Check that the final error is committed
        assert!(error_state.errors().is_committed());

        // The error should be from the committed parser only (not a collection of all errors)
        assert_eq!(
            error_state.errors(),
            &Error::new(
                ErrorKind::Committed(Box::new(Error::new(
                    ErrorKind::Expected(ExpectedError::Is("hello")),
                    Input::new_with_span("xyz123", 0..1)
                ))),
                Input::new_with_span("xyz123", 0..1)
            )
        );
    }

    #[test]
    fn test_any_with_multiple_committed_errors() {
        // Test that when using 'any' with multiple committed parsers,
        // we return the first committed error encountered

        // Both parsers will fail with committed errors
        let first_committed = commit(is("first"));
        let second_committed = commit(is("second"));

        let error_state: State<&str> = any((first_committed, second_committed))
            .process("xyz123".into())
            .unwrap_err();

        assert!(error_state.is_err());
        assert!(error_state.errors().is_committed());

        // We should get just the first committed error, not both
        assert_eq!(
            error_state.errors(),
            &Error::new(
                ErrorKind::Committed(Box::new(Error::new(
                    ErrorKind::Expected(ExpectedError::Is("first")),
                    Input::new_with_span("xyz123", 0..1)
                ))),
                Input::new_with_span("xyz123", 0..1)
            )
        );

        // Make sure we don't have information about the second parser error
        let error_info = format!("{:?}", error_state.errors());
        assert!(!error_info.contains("second"));
    }
}
