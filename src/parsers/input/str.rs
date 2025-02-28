use super::Underlying;

impl Underlying for &str {
    type Glyph = char;

    fn len(&self) -> usize {
        (self as &str).len()
    }

    fn empty() -> Self {
        ""
    }

    fn transparent_clone(&self) -> Self {
        self
    }
}
