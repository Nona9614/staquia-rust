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

    /// Removes the item and gives its value back. The positions of the other items don't change.
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

    /// Removes every item.
    pub fn clear(&mut self) {
        self.items.clear();
        self.first = None;
        self.last = None;
        self.selected = None;
    }
}
