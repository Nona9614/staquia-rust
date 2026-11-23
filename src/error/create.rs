//! The errors of the systems.

use core::fmt;

use crate::MAX_LENGTH;

/// Why [`Staquia::create`](crate::Staquia::create) rejected a configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CreateError {
    /// The characters must be `zero < first <= last`.
    Order {
        /// The zero character.
        zero: char,
        /// The first character.
        first: char,
        /// The last character.
        last: char,
    },
    /// The characters must be in the Basic Multilingual Plane (up to `U+FFFF`) and the range from first to
    /// last can't include the surrogates (`U+D800` to `U+DFFF`), so every language compares them in the same order.
    Character(char),
    /// `ln` and `lz` must be at least `1` and `ln + lz` at most [`MAX_LENGTH`].
    Length {
        /// The length of `N`.
        ln: usize,
        /// The length of `Z`.
        lz: usize,
    },
    /// The threshold factor must be a number from `0` to `1`.
    Threshold(f64),
}

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Order { zero, first, last } => {
                write!(
                    f,
                    "The characters must be zero < first <= last (got '{zero}', '{first}', '{last}')."
                )
            }
            Self::Character(ch) => write!(
                f,
                "The character U+{:04X} can't be used: use characters up to U+FFFF and leave the surrogates (U+D800 to U+DFFF) out of the range.",
                *ch as u32
            ),
            Self::Length { ln, lz } => write!(
                f,
                "The lengths must be ln >= 1, lz >= 1 and ln + lz <= {MAX_LENGTH} (got ln = {ln}, lz = {lz})."
            ),
            Self::Threshold(factor) => write!(
                f,
                "The threshold factor must be from 0 to 1 (got {factor})."
            ),
        }
    }
}

impl std::error::Error for CreateError {}
