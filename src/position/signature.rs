//! The signature of the system that created a position.

/// Identifies the meaning of the digits of a system: its characters and the fixed length of `N`.
/// Systems with the same signature create compatible positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Signature {
    pub zero: char,
    pub first: char,
    pub last: char,
    pub ln: u8,
}

impl Signature {
    /// The character of a digit.
    #[inline]
    pub fn character(&self, digit: u16) -> char {
        if digit == 0 {
            self.zero
        } else {
            // Systems never include the surrogates, so every digit is a valid `char`
            char::from_u32(self.first as u32 + u32::from(digit) - 1)
                .unwrap_or(char::REPLACEMENT_CHARACTER)
        }
    }
}
