//! Walking the list in its order.

use core::iter::FusedIterator;

use super::{Item, LexicalItem, LexicalList};

impl<T: Item> LexicalList<T> {
    /// The items in their order.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            current: self.first.as_ref(),
            remaining: self.items.len(),
        }
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
