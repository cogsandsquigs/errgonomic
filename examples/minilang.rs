use std::{
    fmt,
    io::{stdin, stdout, Write},
};

use errgonomic::{
    combinators::{any, between, decimal, is, separated, whitespace_wrapped as ww},
    parser::{errors::Result, input::Input, state::State, Parser},
};

#[derive(Debug)]
pub struct Value<'a> {
    v: ValueInner<'a>,
    s: Input<&'a str>,
}

impl<'a> Value<'a> {
    pub fn new(v: ValueInner<'a>, s: Input<&'a str>) -> Self {
        Self { v, s }
    }
}

#[derive(Debug)]
pub enum ValueInner<'a> {
    Number(i32),
    List(Vec<Value<'a>>),
}

impl fmt::Display for ValueInner<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

fn number(state: State<&str>) -> Result<&str, Value> {
    ww(decimal)
        // NOTE: See `examples/hex.rs` for why the `unwrap` is safe
        .map(|n: Input<&str>| {
            Value::new(ValueInner::Number(n.as_inner().parse::<i32>().unwrap()), n)
        })
        .process(state)
}

fn list(state: State<&str>) -> Result<&str, Value> {
    ww(between(
        is("["),
        separated(value, ww(is(",")), true),
        is("]"),
    ))
    .map_with_state(|state, vs| {
        let input = state.as_input().fork();
        (state, Value::new(ValueInner::List(vs), input))
    })
    .process(state)
}

fn value(state: State<&str>) -> Result<&str, Value> {
    any((number, list)).process(state)
}

fn parser(state: State<&str>) -> Result<&str, Value> {
    list.process(state)
}

pub fn main() {
    let mut s = String::new();

    loop {
        print!(">> ");

        // Cleanup and parse input
        let _ = stdout().flush();
        stdin().read_line(&mut s).unwrap();

        match parser.parse(s.trim()) {
            Ok(x) => println!("Parsed: {}", x),
            Err(err) => eprintln!("Error: {:?}", err),
        }

        // Cleanup the buffer
        s.clear();
    }
}
