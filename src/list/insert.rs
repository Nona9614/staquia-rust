//! Adding values at the ends of the list.

use super::{Item, LexicalItem, LexicalList};
use crate::{LexicalError, Rejected};

impl<T: Item> LexicalList<T> {
    /// Adds the value at the end of the list and returns its item.
    ///
    /// Its position is the [next](crate::Staquia::next) one of the last item (the start position in an empty
    /// list), written into the value with [`Item::set_position`]: save that one row. The stored position of
    /// the value is ignored.
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
    ///
    /// let first = list.push(task(1))?;
    /// assert_eq!(first.value().position, "mmmmm!"); // the value has its position, save it
    ///
    /// list.push(task(2))?;
    /// assert_eq!(list.last().unwrap().value().position, "mmmmn!");
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// The value is given back in a [`Rejected`] with [`LexicalError::DuplicateId`] if its id is already in
    /// the list, or with [`LexicalError::Position`] if the position could not be created (try again after
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

    /// Adds the value at the start of the list and returns its item.
    ///
    /// Its position is the [previous](crate::Staquia::previous) one of the first item (the start position in
    /// an empty list), written into the value with [`Item::set_position`].
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
    /// list.push(task(1))?; // "mmmmm!"
    ///
    /// let item = list.unshift(task(2))?;
    /// assert_eq!(item.value().position, "mmmml!");
    /// assert_eq!(list.first().unwrap().id(), &2);
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
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
