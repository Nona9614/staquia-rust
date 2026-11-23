//! Moving an item between two others.

use super::{Item, LexicalItem, LexicalList};
use crate::LexicalError;

impl<T: Item> LexicalList<T> {
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
            let item = self
                .items
                .get(id)
                .ok_or_else(|| LexicalError::NotFound(id.clone()))?;
            let before = match previous {
                Some(previous) => Some(
                    self.items
                        .get(previous)
                        .ok_or_else(|| LexicalError::NotFound(previous.clone()))?,
                ),
                None => None,
            };
            let after = match next {
                Some(next) => Some(
                    self.items
                        .get(next)
                        .ok_or_else(|| LexicalError::NotFound(next.clone()))?,
                ),
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
            let not_adjacent = || LexicalError::NotAdjacent {
                previous: previous.cloned(),
                next: next.cloned(),
            };
            if let Some(before) = before {
                let following = if before.next.as_ref() == Some(id) {
                    item.next.as_ref()
                } else {
                    before.next.as_ref()
                };
                if following != next {
                    return Err(not_adjacent());
                }
            } else if let Some(after) = after {
                let preceding = if after.previous.as_ref() == Some(id) {
                    item.previous.as_ref()
                } else {
                    after.previous.as_ref()
                };
                if preceding.is_some() {
                    return Err(not_adjacent());
                }
            }
            // The item is already in the requested place, nothing to do
            if item.previous.as_ref() == previous && item.next.as_ref() == next {
                (None, None, None)
            } else {
                let position = match (before, after) {
                    (Some(before), Some(after)) => {
                        self.system.core(&before.position, &after.position)
                    }
                    (Some(before), None) => self.system.next(&before.position),
                    (None, Some(after)) => self.system.previous(&after.position),
                    (None, None) => return Err(LexicalError::MissingNeighbors(id.clone())),
                }?;
                (Some(position), item.previous.clone(), item.next.clone())
            }
        };
        let Some(position) = position else {
            return self
                .items
                .get(id)
                .ok_or_else(|| LexicalError::NotFound(id.clone()));
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
        let item = self
            .items
            .get_mut(id)
            .ok_or_else(|| LexicalError::NotFound(id.clone()))?;
        item.previous = previous.cloned();
        item.next = next.cloned();
        item.reposition(position);
        Ok(item)
    }
}
