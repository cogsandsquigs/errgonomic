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
pub fn id<I: Underlying, E: CustomError>(state: State<I, E>) -> Result<I, Input<I>, E> {
    let input = state.as_input().fork();
    Ok((state.with_input(input.skip_all()), input))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::input::Input;

    #[test]
    fn can_parse_id() {
        let (state, parsed): (State<&str>, Input<&str>) = id("test".into()).unwrap();
        assert_eq!(parsed, "test");
        assert!(!state.is_err());
        assert_eq!(state.as_input(), &"");
        assert_eq!(state.errors().len(), 0);
    }
}
