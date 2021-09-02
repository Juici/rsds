use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

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
    chars: [u8; N],
}

impl<const N: usize> Ascii<N> {
    /// Returns a reference to the inner byte representation.
    pub fn as_bytes(&self) -> &[u8; N] {
        &self.chars
    }

    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        // TODO: Custom optimised implementation.
        self.chars.iter().position(|&c| c == 0).unwrap_or(N)
    }

    /// Returns the string content.
    pub fn to_str(&self) -> Result<&str, AsciiError> {
        let len = self.len();
        let chars = &self.chars[..len];

        validate_ascii(chars).map(|_| unsafe { std::str::from_utf8_unchecked(chars) })
    }

    /// Returns the string content, with invalid characters replaced by
    /// [`U+FFFD REPLACEMENT CHARACTER`][U+FFFD], which looks like this: �.
    ///
    /// [U+FFFD]: char::REPLACEMENT_CHARACTER
    pub fn to_string_lossy(&self) -> Cow<'_, str> {
        let len = self.len();
        let chars = &self.chars[..len];

        match validate_ascii(chars) {
            Ok(()) => Cow::Borrowed(unsafe { std::str::from_utf8_unchecked(chars) }),
            Err(AsciiError { mut valid_up_to }) => {
                const REPLACEMENT: &str = "\u{FFFD}";

                let mut res = String::with_capacity(len);

                let mut remaining = chars;
                loop {
                    let valid = unsafe { std::str::from_utf8_unchecked(&remaining[..valid_up_to]) };
                    res.push_str(valid);
                    res.push_str(REPLACEMENT);

                    remaining = &remaining[(valid_up_to + 1)..];

                    match validate_ascii(remaining) {
                        Ok(()) => break,
                        Err(err) => valid_up_to = err.valid_up_to,
                    };
                }

                Cow::Owned(res)
            }
        }
    }
}

#[inline]
fn validate_ascii(chars: &[u8]) -> Result<(), AsciiError> {
    match chars.iter().position(|&c| c > 0x7F) {
        Some(valid_up_to) => Err(AsciiError { valid_up_to }),
        None => Ok(()),
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

impl<const N: usize> TryFrom<[u8; N]> for Ascii<N> {
    type Error = AsciiError;

    fn try_from(chars: [u8; N]) -> Result<Self, Self::Error> {
        validate_ascii(&chars).map(|_| Ascii { chars })
    }
}

impl<const N: usize> TryFrom<&[u8]> for Ascii<N> {
    type Error = AsciiError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut chars = [0; N];

        if value.len() >= N {
            chars.copy_from_slice(&value[..N]);
        } else {
            chars[..value.len()].copy_from_slice(value);
        }

        validate_ascii(&chars).map(|_| Ascii { chars })
    }
}

impl<const N: usize> From<Ascii<N>> for [u8; N] {
    #[inline]
    fn from(ascii: Ascii<N>) -> Self {
        ascii.chars
    }
}

impl<const N: usize, const M: usize> PartialEq<Ascii<M>> for Ascii<N> {
    fn eq(&self, other: &Ascii<M>) -> bool {
        let self_len = self.len();
        let other_len = other.len();

        self_len == other_len && self.chars[..self_len] == other.chars[..other_len]
    }
}

impl<const N: usize> PartialEq<str> for Ascii<N> {
    fn eq(&self, other: &str) -> bool {
        self.chars[..self.len()] == *other.as_bytes()
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
        &self.chars
    }
}
