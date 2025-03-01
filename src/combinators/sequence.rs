use crate::parsers::{
    errors::{PResult, ParseError},
    input::ParseInput,
    state::ParserState,
    Parser,
};

/// Tries to parse all the parsers in the slice, and returns the first parser that matches.
/// If none of them match, the parser fails and returns the error from the last parser.
/// NOTE: Will panic if the slice is empty.
pub fn any<I: ParseInput, O, E: ParseError, const N: usize, P: Parser<I, O, E>>(
    mut ps: [P; N],
) -> impl Parser<I, O, E> {
    assert!(N > 0);

    move |s: ParserState<I, E>| -> PResult<I, O, E> {
        let mut last_err = None;

        for p in &mut ps {
            match p.process(s.fork()) {
                Ok((s, o)) => return Ok((s, o)),
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err.expect("This should be an error!"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{combinators::id::id, parsers::errors::DefaultError};

    use super::*;

    #[test]
    fn can_parse_any() {
        let result = any::<&str, _, DefaultError, 1, _>([id])
            .parse("hello")
            .unwrap();
        assert_eq!(result, "hello");

        let result = any::<&str, _, DefaultError, 2, _>([id, id])
            .parse("world")
            .unwrap();
        assert_eq!(result, "world");

        todo!("Try more types of parsers + parsers that fail first")
    }
}
