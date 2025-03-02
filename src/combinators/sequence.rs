// use crate::parser::{errors::Result, input::Underlying, state::State, Parser};
//
// pub fn any<I: Underlying, O, const N: usize>(
//     ps: [&dyn Parser<I, O>; N],
// ) -> impl Parser<I, O> + use<'_, I, O, N> {
//     Any { ps }
// }
//
// pub struct Any<'a, I: Underlying, O, const N: usize> {
//     ps: [&'a dyn Parser<I, O>; N],
// }
//
// impl<'a, I: Underlying, O, const N: usize> Parser<I, O> for Any<'a, I, O, N> {
//     fn process(&mut self, state: State<I>) -> Result<I, O> {
//         todo!()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         combinators::{id, is},
//         parser::input::Input,
//     };
//
//     #[test]
//     fn can_parse_any() {
//         let result: (Input<&str>, &str) = any([&is("a"), &id]).process("a".into()).unwrap();
//     }
// }

use crate::parser::{errors::Result, input::Underlying, state::State, Parser};

/// Parses any of the given parsers. The first parser that succeeds will be the output. Otherwise,
/// if none of the parsers succeed, the error from the last parser will be returned.
pub fn any<I: Underlying, O, L: List<I, O>>(ps: L) -> impl Parser<I, O> {
    Any {
        ps,
        _phantom: core::marker::PhantomData,
    }
}

pub struct Any<I: Underlying, O, L: List<I, O>> {
    ps: L,
    _phantom: core::marker::PhantomData<(I, O)>,
}

impl<I: Underlying, O, L: List<I, O>> Parser<I, O> for Any<I, O, L> {
    fn process(&mut self, state: State<I>) -> Result<I, O> {
        self.ps.any(state)
    }
}

pub trait List<I: Underlying, O> {
    fn any(&mut self, state: State<I>) -> Result<I, O>;
}

impl<I: Underlying, O, P: Parser<I, O>> List<I, O> for (P,) {
    fn any(&mut self, state: State<I>) -> Result<I, O> {
        self.0.process(state)
    }
}

impl<I: Underlying, O, P1: Parser<I, O>, P2: Parser<I, O>> List<I, O> for (P1, P2) {
    fn any(&mut self, state: State<I>) -> Result<I, O> {
        self.0
            .process(state.fork())
            .or_else(|_| self.1.process(state))
    }
}

impl<I: Underlying, O, P1: Parser<I, O>, P2: Parser<I, O>, P3: Parser<I, O>> List<I, O>
    for (P1, P2, P3)
{
    fn any(&mut self, state: State<I>) -> Result<I, O> {
        self.0
            .process(state.fork())
            .or_else(|_| self.1.process(state.fork()))
            .or_else(|_| self.2.process(state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        combinators::{id, is},
        parser::input::Input,
    };

    #[test]
    fn can_parse_any() {
        let result: (State<&str>, Input<&str>) =
            super::any((is("test"),)).process("test".into()).unwrap();

        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "");

        let result: (State<&str>, Input<&str>) = super::any((is::<&str>("x"), is("test")))
            .process("test123".into())
            .unwrap();
        assert_eq!(result.1, "test");
        assert!(!result.0.errors().any_errs());
        assert_eq!(result.0.errors().num_errors(), 0);
        assert_eq!(result.0.errors().errors().len(), 0);
        assert_eq!(result.0.input, "123");

        let result: Input<&str> = super::any((id, is("test"))).parse("test123").unwrap();
        assert_eq!(result, "test123");

        let result: State<&str> = super::any((is("test"),))
            .process("123test".into())
            .unwrap_err();
        assert!(result.errors().any_errs());
        assert_eq!(result.errors().num_errors(), 1);
        assert_eq!(result.errors().errors().len(), 1);
        assert_eq!(
            result.errors().errors()[0],
            crate::parser::errors::Error::Expected {
                expected: "test",
                found: Input::new_with_span("123test", (0..4).into())
            }
        );
    }
}
