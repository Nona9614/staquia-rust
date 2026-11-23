//! A value given back by a list.

use core::fmt;

use super::LexicalError;

/// An operation of a [`LexicalList`](crate::LexicalList) that took ownership of a value failed: the error
/// and the value, given back unchanged.
///
/// It converts into [`LexicalError`] with `?`, and [`into_value`](Self::into_value) gives the value back,
/// for example to retry after [`LexicalList::rebalance`](crate::LexicalList::rebalance).
#[derive(Clone, PartialEq)]
pub struct Rejected<K, V> {
    /// Why the operation failed.
    pub error: LexicalError<K>,
    /// The value that was not added.
    pub value: V,
}

impl<K, V> Rejected<K, V> {
    /// The value that was not added.
    pub fn into_value(self) -> V {
        self.value
    }
}

impl<K, V> From<Rejected<K, V>> for LexicalError<K> {
    fn from(rejected: Rejected<K, V>) -> Self {
        rejected.error
    }
}

impl<K: fmt::Debug, V> fmt::Display for Rejected<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl<K: fmt::Debug, V> fmt::Debug for Rejected<K, V> {
    /// Shows the error only, so the values don't need to implement `Debug`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rejected")
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

impl<K: fmt::Debug, V> std::error::Error for Rejected<K, V> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.error {
            LexicalError::Position(error) => Some(error),
            _ => None,
        }
    }
}
