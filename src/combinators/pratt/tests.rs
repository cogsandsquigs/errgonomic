#![cfg(test)]

use std::{cell::LazyCell, sync::LazyLock};

use super::*;
use crate::{
    combinators::{alphabetic, decimal, is, whitespace_wrapped as ww},
    parser::{errors::DummyError, input::Input},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Int(i32),
    Ident(String),
    Prefix(Op, Box<Expr>),
    Infix(Box<Expr>, Op, Box<Expr>),
    Postfix(Box<Expr>, Op),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Compose,
    Factorial,
}

#[allow(clippy::declare_interior_mutable_const)]
const PRATT_PARSER: LazyLock<Pratt<&str, Expr, Op, DummyError>> = LazyLock::new(|| {
    Pratt::new(
        &atom,
        |op, rhs| Ok(Expr::Prefix(op, Box::new(rhs))),
        |lhs, op, rhs| Ok(Expr::Infix(Box::new(lhs), op, Box::new(rhs))),
        |lhs, op| Ok(Expr::Postfix(Box::new(lhs), op)),
    )
    .with_infix_op(ww(is(".")).map(|_| Op::Compose), Associativity::Right)
    .with_postfix_op(ww(is("!")).map(|_| Op::Factorial))
    .with_prefix_op(ww(is("-")).map(|_| Op::Sub))
    .with_infix_op(ww(is("*")).map(|_| Op::Mul), Associativity::Left)
    .with_infix_op(ww(is("/")).map(|_| Op::Div), Associativity::Left)
    .with_infix_op(ww(is("+")).map(|_| Op::Add), Associativity::Left)
    .with_infix_op(ww(is("-")).map(|_| Op::Sub), Associativity::Left)
});

fn atom(state: State<&str, DummyError>) -> Result<&str, Expr, DummyError> {
    any((
        decimal.map(|n: Input<&str>| Expr::Int(n.as_inner().parse().unwrap())),
        alphabetic.map(|s: Input<&str>| Expr::Ident(s.as_inner().to_string())),
    ))
    .process(state)
}

#[test]
fn can_parse_atomic() {
    let (state, parsed): (State<&str>, Expr) = PRATT_PARSER.process("123".into()).unwrap();
    assert_eq!(parsed, Expr::Int(123));
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_simple_expr() {
    let (state, parsed): (State<&str>, Expr) = PRATT_PARSER.process("123 + 456".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Infix(Box::new(Expr::Int(123)), Op::Add, Box::new(Expr::Int(456)),)
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_complex_expr() {
    let (state, parsed): (State<&str>, Expr) =
        PRATT_PARSER.process("123 + 456 * 789".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Infix(
            Box::new(Expr::Int(123)),
            Op::Add,
            Box::new(Expr::Infix(
                Box::new(Expr::Int(456)),
                Op::Mul,
                Box::new(Expr::Int(789))
            )),
        )
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());

    let (state, parsed): (State<&str>, Expr) =
        PRATT_PARSER.process("123 * 456 + 789".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Infix(
            Box::new(Expr::Infix(
                Box::new(Expr::Int(123)),
                Op::Mul,
                Box::new(Expr::Int(456)),
            )),
            Op::Add,
            Box::new(Expr::Int(789))
        )
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_left_associative() {
    let (state, parsed): (State<&str>, Expr) =
        PRATT_PARSER.process("123 + 456 + 789".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Infix(
            Box::new(Expr::Infix(
                Box::new(Expr::Int(123)),
                Op::Add,
                Box::new(Expr::Int(456)),
            )),
            Op::Add,
            Box::new(Expr::Int(789))
        )
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_right_associative() {
    let (state, parsed): (State<&str>, Expr) = PRATT_PARSER.process("a . b . c".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Infix(
            Box::new(Expr::Ident("a".into())),
            Op::Compose,
            Box::new(Expr::Infix(
                Box::new(Expr::Ident("b".into())),
                Op::Compose,
                Box::new(Expr::Ident("c".into())),
            )),
        )
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_prefix() {
    let (state, parsed): (State<&str>, Expr) = PRATT_PARSER.process("-123".into()).unwrap();
    assert_eq!(parsed, Expr::Prefix(Op::Sub, Box::new(Expr::Int(123))));
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());

    let (state, parsed): (State<&str>, Expr) = PRATT_PARSER.process("--123".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Prefix(
            Op::Sub,
            Box::new(Expr::Prefix(Op::Sub, Box::new(Expr::Int(123))))
        )
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_complex_prefix() {
    let (state, parsed): (State<&str>, Expr) = PRATT_PARSER.process("1 + -23 * 5".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Infix(
            Box::new(Expr::Int(1)),
            Op::Add,
            Box::new(Expr::Infix(
                Box::new(Expr::Prefix(Op::Sub, Box::new(Expr::Int(23)))),
                Op::Mul,
                Box::new(Expr::Int(5))
            )),
        )
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_postfix() {
    let (state, parsed): (State<&str>, Expr) = PRATT_PARSER.process("123!".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Postfix(Box::new(Expr::Int(123)), Op::Factorial)
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_complex_postfix() {
    let (state, parsed): (State<&str>, Expr) = PRATT_PARSER.process("123! + 456".into()).unwrap();
    assert_eq!(
        parsed,
        Expr::Infix(
            Box::new(Expr::Postfix(Box::new(Expr::Int(123)), Op::Factorial)),
            Op::Add,
            Box::new(Expr::Int(456))
        )
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_prefix_and_postfix() {
    let (state, parsed): (State<&str>, Expr) =
        PRATT_PARSER.process("3 * -123! + 456".into()).unwrap();

    // NOTE: Since `!` was declared first, we expect it to bind more tightly than `-`. Thus, we
    // should expect `-123!` to become `-(123!)`.

    assert_eq!(
        parsed,
        Expr::Infix(
            Box::new(Expr::Infix(
                Box::new(Expr::Int(3)),
                Op::Mul,
                Box::new(Expr::Prefix(
                    Op::Sub,
                    Box::new(Expr::Postfix(Box::new(Expr::Int(123)), Op::Factorial,)),
                ))
            )),
            Op::Add,
            Box::new(Expr::Int(456))
        )
    );
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}
