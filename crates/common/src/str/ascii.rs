use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::str;

/// An error in an ASCII string.
#[derive(Clone, Copy, Debug)]
pub struct AsciiError {
    valid_up_to: usize,
}

impl AsciiError {
    /// Returns the index in the string up to which valid ASCII was verified.
    pub fn valid_up_to(&self) -> usize {
        self.valid_up_to
    }
}

impl fmt::Display for AsciiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid ascii at index {}", self.valid_up_to)
    }
}

/// An ASCII string with a fixed capacity, `N`.
///
/// Terminated by `0x00` byte.
#[derive(Clone, Copy, Eq)]
#[repr(transparent)]
pub struct Ascii<const N: usize> {
    pub buf: [u8; N],
}

impl<const N: usize> Ascii<N> {
    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        // TODO: Custom optimised implementation.
        self.buf.iter().position(|&c| c == 0).unwrap_or(N)
    }

    /// Converts the string into a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len()]
    }

    /// Returns the string content.
    pub fn to_str(&self) -> Result<&str, AsciiError> {
        validate_ascii(self.as_bytes())
    }

    /// Returns the string content, with invalid characters replaced by
    /// [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which looks like this: �.
    ///
    /// [U+FFFD]: char::REPLACEMENT_CHARACTER
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        let bytes = self.as_bytes();

        match validate_ascii(bytes) {
            Ok(s) => Cow::Borrowed(s),
            Err(AsciiError { mut valid_up_to }) => {
                const REPLACEMENT: &str = "\u{FFFD}";

                let mut res = String::with_capacity(bytes.len());
                let mut remaining = bytes;
                loop {
                    // Push the valid content.
                    // SAFETY: We have validated the string up to `valid_up_to`.
                    res.push_str(unsafe { str::from_utf8_unchecked(&remaining[..valid_up_to]) });
                    // Add replacement character.
                    res.push_str(REPLACEMENT);

                    // Update the remaining contents.
                    remaining = &remaining[(valid_up_to + 1)..];

                    match validate_ascii(remaining) {
                        Ok(s) => {
                            // Push remaining valid content.
                            res.push_str(s);
                            break;
                        }
                        Err(err) => valid_up_to = err.valid_up_to,
                    }
                }

                Cow::Owned(res)
            }
        }
    }
}

#[inline]
fn validate_ascii(bytes: &[u8]) -> Result<&str, AsciiError> {
    match bytes.iter().position(|&c| c > 0x7F) {
        Some(valid_up_to) => Err(AsciiError { valid_up_to }),
        // SAFETY: We just validated the bytes.
        None => Ok(unsafe { str::from_utf8_unchecked(bytes) }),
    }
}

impl<const N: usize> fmt::Debug for Ascii<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string_lossy().fmt(f)
    }
}

impl<const N: usize> fmt::Display for Ascii<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string_lossy().fmt(f)
    }
}

impl<const N: usize, const M: usize> PartialEq<Ascii<M>> for Ascii<N> {
    fn eq(&self, other: &Ascii<M>) -> bool {
        let self_len = self.len();
        let other_len = other.len();
        self_len == other_len && self.buf[..self_len] == other.buf[..other_len]
    }
}

impl<const N: usize> PartialEq<str> for Ascii<N> {
    fn eq(&self, other: &str) -> bool {
        self.buf[..self.len()] == *other.as_bytes()
    }
}

impl<const N: usize> PartialEq<&str> for Ascii<N> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl<const N: usize> PartialEq<Ascii<N>> for str {
    #[inline]
    fn eq(&self, other: &Ascii<N>) -> bool {
        other == self
    }
}

impl<const N: usize> PartialEq<Ascii<N>> for &str {
    #[inline]
    fn eq(&self, other: &Ascii<N>) -> bool {
        other == self
    }
}

impl<const N: usize> Deref for Ascii<N> {
    type Target = [u8; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}
