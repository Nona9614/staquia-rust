//! A list that keeps the order of your values and their positions in sync.

mod access;
mod insert;
mod item;
mod iter;
mod relocate;
mod remove;
mod replace;
mod sort;
mod spacing;

pub use item::Item;
pub use iter::Iter;

use std::collections::HashMap;

use crate::{Position, Rejected, Staquia};

/// A value of a [`LexicalList`] with its position and the ids of its neighbors.
///
/// The list returns it by reference from its operations (`push`, `move_to`, `find`, `iter`...): read the
/// value to save from [`value`](Self::value), and walk the list with [`previous`](Self::previous) and
/// [`next`](Self::next).
///
/// ```
/// # use staquia::{Item, LexicalList, Position, Staquia};
/// # #[derive(Debug)]
/// # struct Task { id: u32, done: bool, position: String }
/// # impl Item for Task {
/// #     type Id = u32;
/// #     fn id(&self) -> &u32 { &self.id }
/// #     fn position(&self) -> &str { &self.position }
/// #     fn set_position(&mut self, position: &Position) { self.position = position.to_string(); }
/// # }
/// # let task = |id| Task { id, done: false, position: String::new() };
/// # let stored = |id, position: &str| Task { id, done: false, position: position.to_string() };
/// let mut list = LexicalList::new(Staquia::create('!', 'a', 'z', 5, 15, 0.15)?);
/// list.push(task(1))?;
/// let item = list.push(task(2))?;
///
/// assert_eq!(item.id(), &2);
/// assert_eq!(item.position().to_string(), "mmmmn!");
/// assert_eq!(item.previous(), Some(&1));
/// assert_eq!(item.next(), None);
/// # Ok::<(), staquia::Error<u32>>(())
/// ```
pub struct LexicalItem<T: Item> {
    value: T,
    position: Position,
    previous: Option<T::Id>,
    next: Option<T::Id>,
}

/// The result of the operations that change many positions of a [`LexicalList`] at once
/// ([`set`](LexicalList::set), [`set_untrusted`](LexicalList::set_untrusted)): the affected items (the rows
/// to save), or the error with the values given back in a [`Rejected`].
///
/// ```
/// # use staquia::{Item, LexicalList, Position, Staquia};
/// # #[derive(Debug)]
/// # struct Task { id: u32, done: bool, position: String }
/// # impl Item for Task {
/// #     type Id = u32;
/// #     fn id(&self) -> &u32 { &self.id }
/// #     fn position(&self) -> &str { &self.position }
/// #     fn set_position(&mut self, position: &Position) { self.position = position.to_string(); }
/// # }
/// # let task = |id| Task { id, done: false, position: String::new() };
/// # let stored = |id, position: &str| Task { id, done: false, position: position.to_string() };
/// let mut list = LexicalList::new(Staquia::create('!', 'a', 'z', 5, 15, 0.15)?);
/// let rows = vec![stored(1, "mmmmm!"), stored(2, "")];
///
/// let affected = list.set_untrusted(rows)?;
/// assert_eq!(affected.len(), 1); // only task 2 got a new position
/// # Ok::<(), staquia::Error<u32>>(())
/// ```
pub type Replaced<'a, T> = Result<Vec<&'a LexicalItem<T>>, Rejected<<T as Item>::Id, Vec<T>>>;

/// A list that keeps the order of your values and their positions in sync, as a doubly linked list of ids.
///
/// Any value that implements [`Item`] can be stored. Every operation writes the new position into the
/// changed values and tells you which items changed: in the usual case only one, so only one row needs to
/// be saved. Nothing panics: the operations return a [`Result`], and when one fails neither the list nor
/// your values are changed.
///
/// # Example
///
/// ```
/// use staquia::{Item, LexicalList, Position, Staquia};
///
/// struct Task {
///     id: u32,
///     position: String,
/// }
///
/// impl Item for Task {
///     type Id = u32;
///     fn id(&self) -> &u32 { &self.id }
///     fn position(&self) -> &str { &self.position }
///     fn set_position(&mut self, position: &Position) { self.position = position.to_string(); }
/// }
///
/// let task = |id| Task { id, position: String::new() };
/// let mut list = LexicalList::new(Staquia::create('!', 'a', 'z', 5, 15, 0.15)?);
///
/// list.push(task(1))?; // "mmmmm!"
/// list.push(task(2))?; // "mmmmn!"
/// list.push(task(3))?; // "mmmmo!"
///
/// // Moves the task 3 between 1 and 2: only its position changes
/// let moved = list.move_to(&3, Some(&1), Some(&2))?;
/// assert_eq!(moved.value().position, "mmmmmn");
///
/// let order: Vec<u32> = list.iter().map(|item| item.value().id).collect();
/// assert_eq!(order, [1, 3, 2]);
/// # Ok::<(), staquia::Error<u32>>(())
/// ```
pub struct LexicalList<T: Item> {
    system: Staquia,
    items: HashMap<T::Id, LexicalItem<T>>,
    first: Option<T::Id>,
    last: Option<T::Id>,
    selected: Option<T::Id>,
}
