//! Reading the list and its selection.

use core::fmt;
use std::collections::HashMap;

use super::{Item, LexicalItem, LexicalList};
use crate::{Position, PositionError, Staquia};

impl<T: Item> LexicalList<T> {
    /// Creates an empty list that creates the positions with this system.
    ///
    /// Every list needs its own copy of the system (it is a small `Copy` value). Use the same configuration for
    /// every list whose positions are stored together.
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
    /// assert!(list.is_empty());
    ///
    /// list.push(task(1))?;
    /// assert_eq!(list.first().unwrap().position().to_string(), "mmmmm!");
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn new(system: Staquia) -> Self {
        Self {
            system,
            items: HashMap::new(),
            first: None,
            last: None,
            selected: None,
        }
    }

    /// The system that creates the positions, for example to [`forge`](Staquia::forge) a stored string before
    /// comparing it with the positions of the list.
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
    /// let list: LexicalList<Task> = LexicalList::new(Staquia::create('!', 'a', 'z', 5, 15, 0.15)?);
    /// assert_eq!(list.system().base(), 27);
    ///
    /// // Checks a stored string against the positions of the list
    /// let stored = list.system().forge("mmmmn")?;
    /// assert_eq!(stored.to_string(), "mmmmn!");
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn system(&self) -> &Staquia {
        &self.system
    }

    /// The number of items.
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
    /// assert_eq!(list.len(), 2);
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the list has no items.
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
    /// assert!(list.is_empty());
    /// list.push(task(1))?;
    /// assert!(!list.is_empty());
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The first item, the one with the smallest position. `None` when the list is empty.
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
    /// assert!(list.first().is_none());
    ///
    /// list.push(task(1))?;
    /// list.unshift(task(2))?;
    /// assert_eq!(list.first().unwrap().id(), &2);
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn first(&self) -> Option<&LexicalItem<T>> {
        self.first.as_ref().and_then(|id| self.items.get(id))
    }

    /// The last item, the one with the largest position. `None` when the list is empty.
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
    /// assert_eq!(list.last().unwrap().id(), &2);
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn last(&self) -> Option<&LexicalItem<T>> {
        self.last.as_ref().and_then(|id| self.items.get(id))
    }

    /// The item with this id, with its position and neighbors. `None` if there is no such item.
    ///
    /// Finding by id is a hash map lookup, it doesn't walk the list.
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
    /// let item = list.find(&2).unwrap();
    /// assert_eq!(item.position().to_string(), "mmmmn!");
    /// assert_eq!(item.previous(), Some(&1));
    /// assert!(list.find(&9).is_none());
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn find(&self, id: &T::Id) -> Option<&LexicalItem<T>> {
        self.items.get(id)
    }

    /// The value with this id, to change it in place (for example its title or state).
    ///
    /// Don't change its id or its position: the list would not know about it. Use
    /// [`move_to`](Self::move_to) to change the order.
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
    ///
    /// if let Some(task) = list.find_mut(&1) {
    ///     task.done = true;
    /// }
    /// assert!(list.find(&1).unwrap().value().done);
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn find_mut(&mut self, id: &T::Id) -> Option<&mut T> {
        self.items.get_mut(id).map(|item| &mut item.value)
    }

    /// Whether there is an item with this id.
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
    /// assert!(list.contains(&1));
    /// assert!(!list.contains(&2));
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn contains(&self, id: &T::Id) -> bool {
        self.items.contains_key(id)
    }

    /// The item at this index in the order of the list, `None` past the end.
    ///
    /// It walks the list from the first item, so it takes `O(index)` steps; use [`find`](Self::find) when the
    /// id is known.
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
    /// list.unshift(task(3))?; // 3, 1, 2
    ///
    /// assert_eq!(list.at(0).unwrap().id(), &3);
    /// assert_eq!(list.at(2).unwrap().id(), &2);
    /// assert!(list.at(3).is_none());
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn at(&self, index: usize) -> Option<&LexicalItem<T>> {
        self.iter().nth(index)
    }

    /// The selected item, see [`select`](Self::select). `None` when nothing is selected.
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
    /// assert!(list.selected().is_none());
    ///
    /// list.select(&1);
    /// assert_eq!(list.selected().unwrap().id(), &1);
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn selected(&self) -> Option<&LexicalItem<T>> {
        self.selected.as_ref().and_then(|id| self.items.get(id))
    }

    /// Selects the item with this id and returns it, for example the item being dragged in a user interface.
    /// With an id that is not in the list, the selection is cleared and `None` is returned.
    ///
    /// The selection is cleared when the item is deleted, or when the list is [cleared](Self::clear) or
    /// replaced.
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
    /// assert!(list.select(&2).is_some());
    /// list.delete(&2)?;
    /// assert!(list.selected().is_none());
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn select(&mut self, id: &T::Id) -> Option<&LexicalItem<T>> {
        self.selected = self.items.contains_key(id).then(|| id.clone());
        self.selected()
    }

    /// Clears the selection.
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
    /// list.select(&1);
    ///
    /// list.deselect();
    /// assert!(list.selected().is_none());
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
    pub fn deselect(&mut self) {
        self.selected = None;
    }

    /// The position a new item would get at the start of the list (the start position when it is empty),
    /// without changing the list. [`unshift`](Self::unshift) uses it.
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
    /// assert_eq!(list.head()?.to_string(), "mmmmm!");
    ///
    /// list.push(task(1))?;
    /// assert_eq!(list.head()?.to_string(), "mmmml!");
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
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

    /// The position a new item would get at the end of the list (the start position when it is empty),
    /// without changing the list. [`push`](Self::push) uses it.
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
    /// assert_eq!(list.tail()?.to_string(), "mmmmm!");
    ///
    /// list.push(task(1))?;
    /// assert_eq!(list.tail()?.to_string(), "mmmmn!");
    /// # Ok::<(), staquia::Error<u32>>(())
    /// ```
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
