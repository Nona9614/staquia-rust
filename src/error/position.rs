//! The errors of the positions.

use core::fmt;

use crate::Position;

/// The direction of [`Staquia::next`](crate::Staquia::next) and
/// [`Staquia::previous`](crate::Staquia::previous), given by [`PositionError::EdgeReached`] to tell which end
/// of `N` was reached.
///
/// # Example
///
/// ```
/// use staquia::{Direction, PositionError, Staquia};
///
/// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
/// let first = system.forge("!!!!!")?; // the first value of N
///
/// match system.previous(&first) {
///     Err(PositionError::EdgeReached { direction, .. }) => assert_eq!(direction, Direction::Previous),
///     other => panic!("unexpected {other:?}"),
/// }
/// # Ok::<(), staquia::Error>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Towards the last value of the system.
    Next,
    /// Towards the first value of the system.
    Previous,
}

/// Why a position could not be created.
///
/// Every function of [`Staquia`](crate::Staquia) that creates a position returns it in a [`Result`]
/// instead of panicking, so the problem is checked where it happens.
///
/// # Example
///
/// ```
/// use staquia::{PositionError, Staquia};
///
/// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
/// let last = system.forge("zzzzz")?;
///
/// match system.next(&last) {
///     Ok(position) => println!("{position}"),
///     Err(PositionError::EdgeReached { .. }) => println!("\"zzzzz\" is already the last value of N"),
///     Err(error) => println!("{error}"),
/// }
/// # Ok::<(), staquia::Error>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionError {
    /// A character of the string is not a digit of the system (the zero or a character from first to last).
    InvalidCharacter {
        /// The invalid character.
        character: char,
        /// Its index in the string, in characters.
        index: usize,
    },
    /// The position would be longer than the maximum length of the system (`ln + lz`).
    Overflow {
        /// The length of the position.
        length: usize,
        /// The maximum length of the system.
        limit: usize,
    },
    /// `N` is already the last value (for `next`) or the first value (for `previous`) of the system.
    EdgeReached {
        /// The reference position.
        position: Position,
        /// The direction that has no more values.
        direction: Direction,
    },
    /// The position reached the threshold length of the system: it is valid, but the list is getting close
    /// to an overflow and its positions should be spaced again (see
    /// [`LexicalList::rebalance`](crate::LexicalList::rebalance)).
    ThresholdReached {
        /// The created position, still valid.
        position: Position,
        /// Its length.
        length: usize,
        /// The threshold length of the system.
        threshold: usize,
    },
    /// The position was created by a system with another configuration, positions of different systems
    /// can't be mixed.
    SystemMismatch {
        /// The foreign position.
        position: Position,
    },
}

impl PositionError {
    /// Whether it is an overflow: [`Overflow`](Self::Overflow) or [`EdgeReached`](Self::EdgeReached), the same
    /// as the JavaScript `OverflowError`. There is no room for the position: space the list again with
    /// [`LexicalList::rebalance`](crate::LexicalList::rebalance).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let last = system.forge("zzzzz")?;
    ///
    /// assert!(system.next(&last).unwrap_err().is_overflow());
    /// assert!(system.forge(&"a".repeat(21)).unwrap_err().is_overflow());
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn is_overflow(&self) -> bool {
        matches!(self, Self::Overflow { .. } | Self::EdgeReached { .. })
    }

    /// Whether the threshold was reached, see [`ThresholdReached`](Self::ThresholdReached), the same as the
    /// JavaScript `DangerousError`. The position is still valid and can be taken from the error with
    /// [`position`](Self::position).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let error = system.forge(&"a".repeat(17)).unwrap_err();
    ///
    /// assert!(error.is_dangerous());
    /// assert!(error.position().is_some());
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn is_dangerous(&self) -> bool {
        matches!(self, Self::ThresholdReached { .. })
    }

    /// Whether the position belongs to another system, see [`SystemMismatch`](Self::SystemMismatch), the same as
    /// the JavaScript `MismatchError`.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let foreign = Staquia::default().start()?;
    ///
    /// assert!(system.next(&foreign).unwrap_err().is_mismatch());
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn is_mismatch(&self) -> bool {
        matches!(self, Self::SystemMismatch { .. })
    }

    /// Whether the string had a character that is not a digit, see [`InvalidCharacter`](Self::InvalidCharacter),
    /// the same as the JavaScript `InvalidError`.
    ///
    /// ```
    /// use staquia::{PositionError, Staquia};
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let error = system.forge("abC").unwrap_err();
    ///
    /// assert!(error.is_invalid());
    /// assert_eq!(error, PositionError::InvalidCharacter { character: 'C', index: 2 });
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::InvalidCharacter { .. })
    }

    /// The position of the error, when it has one: the reference position of
    /// [`EdgeReached`](Self::EdgeReached), the created (and valid) position of
    /// [`ThresholdReached`](Self::ThresholdReached), or the foreign position of
    /// [`SystemMismatch`](Self::SystemMismatch).
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    ///
    /// // The threshold was reached, but the position can still be used
    /// let position = match system.forge(&"a".repeat(17)) {
    ///     Ok(position) => position,
    ///     Err(error) if error.is_dangerous() => *error.position().unwrap(),
    ///     Err(error) => return Err(error.into()),
    /// };
    /// assert_eq!(position.len(), 17);
    ///
    /// // An invalid string has no position
    /// assert_eq!(system.forge("?").unwrap_err().position(), None);
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn position(&self) -> Option<&Position> {
        match self {
            Self::EdgeReached { position, .. }
            | Self::ThresholdReached { position, .. }
            | Self::SystemMismatch { position } => Some(position),
            Self::InvalidCharacter { .. } | Self::Overflow { .. } => None,
        }
    }
}

impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter { character, index } => write!(
                f,
                "The character '{character}' at index {index} is not a digit of the system (the zero or a character from first to last)."
            ),
            Self::Overflow { length, limit } => write!(
                f,
                "The position caused an overflow as its length '{length}' is bigger than the allowed system length '{limit}'."
            ),
            Self::EdgeReached {
                position,
                direction,
            } => {
                let (direction, edge) = match direction {
                    Direction::Next => ("next", "last"),
                    Direction::Previous => ("previous", "first"),
                };
                write!(
                    f,
                    "The position \"{position}\" has no {direction} value as its 'n' section is already the {edge} value of the system."
                )
            }
            Self::ThresholdReached {
                position,
                length,
                threshold,
            } => write!(
                f,
                "The position \"{position}\" with '{length}' reached the threshold '{threshold}'."
            ),
            Self::SystemMismatch { position } => write!(
                f,
                "The position \"{position}\" was created by a system with a different configuration, positions of different systems can't be mixed."
            ),
        }
    }
}

impl std::error::Error for PositionError {}
