use core::cmp::Ordering;
use core::fmt::{self, Write};
use core::hash::{Hash, Hasher};

use crate::MAX_LENGTH;

/// Identifies the meaning of the digits of a system: its characters and the fixed length of `N`.
/// Systems with the same signature create compatible positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Signature {
    pub zero: char,
    pub first: char,
    pub last: char,
    pub ln: u8,
}

impl Signature {
    /// The character of a digit.
    #[inline]
    pub fn character(&self, digit: u16) -> char {
        if digit == 0 {
            self.zero
        } else {
            // Systems never include the surrogates, so every digit is a valid `char`
            char::from_u32(self.first as u32 + u32::from(digit) - 1).unwrap_or(char::REPLACEMENT_CHARACTER)
        }
    }
}

/// A position: a short string that keeps the order of a list.
///
/// Sort your items by their position string (or by the `Position` itself, it implements [`Ord`])
/// and you get the order back. A [`Staquia`](crate::Staquia) system creates them, they are never built by hand.
///
/// A position is plain data that lives on the stack (it implements [`Copy`]): its digits and the
/// signature of the system that created it, with no reference to that system. It prints as its string
/// with [`Display`](fmt::Display), so `position.to_string()` is the value to store in a database.
///
/// Its digits are split in two sections:
/// - `N`, with the fixed length `ln` of the system, used to stack items before and after others.
/// - `Z`, of at least one digit, used when items are moved between two others.
///
/// # Example
///
/// ```
/// use staquia::Staquia;
///
/// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
///
/// let start = system.start()?;
/// let next = system.next(&start)?;
///
/// assert_eq!(start.to_string(), "mmmmm!");
/// assert!(start < next);
/// # Ok::<(), staquia::Error>(())
/// ```
#[derive(Clone, Copy)]
pub struct Position {
    /// The digits: `N` (most significant first) followed by `Z`.
    digits: [u16; MAX_LENGTH],
    /// The number of digits (`N` plus `Z`).
    length: u8,
    signature: Signature,
}

impl Position {
    /// Creates a position from its digits, `n` is `N` and `z` is `Z`, both most significant first.
    /// The caller verified that both fit.
    pub(crate) fn new(signature: Signature, n: impl Iterator<Item = u16>, z: impl Iterator<Item = u16>) -> Self {
        let mut digits = [0u16; MAX_LENGTH];
        let mut length = 0usize;
        for digit in n.chain(z) {
            digits[length] = digit;
            length += 1;
        }
        Self { digits, length: length as u8, signature }
    }

    pub(crate) fn signature(&self) -> &Signature {
        &self.signature
    }

    /// The digits of the `N` section, the most significant first.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let position = system.forge("abcdefg")?;
    ///
    /// assert_eq!(position.n(), &[1, 2, 3, 4, 5]);
    /// assert_eq!(position.z(), &[6, 7]);
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn n(&self) -> &[u16] {
        &self.digits[..self.signature.ln as usize]
    }

    /// The digits of the `Z` section, the most significant first. It always has at least one digit.
    pub fn z(&self) -> &[u16] {
        &self.digits[self.signature.ln as usize..self.length as usize]
    }

    /// Every digit, `N` followed by `Z`. [`Staquia::from_digits`](crate::Staquia::from_digits) creates the
    /// position back from them.
    pub fn digits(&self) -> &[u16] {
        &self.digits[..self.length as usize]
    }

    /// The length of the position string, in characters (`N` plus `Z`).
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.length as usize
    }

    /// The characters of the position string.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.digits().iter().map(|digit| self.signature.character(*digit))
    }

    /// The `N` digits without the leading zeroes and the `Z` digits without the trailing ones:
    /// two positions are equal when these are.
    fn significant(&self) -> (&[u16], &[u16]) {
        let n = self.n();
        let n = &n[n.iter().position(|d| *d != 0).unwrap_or(n.len())..];
        let z = self.z();
        let z = &z[..z.iter().rposition(|d| *d != 0).map_or(0, |i| i + 1)];
        (n, z)
    }
}

impl Ord for Position {
    /// Compares the positions as numbers, `N` is the integer part and `Z` the fractional part. Positions
    /// of the same system compare exactly as their strings.
    fn cmp(&self, other: &Self) -> Ordering {
        let (left, right) = (self.n(), other.n());
        // Aligns `N` by its least significant digit, the missing digits are zeroes
        let width = left.len().max(right.len());
        let digit = |section: &[u16], i: usize| {
            let missing = width - section.len();
            if i < missing { 0 } else { section[i - missing] }
        };
        for i in 0..width {
            match digit(left, i).cmp(&digit(right, i)) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        // Aligns `Z` by its most significant digit, the missing digits are zeroes
        let (left, right) = (self.z(), other.z());
        for i in 0..left.len().max(right.len()) {
            let l = left.get(i).copied().unwrap_or(0);
            let r = right.get(i).copied().unwrap_or(0);
            match l.cmp(&r) {
                Ordering::Equal => {}
                ordering => return ordering,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Position {
    /// Two positions are equal when they have the same value, the zeroes at the end of `Z` don't
    /// change it (`"mmmmm!"` and `"mmmmm!!"` are equal).
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let (n, z) = self.significant();
        n.hash(state);
        z.hash(state);
    }
}

impl fmt::Display for Position {
    /// Writes the position string, the value to store.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in self.chars() {
            f.write_char(ch)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position(\"{self}\")")
    }
}
