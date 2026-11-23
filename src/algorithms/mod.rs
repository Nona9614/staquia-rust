//! The arithmetic of the positions, a port of the JavaScript `aggregates/algorithms`.
//!
//! Every function works with the [`Digits`](crate::number::Digits) of a system of base `base` and keeps
//! the exact semantics of the JavaScript one, so both languages create the same positions.

mod after;
mod before;
mod between;
mod clean;
mod compact_numbers;
mod diverge;
mod half;
mod resolve_overflow;
mod substract;

pub(crate) use after::after;
pub(crate) use before::before;
pub(crate) use between::between;
pub(crate) use clean::clean;
pub(crate) use compact_numbers::{HalfMode, compact_numbers};
pub(crate) use diverge::diverge;
pub(crate) use half::half;
pub(crate) use resolve_overflow::resolve_overflow;
pub(crate) use substract::substract;

#[cfg(test)]
mod tests;
