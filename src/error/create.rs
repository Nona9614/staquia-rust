//! The errors of the systems.

use core::fmt;

use crate::MAX_LENGTH;

/// Why [`Staquia::create`](crate::Staquia::create) rejected a configuration.
///
/// The checks are cheap and run once, when the system is created, so every system that exists is valid.
///
/// # Example
///
/// ```
/// use staquia::{CreateError, Staquia};
///
/// // The zero must be smaller than the digits
/// assert!(matches!(Staquia::create('a', '!', 'z', 5, 15, 0.15), Err(CreateError::Order { .. })));
///
/// // Characters above U+FFFF compare differently in UTF-16 (JavaScript) and UTF-8
/// assert_eq!(Staquia::create('!', 'a', '😀', 5, 15, 0.15), Err(CreateError::Character('😀')));
///
/// // `N` and `Z` need at least one digit, and at most 32 in total
/// assert_eq!(Staquia::create('!', 'a', 'z', 0, 15, 0.15), Err(CreateError::Length { ln: 0, lz: 15 }));
///
/// // The threshold is a factor from 0 to 1
/// assert_eq!(Staquia::create('!', 'a', 'z', 5, 15, 1.5), Err(CreateError::Threshold(1.5)));
/// ```
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
