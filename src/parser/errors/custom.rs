use super::*;
use crate::parser::input::Underlying;

#[cfg(not(feature = "fancy"))]
pub trait CustomError: core::fmt::Debug + PartialEq + Eq + Clone {}

#[cfg(feature = "fancy")]
pub trait CustomError: core::fmt::Debug + PartialEq + Eq + Clone + core::error::Error {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DummyError;

impl core::error::Error for DummyError {}

impl core::fmt::Display for DummyError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Dummy error! Should never be seen!")
    }
}

impl CustomError for DummyError {}

pub trait CustomErrorMessage<I: Underlying, E: CustomError> {
    /// We expected something, but got something else instead.
    /// NOTE: You don't get what is found, but instead *where* it was found. In the future, this
    /// may change.
    fn expected(expected: ExpectedError<I>, found_at: Input<I>) -> String;

    /// Message for the custom error message.
    fn custom(&self, custom: E, found_at: Input<I>) -> String;
}

pub struct DefaultErrorMessage;

impl<I: Underlying, E: CustomError> CustomErrorMessage<I, E> for DefaultErrorMessage {
    fn expected(expected: ExpectedError<I>, found_at: Input<I>) -> String {
        match expected {
            ExpectedError::Is(x) => format!("expected {:?}", x),
            ExpectedError::Not(_) => todo!(),
            ExpectedError::Digit(_) => todo!(),
            ExpectedError::Alpha => todo!(),
            ExpectedError::AlphaNum => todo!(),
            ExpectedError::Whitespace => todo!(),
            ExpectedError::Newlines => todo!(),
            ExpectedError::WhitespaceNoNewlines => todo!(),
            ExpectedError::Nothing => todo!(),
            ExpectedError::Anything => todo!(),
        }
    }

    fn custom(&self, custom: E, found_at: Input<I>) -> String {
        format!("Custom error: {:?}", custom)
    }
}
