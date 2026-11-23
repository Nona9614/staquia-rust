//! The values of a list.

use core::fmt;
use core::hash::Hash;

use super::LexicalItem;
use crate::Position;

/// A value that can be stored in a [`LexicalList`](crate::LexicalList): it has an id and a position.
///
/// The list owns the values and writes their new position back with [`set_position`](Self::set_position)
/// every time it changes, so the values always hold the position to save. Store it as you prefer: the
/// [`Position`] itself, or its string (`position.to_string()`).
///
/// # Example
///
/// ```
/// use staquia::{Item, Position};
///
/// struct Task {
///     id: u32,
///     title: String,
///     position: String,
/// }
///
/// impl Item for Task {
///     type Id = u32;
///
///     fn id(&self) -> &u32 {
///         &self.id
///     }
///
///     fn position(&self) -> &str {
///         &self.position
///     }
///
///     fn set_position(&mut self, position: &Position) {
///         self.position = position.to_string();
///     }
/// }
/// ```
pub trait Item {
    /// The type of the ids. Each value of a list must have a different id.
    type Id: Eq + Hash + Clone;

    /// The id of the value. It must not change while the value is in a list.
    fn id(&self) -> &Self::Id;

    /// The stored position string, read only by [`LexicalList::set`](crate::LexicalList::set) and [`LexicalList::set_untrusted`](crate::LexicalList::set_untrusted)
    /// (an empty string is a missing position).
    fn position(&self) -> &str;

    /// Receives the new position of the value.
    fn set_position(&mut self, position: &Position);
}

impl<T: Item> LexicalItem<T> {
    /// Wraps the value and writes the position into it.
    pub(super) fn new(mut value: T, position: Position) -> Self {
        value.set_position(&position);
        Self {
            value,
            position,
            previous: None,
            next: None,
        }
    }

    /// Changes the position, also in the value.
    pub(super) fn reposition(&mut self, position: Position) {
        self.value.set_position(&position);
        self.position = position;
    }

    /// The id of the value.
    pub fn id(&self) -> &T::Id {
        self.value.id()
    }

    /// The value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// The position of the value.
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// The id of the item before this one, `None` for the first item.
    pub fn previous(&self) -> Option<&T::Id> {
        self.previous.as_ref()
    }

    /// The id of the item after this one, `None` for the last item.
    pub fn next(&self) -> Option<&T::Id> {
        self.next.as_ref()
    }
}

impl<T: Item + fmt::Debug> fmt::Debug for LexicalItem<T>
where
    T::Id: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LexicalItem")
            .field("value", &self.value)
            .field("position", &self.position)
            .field("previous", &self.previous)
            .field("next", &self.next)
            .finish()
    }
}

impl<T: Item + Clone> Clone for LexicalItem<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            position: self.position,
            previous: self.previous.clone(),
            next: self.next.clone(),
        }
    }
}
