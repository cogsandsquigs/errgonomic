use errgonomic::combinators::{eoi, hexadecimal_digit, is, many_m_n, maybe};
use errgonomic::parser::errors::{CustomError, Result};
use errgonomic::parser::state::State;
use errgonomic::parser::Parser;
use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MyError {
    ParseIntError(ParseIntError),
}

impl CustomError for MyError {}

fn hex_color_channel(state: State<&str, MyError>) -> Result<&str, u8, MyError> {
    many_m_n(2, 2, hexadecimal_digit)
        .map_result(|digits| {
            u8::from_str_radix(
                &(digits
                    .iter()
                    .map(|s| s.as_inner())
                    .collect::<Vec<_>>()
                    .join("")),
                16,
            )
            // NOTE: Not really needed as our input is guaranteed to be a hexadecimal digit, but
            // it's nice to have error handling
            .map_err(MyError::ParseIntError)
        })
        .process(state)
}

fn hex_color(state: State<&str, MyError>) -> Result<&str, Color, MyError> {
    let (state, ((((_, red), green), blue), _)) = maybe(is("#"))
        .then(hex_color_channel)
        .then(hex_color_channel)
        .then(hex_color_channel)
        .then(eoi)
        .process(state)?;

    Ok((state, Color { red, green, blue }))
}

fn main() {
    let mut s = String::new();
    print!("Hexadecimal color input (with or without the #): ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).unwrap();
    match hex_color.parse(s.trim()) {
        Ok(color) => println!("Parsed color: {:?}", color),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
