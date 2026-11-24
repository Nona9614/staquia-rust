//! Reading the list and its selection.

use core::fmt;
use std::collections::HashMap;

use super::{Item, LexicalItem, LexicalList};
use crate::{Position, PositionError, Staquia};

impl<T: Item> LexicalList<T> {
    /// Creates an empty list that creates the positions with this system.
    pub fn new(system: Staquia) -> Self {
        Self {
            system,
            items: HashMap::new(),
            first: None,
            last: None,
            selected: None,
        }
    }

    /// The system that creates the positions.
    pub fn system(&self) -> &Staquia {
        &self.system
    }

    /// The number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the list has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The first item.
    pub fn first(&self) -> Option<&LexicalItem<T>> {
        self.first.as_ref().and_then(|id| self.items.get(id))
    }

    /// The last item.
    pub fn last(&self) -> Option<&LexicalItem<T>> {
        self.last.as_ref().and_then(|id| self.items.get(id))
    }

    /// The item with this id.
    pub fn find(&self, id: &T::Id) -> Option<&LexicalItem<T>> {
        self.items.get(id)
    }

    /// The value with this id, to change it in place.
    ///
    /// Don't change its id or its position: the list would not know about it.
    pub fn find_mut(&mut self, id: &T::Id) -> Option<&mut T> {
        self.items.get_mut(id).map(|item| &mut item.value)
    }

    /// Whether there is an item with this id.
    pub fn contains(&self, id: &T::Id) -> bool {
        self.items.contains_key(id)
    }

    /// The item at this index, walking from the first one.
    pub fn at(&self, index: usize) -> Option<&LexicalItem<T>> {
        self.iter().nth(index)
    }

    /// The selected item, see [`select`](Self::select).
    pub fn selected(&self) -> Option<&LexicalItem<T>> {
        self.selected.as_ref().and_then(|id| self.items.get(id))
    }

    /// Selects the item with this id (or clears the selection if there is none), for example the item being
    /// dragged. The selection is cleared when the item is deleted or the list replaced.
    pub fn select(&mut self, id: &T::Id) -> Option<&LexicalItem<T>> {
        self.selected = self.items.contains_key(id).then(|| id.clone());
        self.selected()
    }

    /// Clears the selection.
    pub fn deselect(&mut self) {
        self.selected = None;
    }

    /// The position for a new item at the start of the list (the start position when it is empty).
    ///
    /// # Errors
    ///
    /// The errors of [`Staquia::previous`] (or [`Staquia::start`] when the list is empty), for example
    /// [`PositionError::EdgeReached`] when the first item already has the first value of `N`.
    pub fn head(&self) -> Result<Position, PositionError> {
        match self.first() {
            Some(first) => self.system.previous(&first.position),
            None => self.system.start(),
        }
    }

    /// The position for a new item at the end of the list (the start position when it is empty).
    ///
    /// # Errors
    ///
    /// The errors of [`Staquia::next`] (or [`Staquia::start`] when the list is empty), for example
    /// [`PositionError::EdgeReached`] when the last item already has the last value of `N`.
    pub fn tail(&self) -> Result<Position, PositionError> {
        match self.last() {
            Some(last) => self.system.next(&last.position),
            None => self.system.start(),
        }
    }
}

impl<T: Item + fmt::Debug> fmt::Debug for LexicalList<T>
where
    T::Id: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
