//! Sorting the list and spacing its positions again.

use core::cmp::Ordering;

use super::spacing::respace;
use super::{Item, LexicalItem, LexicalList};
use crate::{LexicalError, Position};

impl<T: Item> LexicalList<T> {
    /// Sorts the items with this comparison (stable) and gives them new positions spaced from the start.
    ///
    /// Returns the affected items: the ones whose position changed, to save them.
    ///
    /// # Errors
    ///
    /// [`LexicalError::Position`] if the system has no room for every item, the list is not changed.
    pub fn sort_by<F>(
        &mut self,
        mut compare: F,
    ) -> Result<Vec<&LexicalItem<T>>, LexicalError<T::Id>>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut ids: Vec<T::Id> = self.iter().map(|item| item.id().clone()).collect();
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let items = &self.items;
        ids.sort_by(|a, b| match (items.get(a), items.get(b)) {
            (Some(a), Some(b)) => compare(&a.value, &b.value),
            _ => Ordering::Equal,
        });
        // Creates every position before changing anything
        let mut positions: Vec<Option<Position>> = vec![None; ids.len()];
        respace(&self.system, &mut positions)?;
        let mut affected: Vec<T::Id> = Vec::new();
        for (k, id) in ids.iter().enumerate() {
            let previous = k.checked_sub(1).map(|k| ids[k].clone());
            let next = ids.get(k + 1).cloned();
            let (Some(item), Some(position)) = (self.items.get_mut(id), positions[k]) else {
                continue;
            };
            if item.position != position {
                item.reposition(position);
                affected.push(id.clone());
            }
            item.previous = previous;
            item.next = next;
        }
        self.first = ids.first().cloned();
        self.last = ids.last().cloned();
        Ok(affected
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect())
    }

    /// Spaces every position again from the start, keeping the order: the positions become as short as
    /// possible with the most room between them. Use it when an operation fails because a gap is exhausted
    /// (or a position reached the threshold), then try again.
    ///
    /// Returns the affected items: the ones whose position changed, to save them.
    ///
    /// # Errors
    ///
    /// [`LexicalError::Position`] if the system has no room for every item, the list is not changed.
    pub fn rebalance(&mut self) -> Result<Vec<&LexicalItem<T>>, LexicalError<T::Id>> {
        self.sort_by(|_, _| Ordering::Equal)
    }
}
