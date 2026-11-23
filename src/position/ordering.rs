//! The order, equality and hash of the positions.

use core::cmp::Ordering;
use core::hash::{Hash, Hasher};

use super::Position;

impl Position {
    /// The `N` digits without the leading zeroes and the `Z` digits without the trailing ones:
    /// two positions are equal when these are.
    fn significant(&self) -> (&[u16], &[u16]) {
        let n = self.n();
        let n = &n[n.iter().position(|d| *d != 0).unwrap_or(n.len())..];
        let z = self.z();
        let z = &z[..z.iter().rposition(|d| *d != 0).map_or(0, |i| i + 1)];
        (n, z)
    }
}

impl Ord for Position {
    /// Compares the positions as numbers, `N` is the integer part and `Z` the fractional part. Positions
    /// of the same system compare exactly as their strings.
    fn cmp(&self, other: &Self) -> Ordering {
        let (left, right) = (self.n(), other.n());
        // Aligns `N` by its least significant digit, the missing digits are zeroes
        let width = left.len().max(right.len());
        let digit = |section: &[u16], i: usize| {
            let missing = width - section.len();
            if i < missing { 0 } else { section[i - missing] }
        };
        for i in 0..width {
            match digit(left, i).cmp(&digit(right, i)) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        // Aligns `Z` by its most significant digit, the missing digits are zeroes
        let (left, right) = (self.z(), other.z());
        for i in 0..left.len().max(right.len()) {
            let l = left.get(i).copied().unwrap_or(0);
            let r = right.get(i).copied().unwrap_or(0);
            match l.cmp(&r) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Position {
    /// Two positions are equal when they have the same value, the zeroes at the end of `Z` don't
    /// change it (`"mmmmm!"` and `"mmmmm!!"` are equal).
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (n, z) = self.significant();
        n.hash(state);
        z.hash(state);
    }
}
