use super::{
    errors::{Error, Errors},
    input::{Input, Underlying},
};

/// The parser state.
#[derive(Debug)]
pub struct State<I: Underlying> {
    /// The input we are currently parsing.
    pub input: Input<I>,

    /// Any errors that occurred during parsing.
    errors: Errors<I>,
}

impl<I: Underlying> State<I> {
    /// Create a new `State` object.
    pub fn new(input: I) -> Self {
        let input = Input::new(input);

        Self {
            errors: Errors::new(input.fork()),
            input,
        }
    }

    /// Get the errors that occurred during parsing.
    pub fn errors(&self) -> &Errors<I> {
        &self.errors
    }

    /// Append an error to the list of errors.
    pub fn error(mut self, error: Error<I>) -> Self {
        self.errors.push(error);
        self
    }

    /// Fork the state.
    pub fn fork(&self) -> Self {
        Self {
            errors: self.errors.clone(),
            input: self.input.fork(),
        }
    }
}

impl<I> From<Input<I>> for State<I>
where
    I: Underlying,
{
    fn from(input: Input<I>) -> Self {
        Self {
            errors: Errors::new(input.fork()),
            input,
        }
    }
}

impl<I> From<I> for State<I>
where
    I: Underlying,
{
    fn from(input: I) -> Self {
        Self::new(input)
    }
}
