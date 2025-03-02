use crate::parser::span::Span;
use core::fmt::Debug;

/// Anything that implements `Underlying` can be used as an `Input` to the parser.
pub trait Underlying: PartialEq + Eq + Debug + Clone {
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

    fn is_whitespace_not_newline(&self) -> bool {
        self.is_ascii()
            && self
                .as_bytes()
                .iter()
                .all(|c| c.is_ascii_whitespace() && !(*c == b'\n' || *c == b'\r'))
    }

    fn is_newline(&self) -> bool {
        self.is_ascii() && self.as_bytes().iter().all(|c| *c == b'\n' || *c == b'\r')
    }

    fn is_whitespace(&self) -> bool {
        self.is_ascii() && self.as_bytes().iter().all(|c| c.is_ascii_whitespace())
    }
}

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

    fn is_whitespace_not_newline(&self) -> bool {
        self.iter()
            .all(|c| c.is_ascii_whitespace() && !(*c == b'\n' || *c == b'\r'))
    }

    fn is_newline(&self) -> bool {
        self.iter().all(|c| *c == b'\n' || *c == b'\r')
    }

    fn is_whitespace(&self) -> bool {
        self.iter().all(|c| c.is_ascii_whitespace())
    }
}
