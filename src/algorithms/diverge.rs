//! The middle of two fractions.

use super::HalfMode;
use crate::number::Digits;

/// The middle of two fractions, as short as possible.
pub(crate) fn diverge(base: i32, value1: &Digits, value2: &Digits) -> Digits {
    let mut before = value1.duplicate();
    let mut after = value2.duplicate();
    let mut repeated = Digits::new();
    super::compact_numbers(&mut before, &mut after, &mut repeated, HalfMode::Diverge);
    let value = super::half(base, &before, &after, true);
    let mut value = super::clean(value, &before, &after);
    value.stack(&repeated);
    value
}
