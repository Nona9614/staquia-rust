//! The positions created by a system.

mod ordering;
mod signature;

pub(crate) use signature::Signature;

use core::fmt::{self, Write};

use crate::MAX_LENGTH;

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
    pub(crate) fn new(
        signature: Signature,
        n: impl Iterator<Item = u16>,
        z: impl Iterator<Item = u16>,
    ) -> Self {
        let mut digits = [0u16; MAX_LENGTH];
        let mut length = 0usize;
        for digit in n.chain(z) {
            digits[length] = digit;
            length += 1;
        }
        Self {
            digits,
            length: length as u8,
            signature,
        }
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
    ///
    /// `Z` is the part that grows when an item is moved between two others; a stacked position has a single zero
    /// digit there.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let start = system.start()?; // "mmmmm!"
    /// let next = system.next(&start)?; // "mmmmn!"
    /// let core = system.core(&start, &next)?; // "mmmmmn"
    ///
    /// assert_eq!(start.z(), &[0]);
    /// assert_eq!(core.z(), &[14]); // "n"
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn z(&self) -> &[u16] {
        &self.digits[self.signature.ln as usize..self.length as usize]
    }

    /// Every digit, `N` followed by `Z`, the most significant first. The zero character is `0`, `first` is `1`
    /// and `last` is `base - 1`.
    ///
    /// [`Staquia::from_digits`](crate::Staquia::from_digits) creates the position back from them, for example
    /// to store a position as numbers.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let position = system.forge("abc")?;
    /// assert_eq!(position.digits(), &[1, 2, 3, 0, 0, 0]);
    ///
    /// let again = system.from_digits(position.digits())?;
    /// assert_eq!(again, position);
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn digits(&self) -> &[u16] {
        &self.digits[..self.length as usize]
    }

    /// The length of the position string, in characters (`N` plus `Z`). It is never empty: it always has `ln + 1`
    /// characters or more.
    ///
    /// The length is what the [threshold](crate::Staquia::threshold_length) and the
    /// [max length](crate::Staquia::max_length) of a system are compared with.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert_eq!(system.start()?.len(), 6); // "mmmmm!"
    ///
    /// // "a" and "b" are adjacent, the middle needs one more digit
    /// let a = system.forge("mmmmma")?;
    /// let b = system.forge("mmmmmb")?;
    /// assert_eq!(system.core(&a, &b)?.to_string(), "mmmmmam");
    /// assert_eq!(system.core(&a, &b)?.len(), 7);
    /// # Ok::<(), staquia::Error>(())
    /// ```
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.length as usize
    }

    /// The characters of the position string, without allocating a `String`.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let start = system.start()?;
    ///
    /// assert_eq!(start.chars().next(), Some('m'));
    /// assert_eq!(start.chars().collect::<String>(), start.to_string());
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.digits()
            .iter()
            .map(|digit| self.signature.character(*digit))
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
