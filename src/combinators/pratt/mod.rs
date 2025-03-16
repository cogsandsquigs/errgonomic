//! A Pratt parser, for all your Pratt-parsing needs.
//!
//! NOTE: This was mostly adapted from this excellent blog post by Matklad (creater of
//! rust-analyzer):
//! https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

mod operators;
mod tests;
mod utils;

use super::{any, maybe};
use crate::parser::{
    errors::{CustomError, Result},
    input::Underlying,
    state::State,
    Parser,
};
use operators::{InfixOperator, PostfixOperator, PrefixOperator};
use utils::{infix_cons_wrapper, postfix_cons_wrapper, prefix_cons_wrapper};

/// The associativity of an operator. Left-associative operators are parsed from left to right,
/// i.e. `1 + 2 + 3` is parsed as `(1 + 2) + 3`. Right-associative operators are parsed from right-
/// to-left, i.e. `1 ^ 2 ^ 3` is parsed as `1 ^ (2 ^ 3)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Associativity {
    Left,
    Right,
}

/// A pratt parser, which can handle parsing "operations" in expressions, like addition and
/// multiplication, or really anything you can think up that is "expression-like".
pub struct Pratt<'a, I, OExpr, OOp, E, PAtom, CPrefix, CInfix, CPostfix>
where
    I: Underlying,
    E: CustomError,
    PAtom: Parser<I, OExpr, E>,
    CPrefix: Fn(OOp, OExpr) -> std::result::Result<OExpr, E>,
    CInfix: Fn(OExpr, OOp, OExpr) -> std::result::Result<OExpr, E>,
    CPostfix: Fn(OExpr, OOp) -> std::result::Result<OExpr, E>,
{
    /// The atomic parser.
    pa: PAtom,

    /// The prefix combinator
    cons_prefix: CPrefix,

    /// The infix combinator
    cons_infix: CInfix,

    /// The postfix combinator
    cons_postfix: CPostfix,

    /// The prefix operators
    prefix_ops: Vec<PrefixOperator<'a, I, OOp, E>>,

    /// The infix operators
    infix_ops: Vec<InfixOperator<'a, I, OOp, E>>,

    /// The postfix operators
    postfix_ops: Vec<PostfixOperator<'a, I, OOp, E>>,

    _marker: std::marker::PhantomData<(I, OExpr, OOp, E)>,
}

impl<'a, I, OExpr, OOp, E, PA, CPrefix, CInfix, CPostfix>
    Pratt<'a, I, OExpr, OOp, E, PA, CPrefix, CInfix, CPostfix>
where
    I: Underlying,
    E: CustomError,
    PA: Parser<I, OExpr, E>,
    CPrefix: Fn(OOp, OExpr) -> std::result::Result<OExpr, E>,
    CInfix: Fn(OExpr, OOp, OExpr) -> std::result::Result<OExpr, E>,
    CPostfix: Fn(OExpr, OOp) -> std::result::Result<OExpr, E>,
{
    /// Creates a new Pratt parser, with no operators and no parsers. The `cons_prefix` and
    /// `cons_infix` parsers *combine* an operator (prefix or infix, respectively) with expressions
    /// to produce a new expression.
    ///
    /// NOTE: The operators you get for the infix and prefix operators are *guaranteed* to be
    /// those types of operators. Therefore, no other checking is necessary.
    ///
    /// NOTE: If you don't plan on using one of the `cons_*` functions, you can always just use a
    /// closure that returns an `unreachable!()`.
    pub fn new(pa: PA, cons_prefix: CPrefix, cons_infix: CInfix, cons_postfix: CPostfix) -> Self {
        Self {
            pa,
            cons_prefix,
            cons_infix,
            cons_postfix,
            prefix_ops: vec![],
            infix_ops: vec![],
            postfix_ops: vec![],
            _marker: std::marker::PhantomData,
        }
    }

    /// Adds an infix operator to the parser. The order in which you add the operators is their
    /// *precedence*, i.e. the first operator added binds the weakest, and the last operator added
    /// binds the strongest. So, to do multiplication before addition, you would do:
    ///
    /// ```
    /// # use errgonomic::prelude::*;
    /// # let parser = Pratt::new(
    /// #    |_: State<&str, DummyError>| unreachable!(),
    /// #    |_, _: ()| unreachable!(),
    /// #    |_, _, _| unreachable!(),
    /// #    |_, _| unreachable!()
    /// # );
    /// parser
    ///     .with_infix_op(is("*"), Associativity::Left)
    ///     .with_infix_op(is("+"), Associativity::Left);
    /// ```
    pub fn with_infix_op<P: Parser<I, OOp, E> + 'a>(mut self, p: P, assoc: Associativity) -> Self {
        let (lbp, rbp) = match assoc {
            Associativity::Left => (1, 2),
            Associativity::Right => (2, 1),
        };

        // Update the precedences, so that they have the correct precedence.
        // NOTE: We do this twice so that we don't have any overlapping Associativities.
        self.increment_precedence();
        self.increment_precedence();

        self.infix_ops.push(InfixOperator {
            p: Box::new(p),
            lbp,
            rbp,
            _marker: std::marker::PhantomData,
        });
        self
    }

    /// Adds a prefix operator to the parser. Like with `with_infix_op`, the order in which you add
    /// the operators affects their precedence. Notably, if you want precedence over other
    /// operators (including infix ones!), you would put the `with_prefix_op` call before the
    /// others.
    pub fn with_prefix_op<P: Parser<I, OOp, E> + 'a>(mut self, p: P) -> Self {
        self.increment_precedence();

        self.prefix_ops.push(PrefixOperator {
            p: Box::new(p),
            rbp: 1,
            _marker: std::marker::PhantomData,
        });
        self
    }

    /// Adds a postfix operator to the parser. Like with `with_infix_op`, the order in which you add
    /// the operators affects their precedence. Notably, if you want precedence over other
    /// operators (including infix ones!), you would put the `with_postfix_op` call before the
    /// others.
    pub fn with_postfix_op<P: Parser<I, OOp, E> + 'a>(mut self, p: P) -> Self {
        self.increment_precedence();

        self.postfix_ops.push(PostfixOperator {
            p: Box::new(p),
            lbp: 1,
            _marker: std::marker::PhantomData,
        });
        self
    }
}

impl<I, OExpr, OOp, E, PA, CPrefix, CInfix, CPostfix> Parser<I, OExpr, E>
    for Pratt<'_, I, OExpr, OOp, E, PA, CPrefix, CInfix, CPostfix>
where
    I: Underlying,
    E: CustomError,
    PA: Parser<I, OExpr, E>,
    CPrefix: Fn(OOp, OExpr) -> std::result::Result<OExpr, E>,
    CInfix: Fn(OExpr, OOp, OExpr) -> std::result::Result<OExpr, E>,
    CPostfix: Fn(OExpr, OOp) -> std::result::Result<OExpr, E>,
{
    fn process(&mut self, state: State<I, E>) -> Result<I, OExpr, E> {
        self.pratt(state, usize::MIN)
    }
}

impl<I, OExpr, OOp, E, PA, CPrefix, CInfix, CPostfix>
    Pratt<'_, I, OExpr, OOp, E, PA, CPrefix, CInfix, CPostfix>
where
    I: Underlying,
    E: CustomError,
    PA: Parser<I, OExpr, E>,
    CPrefix: Fn(OOp, OExpr) -> std::result::Result<OExpr, E>,
    CInfix: Fn(OExpr, OOp, OExpr) -> std::result::Result<OExpr, E>,
    CPostfix: Fn(OExpr, OOp) -> std::result::Result<OExpr, E>,
{
    /// The actual pratt parser
    fn pratt(&mut self, state: State<I, E>, min_lbp: usize) -> Result<I, OExpr, E> {
        let (mut state, mut lhs): (State<I, E>, OExpr) = {
            // try processing prefix
            // NOTE: Have to extract this expr. outside of the match b/c otherwise Rust doesn't
            // drop `self.prefix_ops` until end of match, causing a multiple-mutable-borrow error.
            // But if we extract the borrow to out here, then the borrow is dropped after `res` is
            // calculated.
            let res = self.maybe_parse_prefix_op(state.fork())?;

            match res {
                // We got a prefix op, parse it!
                (state, Some((op, rbp))) => {
                    let (state, rhs) = self.pratt(state, rbp)?;
                    prefix_cons_wrapper(op, rhs, &mut self.cons_prefix, state)?
                }
                // Never mind :(
                (_, None) => self.pa.process(state)?,
            }
        };

        loop {
            let res = self.maybe_parse_postfix_op(state.fork())?;

            match res {
                // We got a prefix op, parse it!
                (new_state, Some((lbp, op))) => {
                    if lbp < min_lbp {
                        break;
                    }

                    (state, lhs) =
                        postfix_cons_wrapper(lhs, op, &mut self.cons_postfix, new_state)?;

                    continue;
                }
                // Never mind :(
                _ => (),
            };

            let (new_state, (lbp, op, rbp)) =
                match maybe(|s| self.parse_infix_op(s)).process(state.fork())? {
                    (s, Some(x)) => (s, x),
                    (s, None) => return Ok((s, lhs)),
                };

            // NOTE: We don't actually want to assign the `state` until after this check, because
            // otherwise we need to "move up a level" and parse the next operator more weakly.
            if lbp < min_lbp {
                break;
            }

            let (new_state, rhs) = self.pratt(new_state, rbp)?;
            (state, lhs) = infix_cons_wrapper(lhs, op, rhs, &mut self.cons_infix, new_state)?;
        }

        Ok((state, lhs))
    }

    /// Parses first infix operator that works.
    fn parse_infix_op(&mut self, state: State<I, E>) -> Result<I, (usize, OOp, usize), E> {
        any(&mut self.infix_ops).process(state)
    }

    /// Parses first prefix operator that works.
    fn maybe_parse_prefix_op(&mut self, state: State<I, E>) -> Result<I, Option<(OOp, usize)>, E> {
        if self.prefix_ops.is_empty() {
            return Ok((state, None));
        }

        maybe(any(&mut self.prefix_ops)).process(state)
    }

    /// Parses first postfix operator that works.
    fn maybe_parse_postfix_op(&mut self, state: State<I, E>) -> Result<I, Option<(usize, OOp)>, E> {
        if self.postfix_ops.is_empty() {
            return Ok((state, None));
        }

        maybe(any(&mut self.postfix_ops)).process(state)
    }

    /// Increment the precedence of all the operators.
    fn increment_precedence(&mut self) {
        self.prefix_ops.iter_mut().for_each(|x| x.rbp += 1);
        self.postfix_ops.iter_mut().for_each(|x| x.lbp += 1);
        self.infix_ops.iter_mut().for_each(|x| {
            x.lbp += 1;
            x.rbp += 1;
        });
    }
}
