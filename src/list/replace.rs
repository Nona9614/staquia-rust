//! Replacing the content of the list with stored rows.

use core::cmp::Ordering;
use std::collections::HashSet;

use super::spacing::{fill, respace};
use super::{Item, LexicalItem, LexicalList, Replaced};
use crate::{LexicalError, Position, Rejected};

impl<T: Item> LexicalList<T> {
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
            values
                .iter()
                .map(Item::id)
                .find(|id| !ids.insert(*id))
                .cloned()
        };
        if let Some(id) = duplicate {
            return Err(Rejected {
                error: LexicalError::DuplicateId(id),
                value: values,
            });
        }
        // Forges the positions without writing them into the values, nothing must change on failure
        let mut originals: Vec<Option<Position>> = Vec::with_capacity(values.len());
        for value in &values {
            let stored = value.position();
            let forged = (!stored.is_empty()).then(|| self.system.forge(stored));
            match forged {
                Some(Ok(position)) => originals.push(Some(position)),
                forged if !untrusted => {
                    let error = LexicalError::InvalidPosition {
                        id: value.id().clone(),
                        reason: forged.and_then(Result::err),
                    };
                    return Err(Rejected {
                        error,
                        value: values,
                    });
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
                    let error = LexicalError::NotOrdered {
                        index: i,
                        previous: values[i - 1].id().clone(),
                        id: values[i].id().clone(),
                    };
                    return Err(Rejected {
                        error,
                        value: values,
                    });
                }
            }
            positions = originals.clone();
        } else {
            // Untrusted: sorts by position (stable), the values without position go at the end
            let (mut positioned, unpositioned): (Vec<usize>, Vec<usize>) =
                order.iter().copied().partition(|i| originals[*i].is_some());
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
                return Err(Rejected {
                    error: error.into(),
                    value: values,
                });
            }
        }
        // Everything is valid, replaces the content
        self.clear();
        self.items.reserve(values.len());
        let mut slots: Vec<Option<T>> = values.into_iter().map(Some).collect();
        let mut affected: Vec<T::Id> = Vec::new();
        let mut previous: Option<T::Id> = None;
        for (k, index) in order.iter().enumerate() {
            let (Some(value), Some(position)) = (slots[*index].take(), positions[k]) else {
                continue;
            };
            let id = value.id().clone();
            let mut item = LexicalItem::new(value, position);
            if originals[*index] != Some(position) {
                affected.push(id.clone());
            }
            item.previous = previous.clone();
            match previous
                .as_ref()
                .and_then(|previous| self.items.get_mut(previous))
            {
                Some(before) => before.next = Some(id.clone()),
                None => self.first = Some(id.clone()),
            }
            self.items.insert(id.clone(), item);
            previous = Some(id);
        }
        self.last = previous;
        Ok(affected
            .iter()
            .filter_map(|id| self.items.get(id))
            .collect())
    }
}
