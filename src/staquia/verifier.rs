//! The checks of a position against a system (JavaScript `IOverflowVerifier`).

use super::Staquia;
use crate::Position;

impl Staquia {
    /// Whether the position is longer than the [maximum length](Self::max_length) of this system.
    ///
    /// A system never creates such a position, but one created by a system with a longer `Z` section (and the
    /// same characters and `ln`) can be: use it before mixing positions of both systems.
    ///
    /// ```
    /// use staquia::{Options, Staquia};
    ///
    /// let short = Staquia::create('!', 'a', 'z', 5, 5, 0.0)?;
    /// let long = Staquia::create_with_options(Options { lz: 15, ..short.options() })?;
    ///
    /// let position = long.forge("mmmmmabcdefg")?; // 12 characters
    /// assert!(short.has_overflow(&position)); // more than 5 + 5
    /// assert!(!long.has_overflow(&position));
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn has_overflow(&self, position: &Position) -> bool {
        position.len() > self.lmax
    }

    /// Whether the position reached the [threshold length](Self::threshold_length) of this system.
    ///
    /// ```
    /// use staquia::Staquia;
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// assert!(!system.is_threshold_reached(&system.start()?));
    ///
    /// // A position at the threshold is still created, inside the error
    /// let error = system.forge(&"a".repeat(17)).unwrap_err();
    /// let position = error.position().copied().unwrap();
    /// assert!(system.is_threshold_reached(&position));
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn is_threshold_reached(&self, position: &Position) -> bool {
        position.len() >= self.lthreshold
    }

    /// Whether the position was created by this system, or by one with the same characters and `ln` (its
    /// signature). Positions of another system can't be given to this one: they are a
    /// [`PositionError::SystemMismatch`](crate::PositionError::SystemMismatch).
    ///
    /// ```
    /// use staquia::{PositionError, Staquia};
    ///
    /// let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    /// let other = Staquia::default();
    ///
    /// let position = other.start()?;
    /// assert!(other.owns(&position));
    /// assert!(!system.owns(&position));
    /// assert!(matches!(system.next(&position), Err(PositionError::SystemMismatch { .. })));
    /// # Ok::<(), staquia::Error>(())
    /// ```
    pub fn owns(&self, position: &Position) -> bool {
        *position.signature() == self.signature
    }
}
