use crate::parser::{
    errors::Result,
    input::{Input, Underlying},
    state::State,
};

/// The identity parser. Consumes everything and returns it all.
pub fn id<I: Underlying>(mut state: State<I>) -> Result<I, Input<I>> {
    let input = state.input.fork();
    state.input = state.input.take(state.input.len());
    Ok((state, input))
}
