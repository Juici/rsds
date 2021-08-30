use std::fmt;
use std::string::FromUtf16Error;

/// A UTF-16 string with a fixed capacity, `N`.
///
/// Terminated by `0x0000`.
#[derive(Clone, Copy, Eq)]
#[repr(transparent)]
pub struct Utf16<const N: usize> {
    chars: [u16; N],
}

impl<const N: usize> Utf16<N> {
    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        self.chars.iter().position(|&c| c == 0).unwrap_or(N)
    }

    /// Returns the string content.
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        let len = self.len();
        let chars = &self.chars[..len];

        String::from_utf16(chars)
    }

    /// Returns the string content, with invalid characters replaced by
    /// [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which looks like this: �.
    ///
    /// [U+FFFD]: char::REPLACEMENT_CHARACTER
    pub fn to_string_lossy(&self) -> String {
        let len = self.len();
        let chars = &self.chars[..len];

        String::from_utf16_lossy(chars)
    }
}

impl<const N: usize> fmt::Debug for Utf16<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string_lossy().fmt(f)
    }
}

impl<const N: usize> fmt::Display for Utf16<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string_lossy().fmt(f)
    }
}

impl<const N: usize> From<Utf16<N>> for [u16; N] {
    #[inline]
    fn from(ascii: Utf16<N>) -> Self {
        ascii.chars
    }
}

impl<const N: usize, const M: usize> PartialEq<Utf16<M>> for Utf16<N> {
    fn eq(&self, other: &Utf16<M>) -> bool {
        let self_len = self.len();
        let other_len = other.len();

        self_len == other_len && self.chars[..self_len] == other.chars[..other_len]
    }
}

impl<const N: usize> PartialEq<str> for Utf16<N> {
    fn eq(&self, other: &str) -> bool {
        match self.to_string() {
            Ok(s) => s == other,
            Err(_) => false,
        }
    }
}

impl<const N: usize> PartialEq<&str> for Utf16<N> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl<const N: usize> PartialEq<Utf16<N>> for str {
    #[inline]
    fn eq(&self, other: &Utf16<N>) -> bool {
        other == self
    }
}

impl<const N: usize> PartialEq<Utf16<N>> for &str {
    #[inline]
    fn eq(&self, other: &Utf16<N>) -> bool {
        other == self
    }
}
