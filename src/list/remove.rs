//! Removing values from the list.

use super::{Item, LexicalList};
use crate::LexicalError;

impl<T: Item> LexicalList<T> {
    /// Links the neighbors of an item that leaves its place.
    pub(super) fn unlink(&mut self, previous: Option<&T::Id>, next: Option<&T::Id>) {
        match previous.and_then(|id| self.items.get_mut(id)) {
            Some(item) => item.next = next.cloned(),
            None => self.first = next.cloned(),
        }
        match next.and_then(|id| self.items.get_mut(id)) {
            Some(item) => item.previous = previous.cloned(),
            None => self.last = previous.cloned(),
        }
    }

    /// Removes the item and gives its value back. The positions of the other items don't change, so only the
    /// deleted row needs to be removed from the database.
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
    /// list.push(task(3))?;
    ///
    /// let task = list.delete(&2)?;
    /// assert_eq!(task.id, 2);
    /// assert_eq!(list.find(&1).unwrap().next(), Some(&3));
    /// assert_eq!(list.find(&3).unwrap().position().to_string(), "mmmmo!"); // unchanged
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    ///
    ///
    /// # Errors
    ///
    /// [`LexicalError::NotFound`] if there is no item with this id.
    pub fn delete(&mut self, id: &T::Id) -> Result<T, LexicalError<T::Id>> {
        let item = self
            .items
            .remove(id)
            .ok_or_else(|| LexicalError::NotFound(id.clone()))?;
        self.unlink(item.previous.as_ref(), item.next.as_ref());
        if self.selected.as_ref() == Some(id) {
            self.selected = None;
        }
        Ok(item.value)
    }

    /// Removes every item and the selection. The system stays the same.
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
    /// list.clear();
    /// assert!(list.is_empty());
    /// assert_eq!(list.tail()?.to_string(), "mmmmm!"); // starts again
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn clear(&mut self) {
        self.items.clear();
        self.first = None;
        self.last = None;
        self.selected = None;
    }
}
