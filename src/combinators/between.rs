use crate::parser::{errors::CustomError, input::Underlying, Parser};

/// Parses a parser between two other parsers. The first parser is the opening parser, the second
/// is the parser to parse, and the third is the closing parser. The opening and closing parsers'
/// outputs are ignored.
///```
/// # use errgonomic::combinators::{between, decimal, is};
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// # use errgonomic::parser::errors::DummyError;
/// let result = between(is::<_, DummyError>("("), decimal, is(")")).parse("(123)").unwrap();
/// assert_eq!(result, "123");
///```
pub fn between<I: Underlying, O1, O2, O3, E: CustomError, P1, P2, P3>(
    open: P1,
    parser: P2,
    close: P3,
) -> impl Parser<I, O2, E>
where
    P1: Parser<I, O1, E>,
    P2: Parser<I, O2, E>,
    P3: Parser<I, O3, E>,
{
    open.then(parser).then(close).map(|((_, o), _)| o)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        combinators::is,
        parser::{input::Input, state::State},
    };

    #[test]
    fn can_parse_between() {
        let result: (State<&str>, Input<&str>) = between(is("test"), is("123"), is("456"))
            .process("test123456789".into())
            .unwrap();

        assert_eq!(result.1, "123");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "789");
    }
}
