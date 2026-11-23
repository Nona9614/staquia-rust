//! A list that keeps the order of your values and their positions in sync.

use core::cmp::Ordering;
use core::fmt;
use core::hash::Hash;
use core::iter::FusedIterator;
use std::collections::{HashMap, HashSet};

use crate::{LexicalError, Position, PositionError, Rejected, Staquia};

/// A value that can be stored in a [`LexicalList`]: it has an id and a position.
///
/// The list owns the values and writes their new position back with [`set_position`](Self::set_position)
/// every time it changes, so the values always hold the position to save. Store it as you prefer: the
/// [`Position`] itself, or its string (`position.to_string()`).
///
/// # Example
///
/// ```
/// use staquia::{Item, Position};
///
/// struct Task {
///     id: u32,
///     title: String,
///     position: String,
/// }
///
/// impl Item for Task {
///     type Id = u32;
///
///     fn id(&self) -> &u32 {
///         &self.id
///     }
///
///     fn position(&self) -> &str {
///         &self.position
///     }
///
///     fn set_position(&mut self, position: &Position) {
///         self.position = position.to_string();
///     }
/// }
/// ```
pub trait Item {
    /// The type of the ids. Each value of a list must have a different id.
    type Id: Eq + Hash + Clone;

    /// The id of the value. It must not change while the value is in a list.
    fn id(&self) -> &Self::Id;

    /// The stored position string, read only by [`LexicalList::set`] and [`LexicalList::set_untrusted`]
    /// (an empty string is a missing position).
    fn position(&self) -> &str;

    /// Receives the new position of the value.
    fn set_position(&mut self, position: &Position);
}

/// A value of a [`LexicalList`] with its position and the ids of its neighbors.
pub struct LexicalItem<T: Item> {
    value: T,
    position: Position,
    previous: Option<T::Id>,
    next: Option<T::Id>,
}

impl<T: Item> LexicalItem<T> {
    /// Wraps the value and writes the position into it.
    fn new(mut value: T, position: Position) -> Self {
        value.set_position(&position);
        Self { value, position, previous: None, next: None }
    }

    /// Changes the position, also in the value.
    fn reposition(&mut self, position: Position) {
        self.value.set_position(&position);
        self.position = position;
    }

    /// The id of the value.
    pub fn id(&self) -> &T::Id {
        self.value.id()
    }

    /// The value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// The position of the value.
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// The id of the item before this one, `None` for the first item.
    pub fn previous(&self) -> Option<&T::Id> {
        self.previous.as_ref()
    }

    /// The id of the item after this one, `None` for the last item.
    pub fn next(&self) -> Option<&T::Id> {
        self.next.as_ref()
    }
}

impl<T: Item + fmt::Debug> fmt::Debug for LexicalItem<T>
where
    T::Id: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LexicalItem")
            .field("value", &self.value)
            .field("position", &self.position)
            .field("previous", &self.previous)
            .field("next", &self.next)
            .finish()
    }
}

impl<T: Item + Clone> Clone for LexicalItem<T> {
    fn clone(&self) -> Self {
        Self { value: self.value.clone(), position: self.position, previous: self.previous.clone(), next: self.next.clone() }
    }
}

/// The result of the operations that replace the content of a [`LexicalList`]: the affected items, or the
/// error with the values given back.
pub type Replaced<'a, T> = Result<Vec<&'a LexicalItem<T>>, Rejected<<T as Item>::Id, Vec<T>>>;

/// A list that keeps the order of your values and their positions in sync, as a doubly linked list of ids.
///
/// Any value that implements [`Item`] can be stored. Every operation writes the new position into the
/// changed values and tells you which items changed: in the usual case only one, so only one row needs to
/// be saved. Nothing panics: the operations return a [`Result`], and when one fails neither the list nor
/// your values are changed.
///
/// # Example
///
/// ```
/// use staquia::{Item, LexicalList, Position, Staquia};
///
/// struct Task {
///     id: u32,
///     position: String,
/// }
///
/// impl Item for Task {
///     type Id = u32;
///     fn id(&self) -> &u32 { &self.id }
///     fn position(&self) -> &str { &self.position }
///     fn set_position(&mut self, position: &Position) { self.position = position.to_string(); }
/// }
///
/// let task = |id| Task { id, position: String::new() };
/// let mut list = LexicalList::new(Staquia::create('!', 'a', 'z', 5, 15, 0.15)?);
///
/// list.push(task(1))?; // "mmmmm!"
/// list.push(task(2))?; // "mmmmn!"
/// list.push(task(3))?; // "mmmmo!"
///
/// // Moves the task 3 between 1 and 2: only its position changes
/// let moved = list.move_to(&3, Some(&1), Some(&2))?;
/// assert_eq!(moved.value().position, "mmmmmn");
///
/// let order: Vec<u32> = list.iter().map(|item| item.value().id).collect();
/// assert_eq!(order, [1, 3, 2]);
/// # Ok::<(), staquia::Error<u32>>(())
/// ```
pub struct LexicalList<T: Item> {
    system: Staquia,
    items: HashMap<T::Id, LexicalItem<T>>,
    first: Option<T::Id>,
    last: Option<T::Id>,
    selected: Option<T::Id>,
}

impl<T: Item> LexicalList<T> {
    /// Creates an empty list that creates the positions with this system.
    pub fn new(system: Staquia) -> Self {
        Self { system, items: HashMap::new(), first: None, last: None, selected: None }
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

    /// The items in their order.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { list: self, current: self.first.as_ref(), remaining: self.items.len() }
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
    pub fn head(&self) -> Result<Position, PositionError> {
        match self.first() {
            Some(first) => self.system.previous(&first.position),
            None => self.system.start(),
        }
    }

    /// The position for a new item at the end of the list (the start position when it is empty).
    pub fn tail(&self) -> Result<Position, PositionError> {
        match self.last() {
            Some(last) => self.system.next(&last.position),
            None => self.system.start(),
        }
    }

    /// Adds the value at the end of the list, its position is replaced by the next one of the last item.
    ///
    /// # Errors
    ///
    /// The value is given back with [`LexicalError::DuplicateId`] if its id is already in the list, or with
    /// [`LexicalError::Position`] if the position could not be created (try again after
    /// [`rebalance`](Self::rebalance)).
    pub fn push(&mut self, value: T) -> Result<&LexicalItem<T>, Rejected<T::Id, T>> {
        if self.items.contains_key(value.id()) {
            return Err(Rejected { error: LexicalError::DuplicateId(value.id().clone()), value });
        }
        let position = match self.tail() {
            Ok(position) => position,
            Err(error) => return Err(Rejected { error: error.into(), value }),
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
            return Err(Rejected { error: LexicalError::DuplicateId(value.id().clone()), value });
        }
        let position = match self.head() {
            Ok(position) => position,
            Err(error) => return Err(Rejected { error: error.into(), value }),
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

    /// Links the neighbors of an item that leaves its place.
    fn unlink(&mut self, previous: Option<&T::Id>, next: Option<&T::Id>) {
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
        let item = self.items.remove(id).ok_or_else(|| LexicalError::NotFound(id.clone()))?;
        self.unlink(item.previous.as_ref(), item.next.as_ref());
        if self.selected.as_ref() == Some(id) {
            self.selected = None;
        }
        Ok(item.value)
    }

    /// Moves the item between `previous` and `next`, the ids of its new neighbors as they are seen once the
    /// item left its current place. Give only `previous` to move it to the end, only `next` to move it to the
    /// start. Only the moved item gets a new position.
    ///
    /// # Errors
    ///
    /// - [`LexicalError::NotFound`] if an id is not in the list.
    /// - [`LexicalError::MissingNeighbors`] without neighbors, [`LexicalError::SelfNeighbor`] if the item is
    ///   one of them.
    /// - [`LexicalError::NotAdjacent`] if the neighbors are not next to each other, or a single neighbor is
    ///   not at that end.
    /// - [`LexicalError::Position`] if there is no room for a new position: [`rebalance`](Self::rebalance)
    ///   the list and try again.
    pub fn move_to(
        &mut self,
        id: &T::Id,
        previous: Option<&T::Id>,
        next: Option<&T::Id>,
    ) -> Result<&LexicalItem<T>, LexicalError<T::Id>> {
        let (position, old_previous, old_next) = {
            let item = self.items.get(id).ok_or_else(|| LexicalError::NotFound(id.clone()))?;
            let before = match previous {
                Some(previous) => Some(self.items.get(previous).ok_or_else(|| LexicalError::NotFound(previous.clone()))?),
                None => None,
            };
            let after = match next {
                Some(next) => Some(self.items.get(next).ok_or_else(|| LexicalError::NotFound(next.clone()))?),
                None => None,
            };
            if previous.is_none() && next.is_none() {
                return Err(LexicalError::MissingNeighbors(id.clone()));
            }
            if previous == Some(id) || next == Some(id) {
                return Err(LexicalError::SelfNeighbor(id.clone()));
            }
            // Verifies the adjacency as if the item was already unlinked from its current place:
            // - With both neighbors, `previous` must be followed by `next`.
            // - With only `previous`, it must be the last item (moving to the bottom).
            // - With only `next`, it must be the first item (moving to the top).
            let not_adjacent = || LexicalError::NotAdjacent { previous: previous.cloned(), next: next.cloned() };
            if let Some(before) = before {
                let following = if before.next.as_ref() == Some(id) { item.next.as_ref() } else { before.next.as_ref() };
                if following != next {
                    return Err(not_adjacent());
                }
            } else if let Some(after) = after {
                let preceding = if after.previous.as_ref() == Some(id) { item.previous.as_ref() } else { after.previous.as_ref() };
                if preceding.is_some() {
                    return Err(not_adjacent());
                }
            }
            // The item is already in the requested place, nothing to do
            if item.previous.as_ref() == previous && item.next.as_ref() == next {
                (None, None, None)
            } else {
                let position = match (before, after) {
                    (Some(before), Some(after)) => self.system.core(&before.position, &after.position),
                    (Some(before), None) => self.system.next(&before.position),
                    (None, Some(after)) => self.system.previous(&after.position),
                    (None, None) => return Err(LexicalError::MissingNeighbors(id.clone())),
                }?;
                (Some(position), item.previous.clone(), item.next.clone())
            }
        };
        let Some(position) = position else {
            return self.items.get(id).ok_or_else(|| LexicalError::NotFound(id.clone()));
        };
        // Unlinks the item from its current place and wires it into the new one
        self.unlink(old_previous.as_ref(), old_next.as_ref());
        match previous.and_then(|previous| self.items.get_mut(previous)) {
            Some(before) => before.next = Some(id.clone()),
            None => self.first = Some(id.clone()),
        }
        match next.and_then(|next| self.items.get_mut(next)) {
            Some(after) => after.previous = Some(id.clone()),
            None => self.last = Some(id.clone()),
        }
        let item = self.items.get_mut(id).ok_or_else(|| LexicalError::NotFound(id.clone()))?;
        item.previous = previous.cloned();
        item.next = next.cloned();
        item.reposition(position);
        Ok(item)
    }

    /// Replaces the content of the list with these values, already ordered by their stored positions (for
    /// example rows loaded from a database sorted by position). The positions are verified, not changed.
    ///
    /// Returns the affected items, always none for this trusted input.
    ///
    /// # Errors
    ///
    /// The values are given back, and the list is not changed, with [`LexicalError::DuplicateId`],
    /// [`LexicalError::InvalidPosition`] for a missing or invalid position, or [`LexicalError::NotOrdered`].
    /// Use [`set_untrusted`](Self::set_untrusted) to repair them instead.
    pub fn set(&mut self, values: Vec<T>) -> Replaced<'_, T> {
        self.replace(values, false)
    }

    /// Replaces the content of the list with these values, repairing their stored positions: they are sorted
    /// by position, an equal position keeps the first value and the others get a new position right after it,
    /// and the values without a valid position are added at the end. When there is no room to repair them,
    /// every position is spaced again from the start.
    ///
    /// Returns the affected items: the ones whose position changed, to save them.
    ///
    /// # Errors
    ///
    /// The values are given back, and the list is not changed, with [`LexicalError::DuplicateId`] or
    /// [`LexicalError::Position`] when not even spacing every position again works.
    pub fn set_untrusted(&mut self, values: Vec<T>) -> Replaced<'_, T> {
        self.replace(values, true)
    }

    fn replace(&mut self, values: Vec<T>, untrusted: bool) -> Replaced<'_, T> {
        // Validates the ids before touching the current content
        let duplicate = {
            let mut ids = HashSet::with_capacity(values.len());
            values.iter().map(Item::id).find(|id| !ids.insert(*id)).cloned()
        };
        if let Some(id) = duplicate {
            return Err(Rejected { error: LexicalError::DuplicateId(id), value: values });
        }
        // Forges the positions without writing them into the values, nothing must change on failure
        let mut originals: Vec<Option<Position>> = Vec::with_capacity(values.len());
        for value in &values {
            let stored = value.position();
            let forged = (!stored.is_empty()).then(|| self.system.forge(stored));
            match forged {
                Some(Ok(position)) => originals.push(Some(position)),
                forged if !untrusted => {
                    let error = LexicalError::InvalidPosition { id: value.id().clone(), reason: forged.and_then(Result::err) };
                    return Err(Rejected { error, value: values });
                }
                _ => originals.push(None),
            }
        }
        // The order of the values (indexes) and their final positions
        let mut order: Vec<usize> = (0..values.len()).collect();
        let mut positions: Vec<Option<Position>>;
        if !untrusted {
            // Trusted: only verifies that the incoming order is valid
            for i in 1..values.len() {
                if originals[i - 1].cmp(&originals[i]) != Ordering::Less {
                    let error = LexicalError::NotOrdered { index: i, previous: values[i - 1].id().clone(), id: values[i].id().clone() };
                    return Err(Rejected { error, value: values });
                }
            }
            positions = originals.clone();
        } else {
            // Untrusted: sorts by position (stable), the values without position go at the end
            let (mut positioned, unpositioned): (Vec<usize>, Vec<usize>) = order.iter().copied().partition(|i| originals[*i].is_some());
            positioned.sort_by(|a, b| originals[*a].cmp(&originals[*b]));
            order = positioned;
            order.extend(unpositioned);
            positions = order.iter().map(|i| originals[*i]).collect();
            // Equal positions keep the first one, the others will be given a new position after it
            let mut kept: Option<Position> = None;
            for position in positions.iter_mut() {
                match (*position, kept) {
                    (Some(current), Some(previous)) if current == previous => *position = None,
                    (Some(current), _) => kept = Some(current),
                    (None, _) => {}
                }
            }
            // Fills the missing positions, if there is no room spaces every position again
            if !fill(&self.system, &mut positions)
                && let Err(error) = respace(&self.system, &mut positions)
            {
                return Err(Rejected { error: error.into(), value: values });
            }
        }
        // Everything is valid, replaces the content
        self.clear();
        self.items.reserve(values.len());
        let mut slots: Vec<Option<T>> = values.into_iter().map(Some).collect();
        let mut affected: Vec<T::Id> = Vec::new();
        let mut previous: Option<T::Id> = None;
        for (k, index) in order.iter().enumerate() {
            let (Some(value), Some(position)) = (slots[*index].take(), positions[k]) else { continue };
            let id = value.id().clone();
            let mut item = LexicalItem::new(value, position);
            if originals[*index] != Some(position) {
                affected.push(id.clone());
            }
            item.previous = previous.clone();
            match previous.as_ref().and_then(|previous| self.items.get_mut(previous)) {
                Some(before) => before.next = Some(id.clone()),
                None => self.first = Some(id.clone()),
            }
            self.items.insert(id.clone(), item);
            previous = Some(id);
        }
        self.last = previous;
        Ok(affected.iter().filter_map(|id| self.items.get(id)).collect())
    }

    /// Removes every item.
    pub fn clear(&mut self) {
        self.items.clear();
        self.first = None;
        self.last = None;
        self.selected = None;
    }

    /// Sorts the items with this comparison (stable) and gives them new positions spaced from the start.
    ///
    /// Returns the affected items: the ones whose position changed, to save them.
    ///
    /// # Errors
    ///
    /// [`LexicalError::Position`] if the system has no room for every item, the list is not changed.
    pub fn sort_by<F>(&mut self, mut compare: F) -> Result<Vec<&LexicalItem<T>>, LexicalError<T::Id>>
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
            let (Some(item), Some(position)) = (self.items.get_mut(id), positions[k]) else { continue };
            if item.position != position {
                item.reposition(position);
                affected.push(id.clone());
            }
            item.previous = previous;
            item.next = next;
        }
        self.first = ids.first().cloned();
        self.last = ids.last().cloned();
        Ok(affected.iter().filter_map(|id| self.items.get(id)).collect())
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

/// Gives a position to each run of entries without one: between two positions the gap is bisected, at the
/// start and at the end the positions are stacked. Returns `false` if there is no room.
fn fill(system: &Staquia, positions: &mut [Option<Position>]) -> bool {
    fn bisect(system: &Staquia, positions: &mut [Option<Position>], from: usize, to: usize, left: &Position, right: &Position) -> bool {
        if from >= to {
            return true;
        }
        let middle = (from + to) >> 1;
        let Ok(core) = system.core(left, right) else { return false };
        positions[middle] = Some(core);
        bisect(system, positions, from, middle, left, &core) && bisect(system, positions, middle + 1, to, &core, right)
    }

    let mut i = 0;
    while i < positions.len() {
        if positions[i].is_some() {
            i += 1;
            continue;
        }
        // The run of entries without position is [i, j)
        let mut j = i;
        while j < positions.len() && positions[j].is_none() {
            j += 1;
        }
        let left = i.checked_sub(1).and_then(|k| positions[k]);
        let right = positions.get(j).copied().flatten();
        match (left, right) {
            (Some(left), Some(right)) => {
                if !bisect(system, positions, i, j, &left, &right) {
                    return false;
                }
            }
            (None, Some(right)) => {
                let mut reference = right;
                for k in (i..j).rev() {
                    let Ok(previous) = system.previous(&reference) else { return false };
                    positions[k] = Some(previous);
                    reference = previous;
                }
            }
            (left, None) => {
                let mut reference = match left {
                    Some(left) => system.next(&left),
                    None => system.start(),
                };
                let run = &mut positions[i..j];
                let count = run.len();
                for (k, slot) in run.iter_mut().enumerate() {
                    let Ok(position) = reference else { return false };
                    *slot = Some(position);
                    if k + 1 < count {
                        reference = system.next(&position);
                    }
                }
            }
        }
        i = j;
    }
    true
}

/// Gives every entry a new position, stacked from the start position.
fn respace(system: &Staquia, positions: &mut [Option<Position>]) -> Result<(), PositionError> {
    let mut reference = system.start();
    let count = positions.len();
    for (i, slot) in positions.iter_mut().enumerate() {
        let position = reference?;
        *slot = Some(position);
        if i + 1 < count {
            reference = system.next(&position);
        }
    }
    Ok(())
}

impl<T: Item + fmt::Debug> fmt::Debug for LexicalList<T>
where
    T::Id: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// The items of a [`LexicalList`] in their order, see [`LexicalList::iter`].
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
