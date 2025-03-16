#![cfg(test)]

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

fn pratt_parser(state: State<&str>) -> Result<&str, Expr, DummyError> {
    Pratt::new(
        atom,
        |op, rhs| Ok(Expr::Prefix(op, Box::new(rhs))),
        |lhs, op, rhs| Ok(Expr::Infix(Box::new(lhs), op, Box::new(rhs))),
    )
    .with_infix_op(ww(is(".")).map(|_| Op::Compose), Associativity::Right)
    .with_prefix_op(ww(is("-")).map(|_| Op::Sub))
    .with_infix_op(ww(is("*")).map(|_| Op::Mul), Associativity::Left)
    .with_infix_op(ww(is("/")).map(|_| Op::Div), Associativity::Left)
    .with_infix_op(ww(is("+")).map(|_| Op::Add), Associativity::Left)
    .with_infix_op(ww(is("-")).map(|_| Op::Sub), Associativity::Left)
    .process(state)
}

#[test]
fn can_parse_atomic() {
    let (state, parsed): (State<&str>, Expr) = pratt_parser.process("123".into()).unwrap();
    assert_eq!(parsed, Expr::Int(123));
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());
}

#[test]
fn can_parse_simple_expr() {
    let (state, parsed): (State<&str>, Expr) = pratt_parser.process("123 + 456".into()).unwrap();
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
        pratt_parser.process("123 + 456 * 789".into()).unwrap();
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
        pratt_parser.process("123 * 456 + 789".into()).unwrap();
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
        pratt_parser.process("123 + 456 + 789".into()).unwrap();
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
    let (state, parsed): (State<&str>, Expr) = pratt_parser.process("a . b . c".into()).unwrap();
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
    let (state, parsed): (State<&str>, Expr) = pratt_parser.process("-123".into()).unwrap();
    assert_eq!(parsed, Expr::Prefix(Op::Sub, Box::new(Expr::Int(123))));
    assert_eq!(state.as_input(), &"");
    assert!(state.is_ok());

    let (state, parsed): (State<&str>, Expr) = pratt_parser.process("--123".into()).unwrap();
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
    let (state, parsed): (State<&str>, Expr) = pratt_parser.process("1 + -23 * 5".into()).unwrap();
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
