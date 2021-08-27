use std::convert::TryFrom;
use std::fmt;

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
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Ascii<const N: usize> {
    chars: [u8; N],
}

impl<const N: usize> Ascii<N> {
    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        memchr::memchr(0, &self.chars).unwrap_or(N)
    }

    /// Returns the string content.
    pub fn as_str(&self) -> &str {
        let len = self.len();
        unsafe { std::str::from_utf8_unchecked(&self.chars[..len]) }
    }
}

impl<const N: usize> fmt::Debug for Ascii<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl<const N: usize> fmt::Display for Ascii<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
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

#[inline]
fn validate_ascii(chars: &[u8]) -> Result<(), AsciiError> {
    match chars.iter().position(|&c| c > 0x7F) {
        Some(valid_up_to) => Err(AsciiError { valid_up_to }),
        None => Ok(()),
    }
}
