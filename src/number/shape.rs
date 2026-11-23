//! The methods that change the length of a number.

use super::Digits;

impl Digits {
    /// Removes the most significant digit.
    pub fn pop(&mut self) {
        self.remove(self.upper);
        self.upper -= 1;
    }

    /// Removes the zeroes at the left; a number of only zeroes becomes the single digit `0`.
    pub fn compact(&mut self) {
        let mut upper = self.upper;
        let mut n = self.upper;
        while n >= self.lower {
            if self.get(n) == 0 {
                self.remove(n);
                upper -= 1;
            } else {
                break;
            }
            n -= 1;
        }
        if upper < 0 {
            self.lower = 0;
            upper = 0;
            self.set(0, 0);
        }
        self.upper = upper;
    }

    /// Adds `size` zeroes at the left.
    pub fn expand(&mut self, size: i32) {
        let upper = self.upper;
        self.upper = upper + size;
        for i in upper + 1..size + upper + 1 {
            self.set(i, 0);
        }
    }

    /// Shifts the digits from the index `0` by `size` and fills the gap with zeroes.
    pub fn extend(&mut self, size: i32) {
        let upper = self.upper;
        self.upper = size + upper;
        let mut n = upper;
        while n >= 0 {
            self.set(n + size, self.get(n));
            n -= 1;
        }
        for n in 0..size {
            self.set(n, 0);
        }
    }

    /// Writes the digits of `other` at the left of this number.
    pub fn stack(&mut self, other: &Digits) {
        let mut upper = self.upper;
        for i in other.lower..=other.upper {
            upper += 1;
            self.set(upper, other.get(i));
        }
        self.upper = upper;
    }
}
