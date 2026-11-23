//! `value + 1`.

use crate::number::Digits;

/// `value + 1`.
pub(crate) fn after(base: i32, value: &Digits, trim: bool) -> Digits {
    let mut number = value.duplicate();
    number.update(0, 1);
    if trim {
        number.compact();
    }
    number.expand(1);
    super::resolve_overflow(base, &mut number);
    if number.get(number.upper) == 0 {
        number.pop();
    }
    number
}
