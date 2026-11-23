//! `left - right`.

use crate::number::Digits;

/// `left - right`, as its magnitude and whether it is negative.
pub(crate) fn substract(base: i32, left: &Digits, right: &Digits) -> (Digits, bool) {
    let mut result = Digits::new();
    let mut negative = false;
    let mut left = left.duplicate();
    let mut x = right.lower;
    while x <= right.upper {
        let mut value = left.get(x) - right.get(x);
        if value < 0 {
            left.update(x + 1, -1);
            value += base;
        }
        result.set(x, value);
        x += 1;
    }
    result.upper = x - 1;
    if left.get(x) == -1 {
        result.upper = x;
        negative = true;
        // The complement: `base^x - result`
        let mut restore = Digits::new();
        restore.set(x, 1);
        for i in 0..=x {
            let mut value = restore.get(i) - result.get(i);
            if value < 0 {
                restore.update(i + 1, -1);
                value += base;
            }
            result.set(i, value);
        }
    }
    result.compact();
    (result, negative)
}
