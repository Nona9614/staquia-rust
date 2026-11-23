//! Helpers of the unit tests, the Rust version of the JavaScript `AlphabeticalDigits` mock: numbers written
//! with the alphabet, `!` is `0`, `a` is `1` and `z` is `26` (a base 27 system).

use crate::number::Digits;

/// The base of the alphabet.
pub(crate) const BASE: i32 = 27;

/// The number written with these letters, the most significant first (JavaScript `AlphabeticalDigits.from`).
pub(crate) fn alpha(letters: &str) -> Digits {
    let digits: Vec<u16> = letters
        .chars()
        .map(|ch| if ch == '!' { 0 } else { ch as u16 - 96 })
        .collect();
    Digits::from_slice(&digits)
}

/// The letters of a number from its upper to its lower bound (JavaScript `AlphabeticalDigits.toString`).
pub(crate) fn text(number: &Digits) -> String {
    (number.lower..=number.upper)
        .rev()
        .map(|i| letter(number.get(i)))
        .collect()
}

/// The letter of a digit.
pub(crate) fn letter(digit: i32) -> char {
    if digit == 0 {
        '!'
    } else {
        char::from_u32(digit as u32 + 96).unwrap_or('?')
    }
}
