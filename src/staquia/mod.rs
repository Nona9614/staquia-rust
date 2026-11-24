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
    ///
    /// # Errors
    ///
    /// [`CreateError::Order`], [`CreateError::Character`], [`CreateError::Length`] or
    /// [`CreateError::Threshold`] when the configuration breaks one of the rules above.
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

    /// Creates a number system from its [`Options`]: the same as [`Staquia::create`], but only the fields that
    /// differ from the default system need to be written.
    ///
    /// ```
    /// use staquia::{Options, Staquia};
    ///
    /// // The default characters, with room for more moves into the same gap
    /// let system = Staquia::create_with_options(Options { lz: 25, ..Options::default() })?;
    /// assert_eq!(system.max_length(), 30);
    ///
    /// // Every field, the same system as `Staquia::create('!', 'a', 'z', 5, 15, 0.15)`
    /// let alphabet = Staquia::create_with_options(Options {
    ///     zero: '!',
    ///     first: 'a',
    ///     last: 'z',
    ///     ln: 5,
    ///     lz: 15,
    ///     threshold: 0.15,
    /// })?;
    /// assert_eq!(alphabet.base(), 27);
    /// # Ok::<(), staquia::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// The same as [`Staquia::create`].
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

    /// The options of this system, for example to create a similar one with
    /// [`create_with_options`](Self::create_with_options).
    ///
    /// ```
    /// use staquia::{Options, Staquia};
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.options().first, 'a');
    ///
    /// // The same characters with a longer `N` section
    /// let wider = Staquia::create_with_options(Options { ln: 8, ..system.options() })?;
    /// assert_eq!(wider.start()?.to_string(), "mmmmmmmm!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
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
    ///
    /// It is the digit `0`: it only appears in the `Z` section (`start` ends with it) and when a stored string is
    /// completed with [`forge`](Self::forge).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.zero(), '!');
    /// assert_eq!(system.forge("ab")?.to_string(), "ab!!!!");
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn zero(&self) -> char {
        self.signature.zero
    }

    /// The character of the first digit (`1`), the smallest character after the zero.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.first(), 'a');
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn first(&self) -> char {
        self.signature.first
    }

    /// The character of the last digit (`base - 1`), the largest character of the system.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.last(), 'z');
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn last(&self) -> char {
        self.signature.last
    }

    /// The base of the system: the zero plus the characters from `first` to `last`, `(last - first) + 2`.
    ///
    /// A bigger base gives more room in each digit, so positions stay shorter for the same number of items and
    /// moves.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// // "!" plus the 26 letters from "a" to "z"
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.base(), 27);
    ///
    /// assert_eq!(Staquia::default().base(), 1027);
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn base(&self) -> u32 {
        self.base as u32
    }

    /// The fixed length of the `N` section.
    ///
    /// `N` is used to stack items before or after the others ([`next`](Self::next) and
    /// [`previous`](Self::previous)), so it never grows: with base `b` it holds `b^ln` values.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.ln(), 5);
    /// assert_eq!(system.start()?.n().len(), 5);
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn ln(&self) -> usize {
        self.signature.ln as usize
    }

    /// The max length of the `Z` section.
    ///
    /// `Z` grows by one digit each time an item is moved between two adjacent positions
    /// ([`core`](Self::core)), so `lz` limits how many times the same gap can be split before the list needs a
    /// [`rebalance`](crate::LexicalList::rebalance).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.lz(), 15);
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn lz(&self) -> usize {
        self.lz
    }

    /// The max length of a position, `ln + lz`. A longer position is a [`PositionError::Overflow`].
    ///
    /// ```
    /// use staquia::{PositionError, Staquia};
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.max_length(), 20);
    ///
    /// let too_long = "a".repeat(21);
    /// assert!(matches!(system.forge(&too_long), Err(PositionError::Overflow { length: 21, limit: 20, .. })));
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn max_length(&self) -> usize {
        self.lmax
    }

    /// The length at which a position reaches the threshold: `ceil((1 - threshold) * (ln + lz))`.
    ///
    /// A position of this length or longer is still created, but returned inside a
    /// [`PositionError::ThresholdReached`], a warning that the list is close to an overflow and should be
    /// [rebalanced](crate::LexicalList::rebalance).
    ///
    /// ```
    /// use staquia::{PositionError, Staquia};
    ///
    /// // ceil((1 - 0.15) * 20) = 17
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.threshold_length(), 17);
    ///
    /// let long = "a".repeat(17);
    /// assert!(matches!(system.forge(&long), Err(PositionError::ThresholdReached { length: 17, .. })));
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn threshold_length(&self) -> usize {
        self.lthreshold
    }

    /// Verifies the length of a new position, as the JavaScript `result`.
    fn result(
        &self,
        length: usize,
        position: impl FnOnce() -> Position,
    ) -> Result<Position, PositionError> {
        let position = position().measured_as(length);
        if length > self.lmax {
            return Err(PositionError::Overflow {
                position: position.truncated(self.lmax + 1),
                length,
                limit: self.lmax,
            });
        }
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

#[cfg(test)]
mod tests;
