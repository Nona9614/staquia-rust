use core::cmp::Ordering;

use crate::digits::{Algorithms, Digits};
use crate::error::{CreateError, Direction, PositionError};
use crate::position::Signature;
use crate::{MAX_LENGTH, Position};

/// The configuration of a [`Staquia`] system, for [`Staquia::create_with_options`].
///
/// Its [`Default`] is the default system, so only the fields that change need to be written.
///
/// # Example
///
/// ```
/// use staquia::{Options, Staquia};
///
/// // The default characters with a longer `N` section
/// let system = Staquia::create_with_options(Options { ln: 6, ..Options::default() })?;
/// assert_eq!(system.start()?.to_string(), "ʰʰʰʰʰʰ!");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    /// The character of the zero digit.
    pub zero: char,
    /// The character of the first digit (`1`).
    pub first: char,
    /// The character of the last digit (`base - 1`).
    pub last: char,
    /// The fixed length of the `N` section, used to stack items without making positions longer.
    pub ln: usize,
    /// The max length of the `Z` section, used when items are moved between two others.
    pub lz: usize,
    /// A factor from `0` to `1`: the threshold is reached at the length `ceil((1 - threshold) * (ln + lz))`.
    pub threshold: f64,
}

impl Default for Options {
    /// The default system: the zero `!`, the digits from `°` (`U+00B0`) to `ұ` (`U+04B1`), a base `1027`
    /// system, with `ln = 5`, `lz = 15` and a threshold of `0.15`.
    fn default() -> Self {
        Self { zero: '!', first: '\u{00B0}', last: '\u{04B1}', ln: 5, lz: 15, threshold: 0.15 }
    }
}

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
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Staquia {
    signature: Signature,
    lz: usize,
    lmax: usize,
    lthreshold: usize,
    factor: f64,
    algorithms: Algorithms,
}

impl Default for Staquia {
    /// The default system, ready for production, see [`Options::default`].
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::default();
    /// assert_eq!(system.start()?.to_string(), "ʰʰʰʰʰ!");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
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
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn create(zero: char, first: char, last: char, ln: usize, lz: usize, threshold: f64) -> Result<Self, CreateError> {
        Self::create_with_options(Options { zero, first, last, ln, lz, threshold })
    }

    /// Creates a number system from its [`Options`], see [`Staquia::create`].
    pub fn create_with_options(options: Options) -> Result<Self, CreateError> {
        let Options { zero, first, last, ln, lz, threshold } = options;
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
            signature: Signature { zero, first, last, ln: ln as u8 },
            lz,
            lmax,
            lthreshold: ((1.0 - threshold) * lmax as f64).ceil() as usize,
            factor: threshold,
            algorithms: Algorithms { base: last as i32 - first as i32 + 2 },
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
        self.algorithms.base as u32
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

    /// The character of the middle digit, the one repeated by [`start`](Self::start).
    pub fn middle(&self) -> char {
        let middle = (self.signature.last as u32 + self.signature.first as u32 - 1).div_ceil(2);
        // `first..=last` has no surrogates, so the middle is a character
        char::from_u32(middle).unwrap_or(self.signature.first)
    }

    /// The digit of a character, `None` if it is not a digit of the system.
    fn code(&self, ch: char) -> Option<u16> {
        if ch == self.signature.zero {
            Some(0)
        } else if (self.signature.first..=self.signature.last).contains(&ch) {
            Some((ch as u32 - self.signature.first as u32 + 1) as u16)
        } else {
            None
        }
    }

    /// Verifies the length of a new position, as the JavaScript `result`.
    fn result(&self, length: usize, position: impl FnOnce() -> Position) -> Result<Position, PositionError> {
        if length > self.lmax {
            return Err(PositionError::Overflow { length, limit: self.lmax });
        }
        let position = position();
        if length >= self.lthreshold {
            return Err(PositionError::ThresholdReached { position, length, threshold: self.lthreshold });
        }
        Ok(position)
    }

    /// Fails when the position was created by a system with another configuration.
    fn verify(&self, position: &Position) -> Result<(), PositionError> {
        if *position.signature() == self.signature {
            Ok(())
        } else {
            Err(PositionError::SystemMismatch { position: *position })
        }
    }

    /// The `N` section of a position as a number.
    fn n(position: &Position) -> Digits {
        Digits::from_slice(position.n())
    }

    /// Every digit of a position as a number (JavaScript `raw`, with the position's own length).
    fn raw(position: &Position) -> Digits {
        Digits::from_slice(position.digits())
    }

    /// Creates a position from a number of the algorithms (JavaScript `from`).
    fn compose(&self, number: &Digits) -> Result<Position, PositionError> {
        let length = (number.upper - number.lower + 1).max(0) as usize;
        let ln = self.ln() as i32;
        let mut n = Digits::new();
        n.upper = ln - 1;
        let mut z = Digits::new();
        if length > ln as usize {
            let offset = number.upper - n.upper;
            for i in (0..ln).rev() {
                n.set(i, number.get(i + offset));
            }
            let remaining = offset - number.lower;
            z.upper = remaining - 1;
            for k in 0..remaining {
                z.set(remaining - 1 - k, number.get(offset - 1 - k));
            }
        } else if length == ln as usize {
            for i in (number.lower..=number.upper).rev() {
                n.set(i, number.get(i));
            }
        } else {
            let offset = ln - length as i32;
            for i in (offset..ln).rev() {
                n.set(i, number.get(i - offset));
            }
        }
        self.result(length, || {
            Position::new(
                self.signature,
                (0..ln).rev().map(|i| n.get(i) as u16),
                (0..=z.upper).rev().map(|i| z.get(i) as u16),
            )
        })
    }

    /// Creates a position from its digits (`N` followed by `Z`, the most significant first), the inverse
    /// of [`Position::digits`]. `N` takes the `ln` most significant digits.
    ///
    /// # Example
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let start = system.start()?;
    ///
    /// // Rebuilds the same position from its digits
    /// let copy = system.from_digits(start.digits())?;
    /// assert_eq!(copy.to_string(), "mmmmm!");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::Overflow`] if the position is too long, [`PositionError::ThresholdReached`] if it
    /// reached the threshold. A digit bigger than the last digit of the system is reported as an
    /// [`PositionError::InvalidCharacter`] with the character it would have.
    pub fn from_digits(&self, digits: &[u16]) -> Result<Position, PositionError> {
        if digits.len() > self.lmax {
            return Err(PositionError::Overflow { length: digits.len(), limit: self.lmax });
        }
        if let Some(index) = digits.iter().position(|d| u32::from(*d) >= self.base()) {
            let character = char::from_u32(self.signature.first as u32 + u32::from(digits[index]) - 1)
                .unwrap_or(char::REPLACEMENT_CHARACTER);
            return Err(PositionError::InvalidCharacter { character, index });
        }
        self.compose(&Digits::from_slice(digits))
    }

    /// Creates a position from its string, for example one stored in a database.
    ///
    /// When the string is shorter than the `N` section (`ln` characters), `N` is completed with zeroes
    /// **to the right**, which keeps the string order (`"xy"` becomes `"xy!!!!"`).
    ///
    /// # Example
    ///
    /// ```
    /// use staquia::{PositionError, Staquia};
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    ///
    /// assert_eq!(system.forge("xy")?.to_string(), "xy!!!!");
    ///
    /// // Every character must be a digit of the system: the zero or a character from first to last
    /// let invalid = system.forge("abC");
    /// assert!(matches!(invalid, Err(PositionError::InvalidCharacter { character: 'C', index: 2 })));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::InvalidCharacter`] for a character that is not a digit of the system,
    /// [`PositionError::Overflow`] if it is too long, [`PositionError::ThresholdReached`] if it reached the
    /// threshold.
    pub fn forge(&self, string: &str) -> Result<Position, PositionError> {
        let mut length = 0usize;
        for (index, character) in string.chars().enumerate() {
            if self.code(character).is_none() {
                return Err(PositionError::InvalidCharacter { character, index });
            }
            length = index + 1;
        }
        let ln = self.ln();
        let codes = || string.chars().map(|ch| self.code(ch).unwrap_or(0));
        self.result(length, || {
            if length > ln {
                Position::new(self.signature, codes().take(ln), codes().skip(ln))
            } else {
                // `N` is completed with zeroes to the right, `Z` is a single zero
                Position::new(self.signature, codes().chain(core::iter::repeat(0)).take(ln), core::iter::once(0))
            }
        })
    }

    /// Creates the recommended first position of a list: the middle digit repeated in the whole `N`
    /// section, so there is the same room to stack items before and after it.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// // The middle digit of "a" to "z" (with the zero "!") is "m"
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.start()?.to_string(), "mmmmm!");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::ThresholdReached`] when `ln` alone reaches the threshold of the system.
    pub fn start(&self) -> Result<Position, PositionError> {
        let middle = self.code(self.middle()).unwrap_or(0);
        self.result(self.ln(), || {
            Position::new(self.signature, core::iter::repeat_n(middle, self.ln()), core::iter::once(0))
        })
    }

    /// Creates the position right after another, to stack an item at the end of a list.
    ///
    /// It adds `1` to the `N` section, so the position never gets longer (its `Z` section is dropped).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    ///
    /// let next = system.next(&system.start()?)?;
    /// assert_eq!(next.to_string(), "mmmmn!");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::EdgeReached`] when `N` is already the last value (every digit is `last`),
    /// [`PositionError::SystemMismatch`] if the position belongs to another system.
    pub fn next(&self, reference: &Position) -> Result<Position, PositionError> {
        self.verify(reference)?;
        // `n` has a fixed size, so its leading zeroes must be kept (no trim)
        let after = self.algorithms.after(&Self::n(reference), false);
        // A carry beyond the fixed `n` size means `n` was already the last value
        if after.upper >= self.ln() as i32 {
            return Err(PositionError::EdgeReached { position: *reference, direction: Direction::Next });
        }
        self.compose(&after)
    }

    /// Creates the position right before another, to stack an item at the start of a list.
    ///
    /// It subtracts `1` from the `N` section, so the position never gets longer (its `Z` section is dropped).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    ///
    /// let previous = system.previous(&system.start()?)?;
    /// assert_eq!(previous.to_string(), "mmmml!");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::EdgeReached`] when `N` is already the first value (every digit is the zero),
    /// [`PositionError::SystemMismatch`] if the position belongs to another system.
    pub fn previous(&self, reference: &Position) -> Result<Position, PositionError> {
        self.verify(reference)?;
        if reference.n().iter().all(|digit| *digit == 0) {
            return Err(PositionError::EdgeReached { position: *reference, direction: Direction::Previous });
        }
        let before = self.algorithms.before(&Self::n(reference), false);
        self.compose(&before)
    }

    /// Creates a position between two others, to move or insert an item between its neighbors. The order of
    /// the arguments doesn't matter.
    ///
    /// The room left in the `N` section is used first. When the positions are adjacent, the `Z` section takes
    /// the middle value, making the position one digit longer. Moving items again and again into the same gap
    /// makes positions grow until the threshold and the overflow.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let start = system.start()?; // "mmmmm!"
    /// let next = system.next(&start)?; // "mmmmn!"
    ///
    /// // Adjacent positions: Z takes the middle value
    /// assert_eq!(system.core(&start, &next)?.to_string(), "mmmmmn");
    ///
    /// // There is room in N between "a!!!!!" and "b!!!!!"
    /// let room = system.core(&system.forge("a")?, &system.forge("b")?)?;
    /// assert_eq!(room.to_string(), "ammmn!");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`PositionError::ThresholdReached`] or [`PositionError::Overflow`] when the gap is too small,
    /// [`PositionError::SystemMismatch`] if a position belongs to another system.
    pub fn core(&self, previous: &Position, next: &Position) -> Result<Position, PositionError> {
        self.verify(previous)?;
        self.verify(next)?;
        let (previous, next) = if previous.cmp(next) == Ordering::Greater { (next, previous) } else { (previous, next) };
        // Not enough room in `N` (a difference below 2): use the `Z` section
        let (difference, _) = self.algorithms.substract(&Self::n(previous), &Self::n(next));
        let number = if difference.upper == 0 && difference.get(0) < 2 {
            self.algorithms.diverge(&Self::raw(previous), &Self::raw(next))
        } else {
            self.algorithms.between(&Self::n(previous), &Self::n(next), true)
        };
        self.compose(&number)
    }

    /// Whether the position is longer than the maximum length of this system.
    pub fn has_overflow(&self, position: &Position) -> bool {
        position.len() > self.lmax
    }

    /// Whether the position reached the threshold length of this system.
    pub fn is_threshold_reached(&self, position: &Position) -> bool {
        position.len() >= self.lthreshold
    }

    /// Whether the position was created by this system (or one with the same configuration).
    pub fn owns(&self, position: &Position) -> bool {
        *position.signature() == self.signature
    }
}
