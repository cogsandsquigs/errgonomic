use crate::parser::{
    errors::Result,
    input::{Input, Underlying},
    state::State,
};

/// The identity parser. Consumes everything and returns it all.
/// ```
/// # use errgonomic::combinators::id;
/// # use errgonomic::parser::Parser;
/// assert_eq!(id.parse("test").unwrap(), "test");
/// ```
pub fn id<I: Underlying>(mut state: State<I>) -> Result<I, Input<I>> {
    let input = state.input.fork();
    state.input = state.input.take(state.input.len());
    Ok((state, input))
}
