//! `value - 1`.

use crate::number::Digits;

/// `value - 1`.
pub(crate) fn before(base: i32, value: &Digits, trim: bool) -> Digits {
    let mut number = value.duplicate();
    number.update(0, -1);
    if trim {
        number.compact();
    }
    number.expand(1);
    let mut n = number.lower;
    while n <= number.upper {
        if number.get(n) < 0 {
            number.update(n, base);
            number.update(n + 1, -1);
        } else {
            break;
        }
        n += 1;
    }
    if number.get(number.upper) <= 0 {
        number.pop();
    }
    number
}
