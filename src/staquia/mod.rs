//! The number system that creates the positions.

mod forger;
mod lexicon;
mod options;
mod verifier;

pub use options::Options;

use crate::error::{CreateError, PositionError};
use crate::position::Signature;
use crate::{MAX_LENGTH, Position};

/// A lexicographic number system that creates **positions**: short strings that keep the order of a list.
///
/// Sort the items by their position string and you get the order back; moving an item only changes its own
/// position. Create one with [`Staquia::create`] (or [`Staquia::default`]), then use it to create
/// [`Position`]s, or give it to a [`LexicalList`](crate::LexicalList) to keep the positions of your values
/// in sync. Nothing panics: every function that can fail returns a [`Result`].
///
/// A system is a small [`Copy`] value, it never allocates.
///
/// # Example
///
/// ```
/// use staquia::Staquia;
///
/// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
///
/// let start = system.start()?; // "mmmmm!"
/// let next = system.next(&start)?; // "mmmmn!", stacked after start
/// let core = system.core(&start, &next)?; // "mmmmmn", between both
///
/// assert_eq!(core.to_string(), "mmmmmn");
/// assert!(start < core && core < next);
/// # Ok::<(), staquia::Error>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Staquia {
    signature: Signature,
    lz: usize,
    lmax: usize,
    lthreshold: usize,
    factor: f64,
    /// The base of the system, the arithmetic of [`algorithms`](crate::algorithms) works with it.
    base: i32,
}

impl Default for Staquia {
    /// The default system, ready for production, see [`Options::default`].
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::default();
    /// assert_eq!(system.start()?.to_string(), "ʰʰʰʰʰ!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    fn default() -> Self {
        match Self::create_with_options(Options::default()) {
            Ok(system) => system,
            Err(_) => unreachable!("the default options are valid"),
        }
    }
}

impl Staquia {
    /// Creates a number system. Its digits are the `zero` and every character from `first` to `last`,
    /// so its base is `(last - first) + 2` (by their unicode values).
    ///
    /// - `ln`: the fixed length of the `N` section, used to stack items without making positions longer.
    /// - `lz`: the max length of the `Z` section, used when items are moved between two others.
    /// - `threshold`: a factor from `0` to `1`, the threshold is reached at the length
    ///   `ceil((1 - threshold) * (ln + lz))`.
    ///
    /// The characters must be `zero < first <= last`, up to `U+FFFF` and the range from `first` to `last`
    /// can't include the surrogates, so Rust, JavaScript, UTF-8 and UTF-16 compare them in the same order.
    /// `ln + lz` can be at most [`MAX_LENGTH`].
    ///
    /// # Example
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// // The alphabet: "!" is 0, "a" is 1 and "z" is 26, a base 27 system
    /// let alphabet = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(alphabet.base(), 27);
    /// assert_eq!(alphabet.start()?.to_string(), "mmmmm!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn create(
        zero: char,
        first: char,
        last: char,
        ln: usize,
        lz: usize,
        threshold: f64,
    ) -> Result<Self, CreateError> {
        Self::create_with_options(Options {
            zero,
            first,
            last,
            ln,
            lz,
            threshold,
        })
    }

    /// Creates a number system from its [`Options`], see [`Staquia::create`].
    pub fn create_with_options(options: Options) -> Result<Self, CreateError> {
        let Options {
            zero,
            first,
            last,
            ln,
            lz,
            threshold,
        } = options;
        for ch in [zero, first, last] {
            if ch as u32 > 0xFFFF {
                return Err(CreateError::Character(ch));
            }
        }
        if !(zero < first && first <= last) {
            return Err(CreateError::Order { zero, first, last });
        }
        // Surrogates are not characters, they can't be digits
        if (first as u32) < 0xE000 && (last as u32) >= 0xD800 {
            return Err(CreateError::Character(last));
        }
        if ln < 1 || lz < 1 || ln + lz > MAX_LENGTH {
            return Err(CreateError::Length { ln, lz });
        }
        if !(0.0..=1.0).contains(&threshold) {
            return Err(CreateError::Threshold(threshold));
        }
        let lmax = ln + lz;
        Ok(Self {
            signature: Signature {
                zero,
                first,
                last,
                ln: ln as u8,
            },
            lz,
            lmax,
            lthreshold: ((1.0 - threshold) * lmax as f64).ceil() as usize,
            factor: threshold,
            base: last as i32 - first as i32 + 2,
        })
    }

    /// The options of this system.
    pub fn options(&self) -> Options {
        Options {
            zero: self.signature.zero,
            first: self.signature.first,
            last: self.signature.last,
            ln: self.ln(),
            lz: self.lz,
            threshold: self.factor,
        }
    }

    /// The character of the zero digit.
    pub fn zero(&self) -> char {
        self.signature.zero
    }

    /// The character of the first digit.
    pub fn first(&self) -> char {
        self.signature.first
    }

    /// The character of the last digit.
    pub fn last(&self) -> char {
        self.signature.last
    }

    /// The base of the system: the zero plus the characters from first to last.
    pub fn base(&self) -> u32 {
        self.base as u32
    }

    /// The fixed length of the `N` section.
    pub fn ln(&self) -> usize {
        self.signature.ln as usize
    }

    /// The max length of the `Z` section.
    pub fn lz(&self) -> usize {
        self.lz
    }

    /// The max length of a position, `ln + lz`.
    pub fn max_length(&self) -> usize {
        self.lmax
    }

    /// The length at which a position reaches the threshold.
    pub fn threshold_length(&self) -> usize {
        self.lthreshold
    }

    /// Verifies the length of a new position, as the JavaScript `result`.
    fn result(
        &self,
        length: usize,
        position: impl FnOnce() -> Position,
    ) -> Result<Position, PositionError> {
        if length > self.lmax {
            return Err(PositionError::Overflow {
                length,
                limit: self.lmax,
            });
        }
        let position = position();
        if length >= self.lthreshold {
            return Err(PositionError::ThresholdReached {
                position,
                length,
                threshold: self.lthreshold,
            });
        }
        Ok(position)
    }

    /// Fails when the position was created by a system with another configuration.
    fn verify(&self, position: &Position) -> Result<(), PositionError> {
        if *position.signature() == self.signature {
            Ok(())
        } else {
            Err(PositionError::SystemMismatch {
                position: *position,
            })
        }
    }
}
