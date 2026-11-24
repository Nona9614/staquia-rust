//! The characters of a system and their digits (JavaScript `aggregates/lexicon`).

use super::Staquia;
use crate::position::set_digit;

impl Staquia {
    /// The character of the middle digit, the one repeated by [`start`](Self::start).
    ///
    /// Starting a list in the middle of `N` leaves as much room to stack items before the first one as after
    /// the last one.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.middle(), 'm');
    /// assert_eq!(system.start()?.to_string(), "mmmmm!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn middle(&self) -> char {
        if self.signature.set.is_some() {
            // The middle digit of `1..=base - 1`, the same digit the range formula gives
            return self
                .signature
                .character(((self.base - 1) as u32).div_ceil(2) as u16);
        }
        let middle = (self.signature.last() as u32 + self.signature.first() as u32 - 1).div_ceil(2);
        // `first..=last` has no surrogates, so the middle is a character
        char::from_u32(middle).unwrap_or(self.signature.first())
    }

    /// The digit of a character, `None` if it is not a digit of the system.
    pub(super) fn code(&self, ch: char) -> Option<u16> {
        if let Some(set) = self.signature.set {
            set_digit(set, ch)
        } else if ch == self.signature.zero() {
            Some(0)
        } else if (self.signature.first()..=self.signature.last()).contains(&ch) {
            Some((ch as u32 - self.signature.first() as u32 + 1) as u16)
        } else {
            None
        }
    }
}
