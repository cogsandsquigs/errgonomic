pub mod combinators;
pub mod input;
pub mod result;

use input::{PInput, Underlying};
use result::PResult;

pub trait Parser<I: Underlying, O, E> {
    /// Parses according to the rules defined in this `Parser`.
    fn parse(&mut self, input: PInput<I>) -> PResult<I, O, E>;
}

impl<I: Underlying, O, E, P> Parser<I, O, E> for P
where
    P: FnMut(PInput<I>) -> PResult<I, O, E>,
{
    /// Parses according to the rules defined in this parsing function.
    fn parse(&mut self, input: PInput<I>) -> PResult<I, O, E> {
        self(input)
    }
}
