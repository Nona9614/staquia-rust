//! The errors of the lists.

use core::fmt;

use super::PositionError;

/// Why an operation of a [`LexicalList`](crate::LexicalList) failed. `K` is the type of the ids, so the
/// error tells which item caused it.
///
/// When an operation fails, neither the list nor its items are changed, so it is safe to fix the cause and
/// try again. It converts into [`Error`](crate::Error) with `?`.
///
/// # Example
///
/// ```
/// # use staquia::{Item, LexicalList, Position, Staquia};
/// # #[derive(Debug)]
/// # struct Task { id: u32, position: String }
/// # impl Item for Task {
/// #     type Id = u32;
/// #     fn id(&self) -> &u32 { &self.id }
/// #     fn position(&self) -> &str { &self.position }
/// #     fn set_position(&mut self, position: &Position) { self.position = position.to_string(); }
/// # }
/// # let task = |id| Task { id, position: String::new() };
/// use staquia::LexicalError;
///
/// let mut list = LexicalList::new(Staquia::create('!', 'a', 'z', 5, 15, 0.15)?);
/// list.push(task(1))?;
/// list.push(task(2))?;
/// list.push(task(3))?;
///
/// // There is no task 9
/// assert!(matches!(list.delete(&9), Err(LexicalError::NotFound(9))));
///
/// // 2 is not the first task, so it can't be the only neighbor of a move to the start
/// assert!(matches!(
///     list.move_to(&3, None, Some(&2)),
///     Err(LexicalError::NotAdjacent { previous: None, next: Some(2) })
/// ));
/// # Ok::<(), staquia::Error<u32>>(())
/// ```
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
                write!(
                    f,
                    "At least the previous or next item must be provided to move the item (id: {id:?})."
                )
            }
            Self::SelfNeighbor(id) => {
                write!(f, "The item (id: {id:?}) can't be moved next to itself.")
            }
            Self::NotAdjacent { previous, next } => match (previous, next) {
                (Some(previous), Some(next)) => {
                    write!(
                        f,
                        "The previous item (id: {previous:?}) and next item (id: {next:?}) are not adjacent."
                    )
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
            Self::NotOrdered {
                index,
                previous,
                id,
            } => write!(
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
            Self::InvalidPosition {
                reason: Some(reason),
                ..
            } => Some(reason),
            _ => None,
        }
    }
}
