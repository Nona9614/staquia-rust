//! Adding values at the ends of the list.

use super::{Item, LexicalItem, LexicalList};
use crate::{LexicalError, Rejected};

impl<T: Item> LexicalList<T> {
    /// Adds the value at the end of the list, its position is replaced by the next one of the last item.
    ///
    /// # Errors
    ///
    /// The value is given back with [`LexicalError::DuplicateId`] if its id is already in the list, or with
    /// [`LexicalError::Position`] if the position could not be created (try again after
    /// [`rebalance`](Self::rebalance)).
    pub fn push(&mut self, value: T) -> Result<&LexicalItem<T>, Rejected<T::Id, T>> {
        if self.items.contains_key(value.id()) {
            return Err(Rejected {
                error: LexicalError::DuplicateId(value.id().clone()),
                value,
            });
        }
        let position = match self.tail() {
            Ok(position) => position,
            Err(error) => {
                return Err(Rejected {
                    error: error.into(),
                    value,
                });
            }
        };
        let id = value.id().clone();
        let mut item = LexicalItem::new(value, position);
        match self.last.replace(id.clone()) {
            Some(last) => {
                if let Some(last) = self.items.get_mut(&last) {
                    last.next = Some(id.clone());
                }
                item.previous = Some(last);
            }
            None => self.first = Some(id.clone()),
        }
        Ok(self.items.entry(id).or_insert(item))
    }

    /// Adds the value at the start of the list, its position is replaced by the previous one of the first item.
    ///
    /// # Errors
    ///
    /// The same as [`push`](Self::push).
    pub fn unshift(&mut self, value: T) -> Result<&LexicalItem<T>, Rejected<T::Id, T>> {
        if self.items.contains_key(value.id()) {
            return Err(Rejected {
                error: LexicalError::DuplicateId(value.id().clone()),
                value,
            });
        }
        let position = match self.head() {
            Ok(position) => position,
            Err(error) => {
                return Err(Rejected {
                    error: error.into(),
                    value,
                });
            }
        };
        let id = value.id().clone();
        let mut item = LexicalItem::new(value, position);
        match self.first.replace(id.clone()) {
            Some(first) => {
                if let Some(first) = self.items.get_mut(&first) {
                    first.previous = Some(id.clone());
                }
                item.next = Some(first);
            }
            None => self.last = Some(id.clone()),
        }
        Ok(self.items.entry(id).or_insert(item))
    }
}
