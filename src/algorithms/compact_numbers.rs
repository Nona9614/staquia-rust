//! Aligns two numbers and moves their common digits apart.

use core::cmp::Ordering;

use crate::number::{CAPACITY, Digits};

/// How `compact_numbers` aligns two numbers of different lengths.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum HalfMode {
    /// Adds zeroes at the left of the shorter number (integers).
    Between,
    /// Adds zeroes at the right of the shorter number (fractions).
    Diverge,
}

/// Aligns both numbers, orders them (`number1 <= number2`) and moves their common most
/// significant digits to `repeated`.
pub(crate) fn compact_numbers(
    number1: &mut Digits,
    number2: &mut Digits,
    repeated: &mut Digits,
    mode: HalfMode,
) {
    let diff = number1.upper - number2.upper;
    match mode {
        HalfMode::Between => {
            if diff < 0 {
                number1.expand(-diff);
            } else if diff > 0 {
                number2.expand(diff);
            }
        }
        HalfMode::Diverge => {
            if diff < 0 {
                number1.extend(-diff);
            } else if diff > 0 {
                number2.extend(diff);
            }
        }
    }
    if number1.compare(number2) == Ordering::Greater {
        let temp = number1.duplicate();
        *number1 = number2.take();
        *number2 = temp;
    }
    // The common digits, from the most significant
    let mut codes = [0i32; CAPACITY];
    let mut count = 0usize;
    let mut index = number1.upper;
    while index > -1 {
        let (code1, code2) = (number1.get(index), number2.get(index));
        if code1 == code2 {
            number1.pop();
            number2.pop();
            codes[count] = code1;
            count += 1;
            index -= 1;
        } else {
            break;
        }
    }
    repeated.upper = count as i32 - 1;
    let mut ix = repeated.upper;
    for code in &codes[..count] {
        repeated.set(ix, *code);
        ix -= 1;
    }
}
