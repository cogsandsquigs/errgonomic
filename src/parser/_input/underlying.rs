use core::fmt::Debug;
use std::str::Chars;

/// Anything that implements `Underlying` can be used as an `Input` to the parser.
pub trait Underlying: Debug + Clone + Iterator<Item: UnderlyingItem> {}

impl Underlying for Chars<'_> {}

pub trait UnderlyingItem: PartialEq + Eq + Debug + Clone + Copy {
    /// Checks if the item is whitespace, including newlines.
    fn is_whitespace(&self) -> bool;

    /// Checks if the item is a newline.
    fn is_newline(&self) -> bool;

    /// Checks if the item is a digit with radix `n`.
    fn is_digit(&self, radix: u32) -> bool;

    /// Checks if the item is a decimal digit
    fn is_decimal(&self) -> bool {
        self.is_digit(10)
    }

    /// Checks if the item is a hexadecimal digit
    fn is_hex_digit(&self) -> bool {
        self.is_digit(16)
    }
}

impl UnderlyingItem for char {
    fn is_whitespace(&self) -> bool {
        char::is_whitespace(*self)
    }

    fn is_newline(&self) -> bool {
        *self == '\n' // || *self == '\r'
    }

    fn is_digit(&self, radix: u32) -> bool {
        char::is_digit(*self, radix)
    }
}

impl UnderlyingItem for u8 {
    fn is_whitespace(&self) -> bool {
        char::is_whitespace(*self as char)
    }

    fn is_newline(&self) -> bool {
        *self == b'\n' // || *self == b'\r'
    }

    fn is_digit(&self, radix: u32) -> bool {
        char::is_digit(*self as char, radix)
    }
}

/*
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
*/
