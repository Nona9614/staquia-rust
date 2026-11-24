//! The signature of the system that created a position.

/// Identifies the meaning of the digits of a system: its characters and the fixed length of `N`.
/// Systems with the same signature create compatible positions.
///
/// The digits are the `zero` and either every character from `first` to `last` (a range), or the characters of
/// `set` (the zero followed by the digits, sorted by their code). A set that is a range is always stored as one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Signature {
    // The characters are up to `U+FFFF` (validated by `create`): kept as 2 bytes, positions stay small
    zero: u16,
    first: u16,
    last: u16,
    pub ln: u8,
    pub set: Option<&'static &'static str>,
}

/// A character of the Basic Multilingual Plane as its code.
#[inline]
fn code(ch: char) -> u16 {
    u16::try_from(u32::from(ch)).unwrap_or(u16::MAX)
}

/// A code back to its character.
#[inline]
fn character(code: u16) -> char {
    char::from_u32(u32::from(code)).unwrap_or(char::REPLACEMENT_CHARACTER)
}

/// The character of a digit of a set, `None` past its end.
#[inline]
pub(crate) fn set_character(set: &str, digit: u16) -> Option<char> {
    let digit = usize::from(digit);
    if set.is_ascii() {
        set.as_bytes().get(digit).map(|byte| char::from(*byte))
    } else {
        set.chars().nth(digit)
    }
}

/// The digit of a character of a set, `None` if the set doesn't have it.
#[inline]
pub(crate) fn set_digit(set: &str, ch: char) -> Option<u16> {
    let index = if set.is_ascii() {
        // Sorted by their code, so a binary search finds them
        u8::try_from(ch)
            .ok()
            .and_then(|byte| set.as_bytes().binary_search(&byte).ok())
    } else {
        set.chars().position(|item| item == ch)
    }?;
    u16::try_from(index).ok()
}

impl Signature {
    /// The signature of a system: its characters must be up to `U+FFFF`.
    pub fn new(
        zero: char,
        first: char,
        last: char,
        ln: u8,
        set: Option<&'static &'static str>,
    ) -> Self {
        Self {
            zero: code(zero),
            first: code(first),
            last: code(last),
            ln,
            set,
        }
    }

    /// The character of the zero digit.
    #[inline]
    pub fn zero(&self) -> char {
        character(self.zero)
    }

    /// The character of the first digit.
    #[inline]
    pub fn first(&self) -> char {
        character(self.first)
    }

    /// The character of the last digit.
    #[inline]
    pub fn last(&self) -> char {
        character(self.last)
    }

    /// The character of a digit.
    #[inline]
    pub fn character(&self, digit: u16) -> char {
        if let Some(set) = self.set {
            set_character(set, digit).unwrap_or(char::REPLACEMENT_CHARACTER)
        } else if digit == 0 {
            self.zero()
        } else {
            // Systems never include the surrogates, so every digit is a valid `char`
            char::from_u32(u32::from(self.first) + u32::from(digit) - 1)
                .unwrap_or(char::REPLACEMENT_CHARACTER)
        }
    }
}
