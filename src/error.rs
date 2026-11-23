use core::fmt;

use crate::{MAX_LENGTH, Position};

/// The direction of [`Staquia::next`](crate::Staquia::next) and [`Staquia::previous`](crate::Staquia::previous).
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
    /// Whether it is an overflow: [`Overflow`](Self::Overflow) or [`EdgeReached`](Self::EdgeReached).
    pub fn is_overflow(&self) -> bool {
        matches!(self, Self::Overflow { .. } | Self::EdgeReached { .. })
    }

    /// Whether the threshold was reached, see [`ThresholdReached`](Self::ThresholdReached).
    pub fn is_dangerous(&self) -> bool {
        matches!(self, Self::ThresholdReached { .. })
    }

    /// Whether the position belongs to another system, see [`SystemMismatch`](Self::SystemMismatch).
    pub fn is_mismatch(&self) -> bool {
        matches!(self, Self::SystemMismatch { .. })
    }

    /// Whether the string had a character that is not a digit, see [`InvalidCharacter`](Self::InvalidCharacter).
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::InvalidCharacter { .. })
    }

    /// The position of the error, when it has one.
    pub fn position(&self) -> Option<&Position> {
        match self {
            Self::EdgeReached { position, .. } | Self::ThresholdReached { position, .. } | Self::SystemMismatch { position } => {
                Some(position)
            }
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
            Self::EdgeReached { position, direction } => {
                let (direction, edge) = match direction {
                    Direction::Next => ("next", "last"),
                    Direction::Previous => ("previous", "first"),
                };
                write!(
                    f,
                    "The position \"{position}\" has no {direction} value as its 'n' section is already the {edge} value of the system."
                )
            }
            Self::ThresholdReached { position, length, threshold } => write!(
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
                write!(f, "The characters must be zero < first <= last (got '{zero}', '{first}', '{last}').")
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
            Self::Threshold(factor) => write!(f, "The threshold factor must be from 0 to 1 (got {factor})."),
        }
    }
}

impl std::error::Error for CreateError {}

/// Why an operation of a [`LexicalList`](crate::LexicalList) failed. `K` is the type of the ids.
///
/// When an operation fails, neither the list nor its items are changed.
#[derive(Clone, Debug, PartialEq)]
pub enum LexicalError<K> {
    /// A position could not be created.
    Position(PositionError),
    /// There is no item with this id.
    NotFound(K),
    /// The item is already in the list.
    DuplicateId(K),
    /// `move_to` received no neighbor.
    MissingNeighbors(K),
    /// `move_to` received the item itself as a neighbor.
    SelfNeighbor(K),
    /// The neighbors given to `move_to` are not next to each other (with one neighbor: it is not the first
    /// or the last item).
    NotAdjacent {
        /// The previous neighbor.
        previous: Option<K>,
        /// The next neighbor.
        next: Option<K>,
    },
    /// `set` received an item without a valid position, use `set_untrusted` to repair it.
    InvalidPosition {
        /// The id of the item.
        id: K,
        /// Why the position is not valid, `None` when it is empty.
        reason: Option<PositionError>,
    },
    /// `set` received positions that are not in ascending order, use `set_untrusted` to sort and repair them.
    NotOrdered {
        /// The index of the first item out of order.
        index: usize,
        /// The id of the item before it.
        previous: K,
        /// The id of the item out of order.
        id: K,
    },
}

impl<K> From<PositionError> for LexicalError<K> {
    fn from(error: PositionError) -> Self {
        Self::Position(error)
    }
}

impl<K: fmt::Debug> fmt::Display for LexicalError<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Position(error) => error.fmt(f),
            Self::NotFound(id) => write!(f, "The item was not found using the id '{id:?}'."),
            Self::DuplicateId(id) => write!(f, "The item id '{id:?}' already exists in the list."),
            Self::MissingNeighbors(id) => {
                write!(f, "At least the previous or next item must be provided to move the item (id: {id:?}).")
            }
            Self::SelfNeighbor(id) => write!(f, "The item (id: {id:?}) can't be moved next to itself."),
            Self::NotAdjacent { previous, next } => match (previous, next) {
                (Some(previous), Some(next)) => {
                    write!(f, "The previous item (id: {previous:?}) and next item (id: {next:?}) are not adjacent.")
                }
                (Some(previous), None) => write!(
                    f,
                    "The previous item (id: {previous:?}) is not the last item, a next item must be provided."
                ),
                (None, Some(next)) => write!(
                    f,
                    "The next item (id: {next:?}) is not the first item, a previous item must be provided."
                ),
                (None, None) => write!(f, "The neighbors are not adjacent."),
            },
            Self::InvalidPosition { id, reason } => {
                write!(f, "The item (id: {id:?}) has an invalid position. ")?;
                match reason {
                    Some(reason) => reason.fmt(f)?,
                    None => f.write_str("The position is empty.")?,
                }
                f.write_str(" Use 'set_untrusted' to repair it.")
            }
            Self::NotOrdered { index, previous, id } => write!(
                f,
                "The positions are not in ascending order (index {index}: id {id:?} is not after id {previous:?}). Use 'set_untrusted' to sort and repair them."
            ),
        }
    }
}

impl<K: fmt::Debug> std::error::Error for LexicalError<K> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Position(error) => Some(error),
            Self::InvalidPosition { reason: Some(reason), .. } => Some(reason),
            _ => None,
        }
    }
}

/// An operation of a [`LexicalList`](crate::LexicalList) that took ownership of a value failed: the error
/// and the value, given back unchanged.
///
/// It converts into [`LexicalError`] with `?`, and [`into_value`](Self::into_value) gives the value back,
/// for example to retry after [`LexicalList::rebalance`](crate::LexicalList::rebalance).
#[derive(Clone, PartialEq)]
pub struct Rejected<K, V> {
    /// Why the operation failed.
    pub error: LexicalError<K>,
    /// The value that was not added.
    pub value: V,
}

impl<K, V> Rejected<K, V> {
    /// The value that was not added.
    pub fn into_value(self) -> V {
        self.value
    }
}

impl<K, V> From<Rejected<K, V>> for LexicalError<K> {
    fn from(rejected: Rejected<K, V>) -> Self {
        rejected.error
    }
}

impl<K: fmt::Debug, V> fmt::Display for Rejected<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl<K: fmt::Debug, V> fmt::Debug for Rejected<K, V> {
    /// Shows the error only, so the values don't need to implement `Debug`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rejected").field("error", &self.error).finish_non_exhaustive()
    }
}

impl<K: fmt::Debug, V> std::error::Error for Rejected<K, V> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.error {
            LexicalError::Position(error) => Some(error),
            _ => None,
        }
    }
}

/// Any error of the crate, so every operation can use `?` in the same function without boxing the errors.
/// `K` is the type of the ids of a [`LexicalList`](crate::LexicalList), `()` when no list is used.
///
/// # Example
///
/// ```
/// use staquia::Staquia;
///
/// fn first_two() -> Result<[String; 2], staquia::Error> {
///     let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?; // CreateError
///     let start = system.start()?; // PositionError
///     Ok([start.to_string(), system.next(&start)?.to_string()])
/// }
///
/// assert_eq!(first_two()?, ["mmmmm!", "mmmmn!"]);
/// # Ok::<(), staquia::Error>(())
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Error<K = ()> {
    /// A system could not be created.
    Create(CreateError),
    /// A position could not be created.
    Position(PositionError),
    /// An operation of a list failed.
    Lexical(LexicalError<K>),
}

impl<K> From<CreateError> for Error<K> {
    fn from(error: CreateError) -> Self {
        Self::Create(error)
    }
}

impl<K> From<PositionError> for Error<K> {
    fn from(error: PositionError) -> Self {
        Self::Position(error)
    }
}

impl<K> From<LexicalError<K>> for Error<K> {
    fn from(error: LexicalError<K>) -> Self {
        match error {
            LexicalError::Position(error) => Self::Position(error),
            error => Self::Lexical(error),
        }
    }
}

impl<K, V> From<Rejected<K, V>> for Error<K> {
    fn from(rejected: Rejected<K, V>) -> Self {
        rejected.error.into()
    }
}

impl<K: fmt::Debug> fmt::Display for Error<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create(error) => error.fmt(f),
            Self::Position(error) => error.fmt(f),
            Self::Lexical(error) => error.fmt(f),
        }
    }
}

impl<K: fmt::Debug> std::error::Error for Error<K> {}
