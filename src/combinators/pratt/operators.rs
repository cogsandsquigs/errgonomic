use crate::parser::{
    errors::{CustomError, Result},
    input::Underlying,
    state::State,
    Parser,
};

pub struct PrefixOperator<'a, I, O, E>
where
    I: Underlying,
    E: CustomError,
{
    // The operator parser
    pub p: Box<dyn Parser<I, O, E> + 'a>,

    /// The right-precedence of the operator
    pub rbp: usize,

    pub _marker: std::marker::PhantomData<(I, O, E)>,
}

impl<I, O, E> Parser<I, (O, usize), E> for PrefixOperator<'_, I, O, E>
where
    I: Underlying,
    E: CustomError,
{
    /// Returns, in order:
    ///     1. The operator,
    ///     2. the right binding power
    fn process(&mut self, state: State<I, E>) -> Result<I, (O, usize), E> {
        // NOTE: For some reason, I need to map *after* the parse, otherwise Rust gives a "move
        // occurs because `self.p` has type `P`, which does not implement the `Copy` trait" error.
        self.p.process(state).map(|(s, o)| (s, (o, self.rbp)))
    }
}

pub struct InfixOperator<'a, I, O, E>
where
    I: Underlying,
    E: CustomError,
{
    // The operator parser
    pub p: Box<dyn Parser<I, O, E> + 'a>,

    /// The left-precedence of the operator
    pub lbp: usize,

    /// The right-precedence of the operator
    pub rbp: usize,

    pub _marker: std::marker::PhantomData<(I, O, E)>,
}

impl<I, O, E> Parser<I, (O, usize, usize), E> for InfixOperator<'_, I, O, E>
where
    I: Underlying,
    E: CustomError,
{
    /// Returns, in order:
    ///     1. The operator,
    ///     2. the left binding power
    ///     3. the right binding power
    fn process(&mut self, state: State<I, E>) -> Result<I, (O, usize, usize), E> {
        // NOTE: For some reason, I need to map *after* the parse, otherwise Rust gives a "move
        // occurs because `self.p` has type `P`, which does not implement the `Copy` trait" error.
        self.p
            .process(state)
            .map(|(s, o)| (s, (o, self.lbp, self.rbp)))
    }
}
