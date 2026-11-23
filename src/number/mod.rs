//! The internal numbers of the algorithms (JavaScript `Digits`).
//!
//! A port of the JavaScript `Digits` class: a number of any base stored by the index of each digit
//! (`0` is the least significant, negative indexes are fractional digits) with a `lower` and an `upper`
//! bound. The JavaScript version is a sparse `Map`; here it is a fixed array on the stack where a missing
//! digit is `0`, so every algorithm works without allocating.
//!
//! The methods keep the exact semantics of the JavaScript ones, including their quirks (for example
//! `extend` always shifts from the index `0`), because the positions must be identical in both languages.

mod compare;
mod shape;

/// The number of slots, enough for the indexes used by the algorithms with positions of up to
/// `MAX_LENGTH` digits (from about `-MAX_LENGTH - 1` to `2 * MAX_LENGTH + 2`).
pub(crate) const CAPACITY: usize = 128;
/// The slot of the index `0`.
const OFFSET: i32 = 48;

/// A number of any base, see the module documentation.
pub(crate) struct Digits {
    map: [i32; CAPACITY],
    /// The index of the least significant digit.
    pub lower: i32,
    /// The index of the most significant digit.
    pub upper: i32,
}

impl Digits {
    /// An empty number: no digits and both bounds at `0` (JavaScript `create`).
    pub fn new() -> Self {
        Self {
            map: [0; CAPACITY],
            lower: 0,
            upper: 0,
        }
    }

    /// Creates the number with these digits, the most significant first, at the indexes
    /// `digits.len() - 1` down to `0`.
    pub fn from_slice(digits: &[u16]) -> Self {
        let mut number = Self::new();
        let length = digits.len() as i32;
        number.upper = length - 1;
        for (i, digit) in digits.iter().enumerate() {
            number.set(length - 1 - i as i32, i32::from(*digit));
        }
        number
    }

    #[inline]
    fn slot(key: i32) -> Option<usize> {
        let slot = key + OFFSET;
        if (0..CAPACITY as i32).contains(&slot) {
            Some(slot as usize)
        } else {
            None
        }
    }

    /// The digit at `key`, `0` if it was never set.
    #[inline]
    pub fn get(&self, key: i32) -> i32 {
        match Self::slot(key) {
            Some(slot) => self.map[slot],
            None => 0,
        }
    }

    /// Sets the digit at `key`. The bounds are not changed.
    #[inline]
    pub fn set(&mut self, key: i32, value: i32) {
        match Self::slot(key) {
            Some(slot) => self.map[slot] = value,
            // The capacity covers every index the algorithms reach with valid systems
            None => debug_assert!(value == 0, "digit index {key} out of the capacity"),
        }
    }

    /// Adds `offset` to the digit at `key`.
    #[inline]
    pub fn update(&mut self, key: i32, offset: i32) {
        self.set(key, self.get(key) + offset);
    }

    /// Removes the digit at `key` (it becomes `0`).
    #[inline]
    pub fn remove(&mut self, key: i32) {
        self.set(key, 0);
    }

    /// A copy with only the digits inside the bounds (JavaScript `clone`).
    pub fn duplicate(&self) -> Self {
        let mut number = Self::new();
        for i in self.lower..=self.upper {
            number.set(i, self.get(i));
        }
        number.lower = self.lower;
        number.upper = self.upper;
        number
    }

    /// A copy of every digit, also the ones outside the bounds (JavaScript `reference`).
    pub fn take(&self) -> Self {
        Self {
            map: self.map,
            lower: self.lower,
            upper: self.upper,
        }
    }
}
