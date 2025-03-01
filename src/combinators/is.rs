use crate::parsers::{
    errors::{PResult, ParseError},
    input::ParseInput,
    state::ParserState,
    Parser,
};

/// Matches the next bits of input with the given matching string `m`. If the input
/// matches, the parser returns the input. Otherwise, it fails.
/// NOTE: This will fail if the next bits of input are not equal to `m`.
/// (i.e. matching "abc" to "xabc").
pub fn is<I: ParseInput, E: ParseError>(m: I) -> impl Parser<I, I, E> {
    move |s: ParserState<I, E>| -> PResult<I, I, E> {
        if m.len() > s.len() {
            todo!("Should error!")
        }

        let taken = s.take(m.len());

        if taken.input() == m {
            Ok((taken, m.fork()))
        } else {
            todo!("Should error!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::errors::DefaultError;

    #[test]
    fn can_match_input() {
        let result = is::<&str, DefaultError>("hello").parse("hello").unwrap();
        assert_eq!(result, "hello");

        todo!("Try more types of parsers + parsers that fail first")
    }
}
