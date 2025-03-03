//! This shows how you can parse a minature language with `errgonomic`.

use std::{
    fmt,
    io::{stdin, stdout, Write},
};

use errgonomic::{
    combinators::{any, between, decimal, eoi, is, maybe, whitespace, whitespace_wrapped as ww},
    parser::{
        errors::{CustomError, Result},
        input::Input,
        state::State,
        Parser,
    },
};

enum Expression {
    Number(i32),
    Operation {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl Expression {
    fn eval(&self) -> i32 {
        match self {
            Self::Number(n) => *n,
            Self::Operation {
                operator,
                left,
                right,
            } => match operator {
                Operator::Add => left.eval() + right.eval(),
                Operator::Sub => left.eval() - right.eval(),
                Operator::Mul => left.eval() * right.eval(),
                Operator::Div => left.eval() / right.eval(),
            },
        }
    }
}

enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ParseError {
    InvalidOperator,
}

impl CustomError for ParseError {}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Operation {
                operator,
                left,
                right,
            } => {
                write!(f, "({} {} {})", operator, left, right)
            }
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

fn number(state: State<&str, ParseError>) -> Result<&str, Expression, ParseError> {
    ww(decimal)
        // NOTE: See `examples/hex.rs` for why the `unwrap` is safe
        .map(|n: Input<&str>| Expression::Number(n.as_inner().parse::<i32>().unwrap()))
        .process(state)
}

fn operator(state: State<&str, ParseError>) -> Result<&str, Operator, ParseError> {
    ww(any((is("+"), is("-"), is("*"), is("/"))))
        .map_res(|op: Input<&str>| {
            Ok(match op.as_inner() {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                "/" => Operator::Div,
                _ => {
                    return Err(ParseError::InvalidOperator);
                }
            })
        })
        .process(state)
}

fn operation(state: State<&str, ParseError>) -> Result<&str, Expression, ParseError> {
    let (state, (((op, left), _), right)) = between(
        ww(is("(")),
        operator.then(ww(value)).then(maybe(whitespace)).then(value),
        ww(is(")")),
    )
    .process(state)?;

    Ok((
        state,
        Expression::Operation {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
        },
    ))
}

fn value(state: State<&str, ParseError>) -> Result<&str, Expression, ParseError> {
    any((number, operation)).process(state)
}

fn parser(state: State<&str, ParseError>) -> Result<&str, Expression, ParseError> {
    ww(value).then(eoi).map(|(x, _)| x).process(state)
}

pub fn main() {
    let mut s = String::new();

    loop {
        print!(">> ");

        // Cleanup and parse input
        let _ = stdout().flush();
        stdin().read_line(&mut s).unwrap();

        match parser.parse(s.trim()) {
            Ok(x) => println!("{}", x.eval()),
            Err(err) => eprintln!("Error: {:?}", err),
        }

        // Cleanup the buffer
        s.clear();
    }
}
