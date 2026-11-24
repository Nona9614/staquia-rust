//! Walking the list in its order.

use core::iter::FusedIterator;

use super::{Item, LexicalItem, LexicalList};

impl<T: Item> LexicalList<T> {
    /// The items in the order of the list, from the first to the last. `&list` can also be used in a `for`
    /// loop.
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
    /// list.push(task(2))?;
    /// list.unshift(task(3))?;
    ///
    /// let ids: Vec<u32> = list.iter().map(|item| item.value().id).collect();
    /// assert_eq!(ids, [3, 1, 2]);
    ///
    /// for item in &list {
    ///     println!("{} {}", item.id(), item.position());
    /// }
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            current: self.first.as_ref(),
            remaining: self.items.len(),
        }
    }
}

/// The items of a [`LexicalList`] in their order, see [`LexicalList::iter`].
///
/// It knows how many items are left ([`ExactSizeIterator`]).
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
/// list.push(task(2))?;
///
/// let mut iter = list.iter();
/// assert_eq!(iter.len(), 2);
/// assert_eq!(iter.next().unwrap().id(), &1);
/// # Ok::<(), staquia::Error<u32>>(())
/// ```
pub struct Iter<'a, T: Item> {
    list: &'a LexicalList<T>,
    current: Option<&'a T::Id>,
    remaining: usize,
}

impl<'a, T: Item> Iterator for Iter<'a, T> {
    type Item = &'a LexicalItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let item = self.list.items.get(self.current?)?;
        self.current = item.next.as_ref();
        self.remaining -= 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: Item> ExactSizeIterator for Iter<'_, T> {}

impl<T: Item> FusedIterator for Iter<'_, T> {}

impl<'a, T: Item> IntoIterator for &'a LexicalList<T> {
    type Item = &'a LexicalItem<T>;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
