use std::num::ParseIntError;

use errgonomic::combinators::{hexadecimal_digit, is};
use errgonomic::parser::errors::{CustomError, Error, Result};
use errgonomic::parser::input::Input;
use errgonomic::parser::state::State;
use errgonomic::parser::Parser;

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
    hexadecimal_digit::<&str, MyError>
        .then(hexadecimal_digit)
        .map_result(|(digit_1, digit_2)| {
            u8::from_str_radix(&(digit_1.as_inner().to_string() + digit_2.as_inner()), 16)
                .map_err(MyError::ParseIntError)
        })
        .process(state)
}

fn hex_color(state: State<&str, MyError>) -> Result<&str, Color, MyError> {
    let (state, _) = is("#").process(state)?;
    let (state, red) = hex_color_channel.process(state)?;
    let (state, green) = hex_color_channel.process(state)?;
    let (state, blue) = hex_color_channel.process(state)?;

    Ok((state, Color { red, green, blue }))
}

#[test]
fn hex_parser() {
    let result = hex_color.parse("#FF00FF").unwrap();
    assert_eq!(
        result,
        Color {
            red: 255,
            green: 0,
            blue: 255,
        }
    );

    let err = hex_color.parse("#FF00FG").unwrap_err();
    assert_eq!(
        err.errors()[0],
        Error::ExpectedHex {
            found: Input::new_with_span("#FF00FG", (6..7).into())
        }
    );
}
