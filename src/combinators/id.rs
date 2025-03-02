use crate::parser::{
    errors::{CustomError, Result},
    input::{Input, Underlying},
    state::State,
};

/// The identity parser. Consumes everything and returns it all.
/// ```
/// # use errgonomic::combinators::id;
/// # use errgonomic::parser::Parser;
/// # use errgonomic::parser::input::Input;
/// # use errgonomic::parser::state::State;
/// let (state, parsed): (State<&str>, Input<&str>) = id.process("test".into()).unwrap();
/// assert_eq!(parsed, "test");
/// assert_eq!(state.as_input().as_inner(), "");
/// ```
pub fn id<I: Underlying, E: CustomError>(mut state: State<I, E>) -> Result<I, Input<I>, E> {
    let input = state.input.fork();
    state.input = state.input.fork().skip(state.input.len());
    Ok((state, input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::input::Input;

    #[test]
    fn can_parse_id() {
        let result: (State<&str>, Input<&str>) = id("test".into()).unwrap();
        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "");
    }
}
