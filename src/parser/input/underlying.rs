use crate::parser::span::Span;
use core::fmt::Debug;

/// Anything that implements `Underlying` can be used as an `Input` to the parser.
pub trait Underlying: PartialEq + Eq + Debug + Clone + Fancy {
    /// Get the length of the input.
    fn len(&self) -> usize;

    /// Check if the input is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Slice the input.
    fn slice(&self, span: Span) -> Self;

    /// Fork the input. Essentially, a transparent `Clone`. For types that are references, it
    /// should simply clone the reference. For types that are owned, it should clone the owned
    /// object.
    fn fork(&self) -> Self;

    /// Checks if the input itself is a string of digits or not.
    fn is_decimal(&self) -> bool;

    /// Checks if the input is a hex
    fn is_hex(&self) -> bool;

    /// Checks if the input is whitespace, but *not* newlines.
    fn is_whitespace_not_newline(&self) -> bool;

    /// Checks if the input is newlines, but *not* whitespace.
    fn is_newline(&self) -> bool;

    /// Checks if the input is whitespace, including newlines.
    fn is_whitespace(&self) -> bool;
}

impl Underlying for &str {
    fn len(&self) -> usize {
        (self as &str).len()
    }

    fn slice(&self, span: Span) -> Self {
        &self[span.head..span.tail]
    }

    fn fork(&self) -> Self {
        self
    }

    fn is_decimal(&self) -> bool {
        self.is_ascii() && self.as_bytes().iter().all(|c| c.is_ascii_digit())
    }

    fn is_hex(&self) -> bool {
        self.is_ascii() && self.as_bytes().iter().all(|c| c.is_ascii_hexdigit())
    }

    // TODO: explore ways to make this faster - SIMD would be a big win
    fn is_whitespace_not_newline(&self) -> bool {
        self.is_ascii()
            && self
                .as_bytes()
                .iter()
                .all(|c| c.is_ascii_whitespace() && !(*c == b'\n' || *c == b'\r'))
    }

    // TODO: explore ways to make this faster - SIMD would be a big win
    fn is_newline(&self) -> bool {
        self.is_ascii() && self.as_bytes().iter().all(|c| *c == b'\n' || *c == b'\r')
    }

    // TODO: explore ways to make this faster - SIMD would be a big win
    fn is_whitespace(&self) -> bool {
        self.is_ascii() && self.as_bytes().iter().all(|c| c.is_ascii_whitespace())
    }
}

#[cfg(not(feature = "fancy"))]
impl Underlying for &[u8] {
    fn len(&self) -> usize {
        (self as &[u8]).len()
    }

    fn slice(&self, span: Span) -> Self {
        &self[span.head..span.tail]
    }

    fn fork(&self) -> Self {
        self
    }

    fn is_decimal(&self) -> bool {
        self.iter().all(|c| c.is_ascii_digit())
    }

    fn is_hex(&self) -> bool {
        self.iter().all(|c| c.is_ascii_hexdigit())
    }

    // TODO: explore ways to make this faster - SIMD would be a big win
    fn is_whitespace_not_newline(&self) -> bool {
        self.iter()
            .all(|c| c.is_ascii_whitespace() && !(*c == b'\n' || *c == b'\r'))
    }

    // TODO: explore ways to make this faster - SIMD would be a big win
    fn is_newline(&self) -> bool {
        self.iter().all(|c| *c == b'\n' || *c == b'\r')
    }

    // TODO: explore ways to make this faster - SIMD would be a big win
    fn is_whitespace(&self) -> bool {
        self.iter().all(|c| c.is_ascii_whitespace())
    }
}

// This trait controls the `Display` implementation for `Underlying` types. When the feature-flag
// `fancy` is enabled, it will require the `Underlying` type to implement `core::fmt::Display`.
// Since bytes (`[u8]`) are not `Display`, we need to disable the `fancy` feature for them.

#[cfg(not(feature = "fancy"))]
pub trait Fancy {}
#[cfg(feature = "fancy")]
pub trait Fancy: core::fmt::Display {
    /// The current line number of the input.
    fn current_line_num(&self) -> usize;

    /// The current column number of the input.
    fn current_col_num(&self) -> usize;

    /// The bytes of the input.
    fn data<'a>(&'a self) -> &'a [u8];
}

#[cfg(not(feature = "fancy"))]
impl Fancy for &str {}

#[cfg(feature = "fancy")]
impl Fancy for &str {
    // TODO: Make this faster? SIMD?
    fn current_line_num(&self) -> usize {
        self.bytes().filter(|c| *c == b'\n').count() + 1
    }

    // TODO: Make this faster? SIMD?
    fn current_col_num(&self) -> usize {
        // If the "last line" isn't included, then it must simply be a newline without anything
        // after it. Therefore, we are at column 0.
        if self.lines().count() < self.current_line_num() {
            0
        } else {
            self.lines().last().map(|l| l.len()).unwrap_or(0)
        }
    }

    fn data(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(not(feature = "fancy"))]
impl Fancy for &[u8] {}

#[cfg(feature = "fancy")]
#[cfg(test)] // NOTE: Not in `cfg_attr` since we want this to disappear when `fancy` is disabled.
mod fancy_tests {
    use super::*;

    #[test]
    fn can_get_correct_lines() {
        let s = "test";
        assert_eq!(s.current_line_num(), 1);

        let s = "test\ntest";
        assert_eq!(s.current_line_num(), 2);

        let s = "test\ntest\r\n";
        assert_eq!(s.current_line_num(), 3);
    }

    #[test]
    fn can_get_correct_cols() {
        let s = "test";
        assert_eq!(s.current_col_num(), 4);

        let s = "test\ntes";
        assert_eq!(s.current_col_num(), 3);

        let s = "test\ntest\r\n";
        assert_eq!(s.current_col_num(), 0);
    }
}
