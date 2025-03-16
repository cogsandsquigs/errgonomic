use super::{any, maybe};
use crate::parser::{
    errors::{CustomError, Error, ErrorKind, Result},
    input::Underlying,
    state::State,
    Parser,
};

/// A pratt parser, which can handle parsing "operations" in expressions, like addition and
/// multiplication, or really anything you can think up that is "expression-like".
pub struct Pratt<'a, I, OA, OO, E, PA, C>
where
    I: Underlying,
    E: CustomError,
    PA: Parser<I, OA, E>,
    C: Fn(OA, OO, OA) -> std::result::Result<OA, E>,
{
    /// The atomic parser.
    pa: PA,

    /// The combinator
    combinator: C,

    /// The operators
    ops: Vec<Operator<'a, I, OO, E>>,

    _marker: std::marker::PhantomData<(I, OA, OO, E, C)>,
}

impl<'a, I, OA, OO, E, PA, C> Pratt<'a, I, OA, OO, E, PA, C>
where
    I: Underlying,
    E: CustomError,
    PA: Parser<I, OA, E>,
    C: Fn(OA, OO, OA) -> std::result::Result<OA, E>,
{
    /// Creates a new Pratt parser, with no operators and no parsers.
    pub fn new(pa: PA, combinator: C) -> Self {
        Self {
            pa,
            combinator,
            ops: vec![],
            _marker: std::marker::PhantomData,
        }
    }

    /// Adds an operator to the parser.
    pub fn with_op<P: Parser<I, OO, E> + 'a>(mut self, p: P, lbp: usize, rbp: usize) -> Self {
        self.ops.push(Operator {
            p: Box::new(p),
            lbp,
            rbp,
            _marker: std::marker::PhantomData,
        });
        self
    }

    /* PRIVATE SHIT */
    /// The actual pratt parser
    fn expr_bp(&mut self, state: State<I, E>, min_lbp: usize) -> Result<I, OA, E> {
        let (mut state, mut lhs) = self.pa.process(state)?;

        loop {
            let (new_state, (op, lbp, rbp)) = match self.parse_op(state.fork())? {
                (s, Some(x)) => (s, x),
                (s, None) => return Ok((s, lhs)),
            };

            // NOTE: We don't actually want to assign the `state` until after this check, because
            // otherwise we need to "move up a level" and parse the next operator more weakly.
            if lbp < min_lbp {
                break;
            }

            let (new_state, rhs) = self.expr_bp(new_state, rbp)?;
            state = new_state; // reassign to `state`, essentially "advances" parser
            lhs = match (self.combinator)(lhs, op, rhs) {
                Err(e) => {
                    let location = state.as_input().fork();
                    state = state.with_error(Error::new(ErrorKind::custom(e), location));
                    return Err(state);
                }
                Ok(x) => x,
            };
        }

        Ok((state, lhs))
    }

    /// Parses first operator that works.
    fn parse_op(&mut self, state: State<I, E>) -> Result<I, Option<(OO, usize, usize)>, E> {
        println!("a");
        maybe(any(&mut self.ops)).process(state)
    }
}

impl<I, OA, OO, E, PA, C> Parser<I, OA, E> for Pratt<'_, I, OA, OO, E, PA, C>
where
    I: Underlying,
    E: CustomError,
    PA: Parser<I, OA, E>,
    C: Fn(OA, OO, OA) -> std::result::Result<OA, E>,
{
    fn process(&mut self, state: State<I, E>) -> Result<I, OA, E> {
        self.expr_bp(state, usize::MIN)
    }
}

struct Operator<'a, I, O, E>
where
    I: Underlying,
    E: CustomError,
{
    // The operator parser
    p: Box<dyn Parser<I, O, E> + 'a>,

    /// The left-precedence of the operator
    lbp: usize,

    /// The right-precedence of the operator
    rbp: usize,

    _marker: std::marker::PhantomData<(I, O, E)>,
}

impl<I, O, E> Parser<I, (O, usize, usize), E> for Operator<'_, I, O, E>
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        combinators::{alphabetic, decimal, is, whitespace_wrapped as ww},
        parser::{errors::DummyError, input::Input},
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Expr {
        Int(i32),
        Ident(String),
        Op(Box<Expr>, Op, Box<Expr>),
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Op {
        Add,
        Sub,
        Mul,
        Div,
        Compose,
    }

    fn atom(state: State<&str, DummyError>) -> Result<&str, Expr, DummyError> {
        any((
            decimal.map(|n: Input<&str>| Expr::Int(n.as_inner().parse().unwrap())),
            alphabetic.map(|s: Input<&str>| Expr::Ident(s.as_inner().to_string())),
        ))
        .process(state)
    }

    fn combinator(lhs: Expr, op: Op, rhs: Expr) -> std::result::Result<Expr, DummyError> {
        Ok(Expr::Op(Box::new(lhs), op, Box::new(rhs)))
    }

    fn pratt_parser(state: State<&str>) -> Result<&str, Expr, DummyError> {
        Pratt::new(atom, combinator)
            .with_op(ww(is("+")).map(|_| Op::Add), 1, 2)
            .with_op(ww(is("-")).map(|_| Op::Sub), 1, 2)
            .with_op(ww(is("*")).map(|_| Op::Mul), 3, 4)
            .with_op(ww(is("/")).map(|_| Op::Div), 3, 4)
            .with_op(ww(is(".")).map(|_| Op::Compose), 6, 5)
            .process(state)
    }

    #[test]
    fn can_parse_atomic() {
        let (state, parsed): (State<&str>, Expr) = pratt_parser.process("123".into()).unwrap();
        assert_eq!(parsed, Expr::Int(123));
        assert_eq!(state.as_input(), &"");
    }

    #[test]
    fn can_parse_simple_expr() {
        let (state, parsed): (State<&str>, Expr) =
            pratt_parser.process("123 + 456".into()).unwrap();
        assert_eq!(
            parsed,
            Expr::Op(Box::new(Expr::Int(123)), Op::Add, Box::new(Expr::Int(456)),)
        );
        assert_eq!(state.as_input(), &"");
    }

    #[test]
    fn can_parse_complex_expr() {
        let (state, parsed): (State<&str>, Expr) =
            pratt_parser.process("123 + 456 * 789".into()).unwrap();
        assert_eq!(
            parsed,
            Expr::Op(
                Box::new(Expr::Int(123)),
                Op::Add,
                Box::new(Expr::Op(
                    Box::new(Expr::Int(456)),
                    Op::Mul,
                    Box::new(Expr::Int(789))
                )),
            )
        );
        assert_eq!(state.as_input(), &"");

        let (state, parsed): (State<&str>, Expr) =
            pratt_parser.process("123 * 456 + 789".into()).unwrap();
        assert_eq!(
            parsed,
            Expr::Op(
                Box::new(Expr::Op(
                    Box::new(Expr::Int(123)),
                    Op::Mul,
                    Box::new(Expr::Int(456)),
                )),
                Op::Add,
                Box::new(Expr::Int(789))
            )
        );
        assert_eq!(state.as_input(), &"");
    }

    #[test]
    fn can_parse_left_associative() {
        let (state, parsed): (State<&str>, Expr) =
            pratt_parser.process("123 + 456 + 789".into()).unwrap();
        assert_eq!(
            parsed,
            Expr::Op(
                Box::new(Expr::Op(
                    Box::new(Expr::Int(123)),
                    Op::Add,
                    Box::new(Expr::Int(456)),
                )),
                Op::Add,
                Box::new(Expr::Int(789))
            )
        );
        assert_eq!(state.as_input(), &"");
    }

    #[test]
    fn can_parse_right_associative() {
        let (state, parsed): (State<&str>, Expr) =
            pratt_parser.process("a . b . c".into()).unwrap();
        assert_eq!(
            parsed,
            Expr::Op(
                Box::new(Expr::Ident("a".into())),
                Op::Compose,
                Box::new(Expr::Op(
                    Box::new(Expr::Ident("b".into())),
                    Op::Compose,
                    Box::new(Expr::Ident("c".into())),
                )),
            )
        );
        assert_eq!(state.as_input(), &"");
    }
}
