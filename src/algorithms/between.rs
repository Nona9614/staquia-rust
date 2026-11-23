//! The middle of two integers.

use super::HalfMode;
use crate::number::Digits;

/// The middle of two integers.
pub(crate) fn between(base: i32, before: &Digits, after: &Digits, round: bool) -> Digits {
    let mut before = before.duplicate();
    let mut after = after.duplicate();
    let mut repeated = Digits::new();
    super::compact_numbers(&mut before, &mut after, &mut repeated, HalfMode::Between);
    let mut value = super::half(base, &before, &after, round);
    value.stack(&repeated);
    value
}
