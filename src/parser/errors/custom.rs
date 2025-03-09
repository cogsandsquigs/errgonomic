pub trait CustomError: core::fmt::Debug + PartialEq + Eq + Clone + core::error::Error {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DummyError;

impl CustomError for DummyError {}

impl core::error::Error for DummyError {}

impl core::fmt::Display for DummyError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Dummy error! Should never be seen!")
    }
}
