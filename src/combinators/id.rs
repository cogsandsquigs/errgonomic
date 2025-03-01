use crate::parsers::{
    errors::{PResult, ParseError},
    input::ParseInput,
    state::ParserState,
};

/// Consumes the input and returns it.
pub fn id<I: ParseInput, E: ParseError>(s: ParserState<I, E>) -> PResult<I, I, E> {
    let consumed = s.input();
    Ok((s.skip(s.len()), consumed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{errors::DefaultError, Parser};

    #[test]
    fn can_consume_input() {
        let result = id::<&str, DefaultError>.parse("hello").unwrap();
        assert_eq!(result, "hello");

        let result = id::<&str, DefaultError>
            .process(ParserState::new("world"))
            .unwrap();
        assert_eq!(result.1, "world");
        assert_eq!(result.0.input(), "");
        assert_eq!(result.0.len(), 0);
        assert!(result.0.is_empty());
        assert_eq!(result.0.fork().input(), "");
    }
}
