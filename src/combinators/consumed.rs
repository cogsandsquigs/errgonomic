use crate::parser::{
    errors::CustomError,
    input::{Input, Underlying},
    state::State,
    Parser,
};

/// Gets the input consumed by the parser and returns it as the output.
pub fn consumed<I: Underlying, O, E: CustomError, P: Parser<I, O, E>>(
    mut p: P,
) -> impl Parser<I, Input<I>, E> {
    move |state: State<I, E>| {
        let (new_state, _) = p.process(state.fork())?;
        let found = state.as_input().subtract(new_state.as_input());

        Ok((new_state, found))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinators::is;
    use crate::parser::Parser;

    #[test]
    fn can_parse_consumed() {
        let (state, parsed): (State<&str>, Input<&str>) =
            consumed(is("te")).process("test".into()).unwrap();
        assert_eq!(parsed, "te");
        assert_eq!(state.as_input().as_inner(), "st");
        assert!(!state.is_err());
    }
}
