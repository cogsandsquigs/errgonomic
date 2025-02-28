pub mod str;

use core::fmt;

/// The underlying input type that the parsers will work with. When we talk about indexing, we are
/// considering indexes of `Glyph`s. `Ghyph`s are the elemntal unit of the type (i.e. for strings
/// it's `char`, for byte arrays it's `u8`).
pub trait Underlying: PartialEq + Eq + fmt::Debug {
    type Glyph;

    /// The length of the underlying input.
    fn len(&self) -> usize;

    /// Whether the underlying input is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The empty value of the underlying input.
    fn empty() -> Self;

    /// For types that are a reference, it should take a reference to itself. Otherwise, it should
    /// implement a clone.
    fn transparent_clone(&self) -> Self;
}

/// The input type that the parsers will work with. This is the type that will be passed to the
/// parsers.
#[derive(PartialEq, Eq, Debug)]
pub struct PInput<I: Underlying> {
    /// The underlying input.
    input: I,

    /// The head-index of the input, w.r.t the `Glyph`s. Essentially, the "start" of the input.
    pub(crate) head: usize,

    /// The tail-index of the input, w.r.t the `Glyph`s. Essentially, the "end" of the input. This
    /// is *exclusive* of the last `Glyph`, so `tail - 1` is the last glyph.
    pub(crate) tail: usize,
}

impl<I: Underlying> PInput<I> {
    pub fn new(input: I) -> Self {
        PInput {
            head: 0,
            tail: input.len(),
            input,
        }
    }

    pub fn new_at(input: I, index: usize) -> Self {
        PInput {
            head: index,
            tail: input.len(),
            input,
        }
    }

    pub fn len(&self) -> usize {
        self.tail - self.head
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn take(&self, n: usize) -> Self {
        PInput {
            input: self.input.transparent_clone(),
            head: self.head + n,
            tail: self.tail,
        }
    }

    pub fn take_from(&self, n: usize) -> Self {
        PInput {
            input: self.input.transparent_clone(),
            head: self.head,
            tail: self.tail - n,
        }
    }
}

/* TRAIT IMPLS */

impl<'a> From<&'a str> for PInput<&'a str> {
    fn from(input: &'a str) -> Self {
        PInput::new(input)
    }
}
