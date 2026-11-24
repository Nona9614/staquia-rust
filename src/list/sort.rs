//! Sorting the list and spacing its positions again.

use core::cmp::Ordering;

use super::spacing::respace;
use super::{Item, LexicalItem, LexicalList};
use crate::{LexicalError, Position};

impl<T: Item> LexicalList<T> {
    /// Sorts the items with this comparison (stable) and gives them new positions spaced from the start.
    ///
    /// Unlike moving items one by one, sorting changes many positions: save every affected item. Every position
    /// is created before anything changes, so a failure leaves the list as it was.
    ///
    /// Returns the affected items: the ones whose position changed, to save them.
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
    /// list.push(task(2))?; // "mmmmn!"
    /// list.push(task(3))?; // "mmmmo!"
    ///
    /// // Highest id first
    /// let affected: Vec<u32> = list.sort_by(|a, b| b.id.cmp(&a.id))?.iter().map(|item| item.value().id).collect();
    /// assert_eq!(affected, [3, 1]); // 2 kept "mmmmn!"
    ///
    /// let ids: Vec<u32> = list.iter().map(|item| item.value().id).collect();
    /// assert_eq!(ids, [3, 2, 1]);
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
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
    /// list.push(task(2))?; // "mmmmn!"
    /// list.push(task(3))?;
    /// list.move_to(&3, Some(&1), Some(&2))?; // "mmmmmn", one digit longer than the others
    ///
    /// let affected: Vec<u32> = list.rebalance()?.iter().map(|item| item.value().id).collect();
    /// assert_eq!(affected, [3, 2]);
    /// assert_eq!(list.find(&3).unwrap().position().to_string(), "mmmmn!"); // short again, same order
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    ///
    /// # Errors
    ///
    /// [`LexicalError::Position`] if the system has no room for every item, the list is not changed.
    pub fn rebalance(&mut self) -> Result<Vec<&LexicalItem<T>>, LexicalError<T::Id>> {
        self.sort_by(|_, _| Ordering::Equal)
    }
}
