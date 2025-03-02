use crate::parser::{input::Underlying, Parser};

/// Parses a parser between two other parsers. The first parser is the opening parser, the second
/// is the parser to parse, and the third is the closing parser. The opening and closing parsers'
/// outputs are ignored.
///```
/// # use errgonomic::combinators::{between, decimal, is};
/// # use errgonomic::parser::Parser;
/// assert_eq!(between(is("("), decimal, is(")")).parse("(123)").unwrap(), "123");
///```
pub fn between<I: Underlying, O1, O2, O3, P1, P2, P3>(
    open: P1,
    parser: P2,
    close: P3,
) -> impl Parser<I, O2>
where
    P1: Parser<I, O1>,
    P2: Parser<I, O2>,
    P3: Parser<I, O3>,
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
