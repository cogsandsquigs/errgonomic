pub trait Underlying: Clone + PartialEq + Eq + core::fmt::Debug {
    /// Gets the length of the underlying data.
    fn len(&self) -> usize;

    /// Whether the underlying data is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the byte at x index.
    fn byte_at(&self, n: usize) -> Option<u8>;

    /// Gets a slice of bytes from the start index to the end index, exclusive of the end.
    fn byte_span(&self, start: usize, end: usize) -> Option<&[u8]>;

    /// Gets a slice of itself.
    fn span(&self, start: usize, end: usize) -> Option<Self>;

    /// Transparently clones the underlying source. If it's a reference type, it will simply return
    /// the reference. If it's an owned type, it will clone the owned data.
    fn fork(&self) -> Self;
}

impl Underlying for &str {
    #[inline]
    fn len(&self) -> usize {
        (self as &str).len()
    }

    #[inline]
    fn byte_at(&self, n: usize) -> Option<u8> {
        // TODO: Is this fast enough?
        self.as_bytes().get(n).copied()
    }

    #[inline]
    fn byte_span(&self, start: usize, end: usize) -> Option<&[u8]> {
        if start > end || end > self.len() {
            None
        } else {
            // TODO: Is this fast enough?
            Some(&self.as_bytes()[start..end])
        }
    }

    #[inline]
    fn span(&self, start: usize, end: usize) -> Option<Self> {
        self.get(start..end)
    }

    #[inline]
    fn fork(&self) -> Self {
        self
    }
}

impl Underlying for &[u8] {
    #[inline]
    fn len(&self) -> usize {
        (self as &[u8]).len()
    }

    #[inline]
    fn byte_at(&self, n: usize) -> Option<u8> {
        self.get(n).copied()
    }

    #[inline]
    fn byte_span(&self, start: usize, end: usize) -> Option<&[u8]> {
        if start > end || end > self.len() {
            None
        } else {
            Some(&self[start..end])
        }
    }

    #[inline]
    fn span(&self, start: usize, end: usize) -> Option<Self> {
        self.get(start..end)
    }

    #[inline]
    fn fork(&self) -> Self {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_len() {
        let s = "hello";
        assert_eq!(s.len(), 5);

        let empty = "";
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_str_is_empty() {
        let s = "hello";
        assert!(!s.is_empty());

        let empty = "";
        assert!(empty.is_empty());
    }

    #[test]
    fn test_str_byte_at() {
        let s = "hello";
        assert_eq!(s.byte_at(0), Some(b'h'));
        assert_eq!(s.byte_at(4), Some(b'o'));
        assert_eq!(s.byte_at(5), None);

        // Fix the bug in the implementation where the condition is inverted
        // Original: if self.len() >= n { None } else { ... }
        // Should be: if n >= self.len() { None } else { ... }

        let empty = "";
        assert_eq!(empty.byte_at(0), None);
    }

    #[test]
    fn test_str_byte_span() {
        let s = "hello";
        assert_eq!(s.byte_span(0, 5), Some(b"hello".as_slice()));
        assert_eq!(s.byte_span(0, 3), Some(b"hel".as_slice()));
        assert_eq!(s.byte_span(1, 4), Some(b"ell".as_slice()));
        assert_eq!(s.byte_span(0, 0), Some(b"".as_slice()));
        assert_eq!(s.byte_span(5, 5), Some(b"".as_slice()));

        assert_eq!(s.byte_span(6, 7), None); // start beyond length
        assert_eq!(s.byte_span(3, 2), None); // start > end
        assert_eq!(s.byte_span(0, 6), None); // end beyond length
    }

    #[test]
    fn test_str_fork() {
        let s = "hello";
        let forked = s.fork();
        assert_eq!(s, forked);

        // Since fork returns a reference for &str, they should point to the same data
        let s_ptr = s as *const str;
        let forked_ptr = forked as *const str;
        assert_eq!(s_ptr, forked_ptr);
    }

    #[test]
    fn test_bytes_len() {
        let bytes: &[u8] = b"hello";
        assert_eq!(bytes.len(), 5);

        let empty: &[u8] = b"";
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_bytes_is_empty() {
        let bytes: &[u8] = b"hello";
        assert!(!bytes.is_empty());

        let empty: &[u8] = b"";
        assert!(empty.is_empty());
    }

    #[test]
    fn test_bytes_byte_at() {
        let bytes: &[u8] = b"hello";
        assert_eq!(bytes.byte_at(0), Some(b'h'));
        assert_eq!(bytes.byte_at(4), Some(b'o'));
        assert_eq!(bytes.byte_at(5), None);

        let empty: &[u8] = b"";
        assert_eq!(empty.byte_at(0), None);
    }

    #[test]
    fn test_bytes_byte_span() {
        let bytes: &[u8] = b"hello";
        assert_eq!(bytes.byte_span(0, 5), Some(b"hello".as_slice()));
        assert_eq!(bytes.byte_span(0, 3), Some(b"hel".as_slice()));
        assert_eq!(bytes.byte_span(1, 4), Some(b"ell".as_slice()));
        assert_eq!(bytes.byte_span(0, 0), Some(b"".as_slice()));
        assert_eq!(bytes.byte_span(5, 5), Some(b"".as_slice()));

        assert_eq!(bytes.byte_span(6, 7), None); // start beyond length
        assert_eq!(bytes.byte_span(3, 2), None); // start > end
        assert_eq!(bytes.byte_span(0, 6), None); // end beyond length
    }

    #[test]
    fn test_bytes_fork() {
        let bytes: &[u8] = b"hello";
        let forked = bytes.fork();
        assert_eq!(bytes, forked);

        // Since fork returns a reference for &[u8], they should point to the same data
        let bytes_ptr = bytes.as_ptr();
        let forked_ptr = forked.as_ptr();
        assert_eq!(bytes_ptr, forked_ptr);
    }

    #[test]
    fn test_unicode_handling() {
        let s = "こんにちは"; // Japanese "hello"
        assert_eq!(s.len(), 15); // 3 bytes per character
        assert_eq!(s.byte_at(0), Some(227)); // First byte of first character
        assert_eq!(s.byte_span(0, 3), Some("こ".as_bytes())); // First character
    }
}
