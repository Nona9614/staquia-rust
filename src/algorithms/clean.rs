//! Shortens a middle value.

use crate::number::Digits;

/// Removes the least significant digits of `value` that are not needed to keep it
/// between `before` and `after`.
pub(crate) fn clean(value: Digits, before: &Digits, after: &Digits) -> Digits {
    let mut n = value.upper;
    let br = value.lower - 1;
    let mut before_index = br;
    let mut after_index = br;
    while before_index == br && after_index == br && n >= value.lower {
        let (b, v, a) = (before.get(n), value.get(n), after.get(n));
        if b < v {
            before_index = n;
        }
        if v < a {
            after_index = n;
        }
        if before_index != br || after_index != br {
            break;
        }
        n -= 1;
    }
    if before_index == br && after_index == br {
        return value;
    }
    if after_index == br {
        while after_index == br && n >= value.lower {
            if value.get(n) < after.get(n) {
                after_index = n;
                break;
            }
            n -= 1;
        }
        if after_index == br {
            return value;
        }
    } else {
        while before_index == br && n >= value.lower {
            if before.get(n) < value.get(n) {
                before_index = n;
                break;
            }
            n -= 1;
        }
        if before_index == br {
            return value;
        }
    }
    let mut number = Digits::new();
    number.upper = value.upper - n + value.lower;
    number.lower = value.lower;
    let mut i = value.upper;
    while i >= n {
        number.set(i - n + value.lower, value.get(i));
        i -= 1;
    }
    number
}
