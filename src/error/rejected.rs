//! A value given back by a list.

use core::fmt;

use super::LexicalError;

/// An operation of a [`LexicalList`](crate::LexicalList) that took ownership of a value failed: the error
/// and the value, given back unchanged.
///
/// [`push`](crate::LexicalList::push), [`unshift`](crate::LexicalList::unshift),
/// [`set`](crate::LexicalList::set) and [`set_untrusted`](crate::LexicalList::set_untrusted) move the values
/// into the list, so on failure they can't simply return an error: the values would be lost. `Rejected`
/// holds both. It converts into [`LexicalError`] and [`Error`](crate::Error) with `?` (dropping the value),
/// and [`into_value`](Self::into_value) gives the value back, for example to retry after
/// [`LexicalList::rebalance`](crate::LexicalList::rebalance).
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
///
/// // The id 1 is already in the list: the task comes back
/// let rejected = list.push(task(1)).unwrap_err();
/// assert_eq!(rejected.error, LexicalError::DuplicateId(1));
/// assert_eq!(rejected.value.id, 1);
/// # Ok::<(), staquia::Error<u32>>(())
/// ```
#[derive(Clone, PartialEq)]
pub struct Rejected<K, V> {
    /// Why the operation failed.
    pub error: LexicalError<K>,
    /// The value that was not added.
    pub value: V,
}

impl<K, V> Rejected<K, V> {
    /// Gives back the value that was not added, dropping the error.
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
    /// let mut list = LexicalList::new(Staquia::create('!', 'a', 'z', 5, 15, 0.15)?);
    /// list.push(task(1))?;
    ///
    /// let mut duplicate = list.push(task(1)).unwrap_err().into_value();
    /// duplicate.id = 2; // fixes the id and tries again
    /// list.push(duplicate)?;
    /// assert_eq!(list.len(), 2);
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
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
        f.debug_struct("Rejected")
            .field("error", &self.error)
            .finish_non_exhaustive()
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
