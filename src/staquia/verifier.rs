//! The checks of a position against a system (JavaScript `IOverflowVerifier`).

use super::Staquia;
use crate::Position;

impl Staquia {
    /// Whether the position is longer than the maximum length of this system.
    pub fn has_overflow(&self, position: &Position) -> bool {
        position.len() > self.lmax
    }

    /// Whether the position reached the threshold length of this system.
    pub fn is_threshold_reached(&self, position: &Position) -> bool {
        position.len() >= self.lthreshold
    }

    /// Whether the position was created by this system (or one with the same configuration).
    pub fn owns(&self, position: &Position) -> bool {
        *position.signature() == self.signature
    }
}
