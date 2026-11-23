//! The characters of a system and their digits (JavaScript `aggregates/lexicon`).

use super::Staquia;

impl Staquia {
    /// The character of the middle digit, the one repeated by [`start`](Self::start).
    pub fn middle(&self) -> char {
        let middle = (self.signature.last as u32 + self.signature.first as u32 - 1).div_ceil(2);
        // `first..=last` has no surrogates, so the middle is a character
        char::from_u32(middle).unwrap_or(self.signature.first)
    }

    /// The digit of a character, `None` if it is not a digit of the system.
    pub(super) fn code(&self, ch: char) -> Option<u16> {
        if ch == self.signature.zero {
            Some(0)
        } else if (self.signature.first..=self.signature.last).contains(&ch) {
            Some((ch as u32 - self.signature.first as u32 + 1) as u16)
        } else {
            None
        }
    }
}
