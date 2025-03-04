use super::{
    errors::{CustomError, DummyError, Error, Errors},
    input::{Input, Underlying},
};

/// The parser state.
#[derive(Debug)]
pub struct State<I, E = DummyError>
where
    I: Underlying,
    E: CustomError,
{
    /// The input we are currently parsing.
    input: Input<I>,

    /// Any errors that occurred during parsing.
    errors: Errors<I, E>,
}

impl<I, E> State<I, E>
where
    I: Underlying,
    E: CustomError,
{
    /// Create a new `State` object.
    pub fn new(input: I) -> Self {
        let input = Input::new(input);

        Self {
            errors: Errors::new(input.fork()),
            input,
        }
    }

    /// Checks if there are any errors.
    pub fn is_err(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Get the errors that occurred during parsing.
    pub fn errors(&self) -> &Errors<I, E> {
        &self.errors
    }

    /// Append an error to the list of errors.
    pub fn with_error(mut self, error: Error<I, E>) -> Self {
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

    /// Gets the input.
    pub fn as_input(&self) -> &Input<I> {
        &self.input
    }

    /// Gets a mutable reference to the input.
    pub fn as_input_mut(&mut self) -> &mut Input<I> {
        &mut self.input
    }

    /// Sets the input.
    pub fn with_input(mut self, input: Input<I>) -> Self {
        self.input = input;
        self
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

impl<I, E> From<I> for State<I, E>
where
    I: Underlying,
    E: CustomError,
{
    fn from(input: I) -> Self {
        Self::new(input)
    }
}
