//! Moves the digits that reached the base to the next index.

use crate::number::Digits;

/// Moves to the next index every digit equal to or bigger than the base.
pub(crate) fn resolve_overflow(base: i32, number: &mut Digits) {
    let mut n = number.lower;
    while n <= number.upper {
        if number.get(n) >= base {
            number.update(n, -base);
            number.update(n + 1, 1);
        }
        n += 1;
    }
}
