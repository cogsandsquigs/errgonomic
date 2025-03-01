use super::errors::Result;
use super::input::Underlying;
use super::state::State;
use super::Parser;

pub struct Map<I: Underlying, O, P: Parser<I, O>, F: Fn(O) -> O2, O2> {
    parser: P,
    f: F,
    _1: core::marker::PhantomData<I>,
    _2: core::marker::PhantomData<O>,
    _3: core::marker::PhantomData<O2>,
}

impl<I: Underlying, O, P: Parser<I, O>, F: Fn(O) -> O2, O2> Map<I, O, P, F, O2> {
    pub fn new(parser: P, f: F) -> Self {
        Self {
            parser,
            f,
            _1: core::marker::PhantomData,
            _2: core::marker::PhantomData,
            _3: core::marker::PhantomData,
        }
    }
}

impl<I: Underlying, O, P, F: Fn(O) -> O2, O2> Parser<I, O2> for Map<I, O, P, F, O2>
where
    I: Underlying,
    P: Parser<I, O>,
    F: Fn(O) -> O2,
{
    fn process(&mut self, state: State<I>) -> Result<I, O2> {
        self.parser
            .process(state)
            .map(|(state, output)| (state, (self.f)(output)))
    }
}

pub struct Chain<I: Underlying, O1, O2, P1: Parser<I, O1>, P2: Parser<I, O2>> {
    p1: P1,
    p2: P2,

    _1: core::marker::PhantomData<I>,
    _2: core::marker::PhantomData<O1>,
    _3: core::marker::PhantomData<O2>,
}

impl<I: Underlying, O1, O2, P1: Parser<I, O1>, P2: Parser<I, O2>> Chain<I, O1, O2, P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            p1,
            p2,
            _1: core::marker::PhantomData,
            _2: core::marker::PhantomData,
            _3: core::marker::PhantomData,
        }
    }
}

impl<I: Underlying, O1, O2, P1, P2> Parser<I, (O1, O2)> for Chain<I, O1, O2, P1, P2>
where
    I: Underlying,
    P1: Parser<I, O1>,
    P2: Parser<I, O2>,
{
    fn process(&mut self, state: State<I>) -> Result<I, (O1, O2)> {
        self.p1.process(state).and_then(|(state, output1)| {
            self.p2
                .process(state)
                .map(|(state, output2)| (state, (output1, output2)))
        })
    }
}
