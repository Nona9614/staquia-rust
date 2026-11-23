//! The middle of two aligned numbers.

use crate::number::Digits;

/// The middle of two aligned numbers, with one fractional digit (index `-1`) when the
/// half is not exact, unless it can be rounded.
pub(crate) fn half(base: i32, before: &Digits, after: &Digits, round: bool) -> Digits {
    let mut value = Digits::new();
    value.lower = -1;
    value.upper = before.upper;
    // The JavaScript version works with halves (`x.5`), these are doubled to stay integers
    let mut trailing2 = 0;
    let mut i = value.upper;
    while i >= value.lower {
        let q2 = after.get(i) + before.get(i) + trailing2;
        let m2 = q2.rem_euclid(2);
        value.set(i, (q2 - m2) / 2);
        trailing2 = m2 * base;
        i -= 1;
    }
    let has_decimals = value.get(-1) != 0;
    let (substraction, negative) = super::substract(base, after, before);
    let roundable = !negative && !(substraction.upper == 0 && substraction.get(0) < 3);
    if !has_decimals {
        value.remove(-1);
        value.lower = 0;
    }
    if has_decimals && round && roundable {
        value.remove(-1);
        value.lower = 0;
        value.update(0, 1);
    }
    super::resolve_overflow(base, &mut value);
    value
}
