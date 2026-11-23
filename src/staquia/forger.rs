//! The functions that create positions (JavaScript `aggregates/forger`).

use core::cmp::Ordering;

use super::Staquia;
use crate::Position;
use crate::algorithms;
use crate::error::{Direction, PositionError};
use crate::number::Digits;

/// The `N` section of a position as a number.
fn n(position: &Position) -> Digits {
    Digits::from_slice(position.n())
}

/// Every digit of a position as a number (JavaScript `raw`, with the position's own length).
fn raw(position: &Position) -> Digits {
    Digits::from_slice(position.digits())
}

impl Staquia {
    /// Creates a position from a number of the algorithms (JavaScript `from`).
    fn compose(&self, number: &Digits) -> Result<Position, PositionError> {
        let length = (number.upper - number.lower + 1).max(0) as usize;
        let ln = self.ln() as i32;
        let mut n = Digits::new();
        n.upper = ln - 1;
        let mut z = Digits::new();
        if length > ln as usize {
            let offset = number.upper - n.upper;
            for i in (0..ln).rev() {
                n.set(i, number.get(i + offset));
            }
            let remaining = offset - number.lower;
            z.upper = remaining - 1;
            for k in 0..remaining {
                z.set(remaining - 1 - k, number.get(offset - 1 - k));
            }
        } else if length == ln as usize {
            for i in (number.lower..=number.upper).rev() {
                n.set(i, number.get(i));
            }
        } else {
            let offset = ln - length as i32;
            for i in (offset..ln).rev() {
                n.set(i, number.get(i - offset));
            }
        }
        self.result(length, || {
            Position::new(
                self.signature,
                (0..ln).rev().map(|i| n.get(i) as u16),
                (0..=z.upper).rev().map(|i| z.get(i) as u16),
            )
        })
    }

    /// Creates a position from its digits (`N` followed by `Z`, the most significant first), the inverse
    /// of [`Position::digits`]. `N` takes the `ln` most significant digits.
    ///
    /// # Example
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let start = system.start()?;
    ///
    /// // Rebuilds the same position from its digits
    /// let copy = system.from_digits(start.digits())?;
    /// assert_eq!(copy.to_string(), "mmmmm!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::Overflow`] if the position is too long, [`PositionError::ThresholdReached`] if it
    /// reached the threshold. A digit bigger than the last digit of the system is reported as an
    /// [`PositionError::InvalidCharacter`] with the character it would have.
    pub fn from_digits(&self, digits: &[u16]) -> Result<Position, PositionError> {
        if digits.len() > self.lmax {
            return Err(PositionError::Overflow {
                length: digits.len(),
                limit: self.lmax,
            });
        }
        if let Some(index) = digits.iter().position(|d| u32::from(*d) >= self.base()) {
            let character =
                char::from_u32(self.signature.first as u32 + u32::from(digits[index]) - 1)
                    .unwrap_or(char::REPLACEMENT_CHARACTER);
            return Err(PositionError::InvalidCharacter { character, index });
        }
        self.compose(&Digits::from_slice(digits))
    }

    /// Creates a position from its string, for example one stored in a database.
    ///
    /// When the string is shorter than the `N` section (`ln` characters), `N` is completed with zeroes
    /// **to the right**, which keeps the string order (`"xy"` becomes `"xy!!!!"`).
    ///
    /// # Example
    ///
    /// ```
    /// use staquia::{PositionError, Staquia};
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    ///
    /// assert_eq!(system.forge("xy")?.to_string(), "xy!!!!");
    ///
    /// // Every character must be a digit of the system: the zero or a character from first to last
    /// let invalid = system.forge("abC");
    /// assert!(matches!(invalid, Err(PositionError::InvalidCharacter { character: 'C', index: 2 })));
    /// # Ok::<(), staquia::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::InvalidCharacter`] for a character that is not a digit of the system,
    /// [`PositionError::Overflow`] if it is too long, [`PositionError::ThresholdReached`] if it reached the
    /// threshold.
    pub fn forge(&self, string: &str) -> Result<Position, PositionError> {
        let mut length = 0usize;
        for (index, character) in string.chars().enumerate() {
            if self.code(character).is_none() {
                return Err(PositionError::InvalidCharacter { character, index });
            }
            length = index + 1;
        }
        let ln = self.ln();
        let codes = || string.chars().map(|ch| self.code(ch).unwrap_or(0));
        self.result(length, || {
            if length > ln {
                Position::new(self.signature, codes().take(ln), codes().skip(ln))
            } else {
                // `N` is completed with zeroes to the right, `Z` is a single zero
                Position::new(
                    self.signature,
                    codes().chain(core::iter::repeat(0)).take(ln),
                    core::iter::once(0),
                )
            }
        })
    }

    /// Creates the recommended first position of a list: the middle digit repeated in the whole `N`
    /// section, so there is the same room to stack items before and after it.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// // The middle digit of "a" to "z" (with the zero "!") is "m"
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.start()?.to_string(), "mmmmm!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::ThresholdReached`] when `ln` alone reaches the threshold of the system.
    pub fn start(&self) -> Result<Position, PositionError> {
        let middle = self.code(self.middle()).unwrap_or(0);
        self.result(self.ln(), || {
            Position::new(
                self.signature,
                core::iter::repeat_n(middle, self.ln()),
                core::iter::once(0),
            )
        })
    }

    /// Creates the position right after another, to stack an item at the end of a list.
    ///
    /// It adds `1` to the `N` section, so the position never gets longer (its `Z` section is dropped).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    ///
    /// let next = system.next(&system.start()?)?;
    /// assert_eq!(next.to_string(), "mmmmn!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::EdgeReached`] when `N` is already the last value (every digit is `last`),
    /// [`PositionError::SystemMismatch`] if the position belongs to another system.
    pub fn next(&self, reference: &Position) -> Result<Position, PositionError> {
        self.verify(reference)?;
        // `n` has a fixed size, so its leading zeroes must be kept (no trim)
        let after = algorithms::after(self.base, &n(reference), false);
        // A carry beyond the fixed `n` size means `n` was already the last value
        if after.upper >= self.ln() as i32 {
            return Err(PositionError::EdgeReached {
                position: *reference,
                direction: Direction::Next,
            });
        }
        self.compose(&after)
    }

    /// Creates the position right before another, to stack an item at the start of a list.
    ///
    /// It subtracts `1` from the `N` section, so the position never gets longer (its `Z` section is dropped).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    ///
    /// let previous = system.previous(&system.start()?)?;
    /// assert_eq!(previous.to_string(), "mmmml!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::EdgeReached`] when `N` is already the first value (every digit is the zero),
    /// [`PositionError::SystemMismatch`] if the position belongs to another system.
    pub fn previous(&self, reference: &Position) -> Result<Position, PositionError> {
        self.verify(reference)?;
        if reference.n().iter().all(|digit| *digit == 0) {
            return Err(PositionError::EdgeReached {
                position: *reference,
                direction: Direction::Previous,
            });
        }
        let before = algorithms::before(self.base, &n(reference), false);
        self.compose(&before)
    }

    /// Creates a position between two others, to move or insert an item between its neighbors. The order of
    /// the arguments doesn't matter.
    ///
    /// The room left in the `N` section is used first. When the positions are adjacent, the `Z` section takes
    /// the middle value, making the position one digit longer. Moving items again and again into the same gap
    /// makes positions grow until the threshold and the overflow.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let start = system.start()?; // "mmmmm!"
    /// let next = system.next(&start)?; // "mmmmn!"
    ///
    /// // Adjacent positions: Z takes the middle value
    /// assert_eq!(system.core(&start, &next)?.to_string(), "mmmmmn");
    ///
    /// // There is room in N between "a!!!!!" and "b!!!!!"
    /// let room = system.core(&system.forge("a")?, &system.forge("b")?)?;
    /// assert_eq!(room.to_string(), "ammmn!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::ThresholdReached`] or [`PositionError::Overflow`] when the gap is too small,
    /// [`PositionError::SystemMismatch`] if a position belongs to another system.
    pub fn core(&self, previous: &Position, next: &Position) -> Result<Position, PositionError> {
        self.verify(previous)?;
        self.verify(next)?;
        let (previous, next) = if previous.cmp(next) == Ordering::Greater {
            (next, previous)
        } else {
            (previous, next)
        };
        // Not enough room in `N` (a difference below 2): use the `Z` section
        let (difference, _) = algorithms::substract(self.base, &n(previous), &n(next));
        let number = if difference.upper == 0 && difference.get(0) < 2 {
            algorithms::diverge(self.base, &raw(previous), &raw(next))
        } else {
            algorithms::between(self.base, &n(previous), &n(next), true)
        };
        self.compose(&number)
    }
}
