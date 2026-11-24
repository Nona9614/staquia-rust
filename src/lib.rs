//! Staquia creates **positions**: short strings that keep the order of a list.
//!
//! Sort the items by their position string and you get the order back; moving an item only changes its own
//! position, so only one row is written to the database per change. It is the Rust version of the
//! [staquia](https://nona9614.github.io/staquia-js/) JavaScript library: for the same system and operations,
//! both create the same positions.
//!
//! The guides (how positions work, storing them in a database, error handling) are on the
//! [documentation site](https://nona9614.github.io/staquia-js/): choose **Rust** in its language selector to
//! see the examples in Rust.
//!
//! - [`Staquia`] is a number system that creates [`Position`]s.
//! - [`LexicalList`] keeps the order of your values and their positions in sync.
//!
//! Nothing panics: every operation that can fail returns a [`Result`] with an error enum
//! ([`PositionError`], [`LexicalError`]) that works with `?`.
//!
//! # Example
//!
//! ```
//! use staquia::Staquia;
//!
//! let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
//!
//! let first = system.start()?; // "mmmmm!"
//! let second = system.next(&first)?; // "mmmmn!"
//! let between = system.core(&first, &second)?; // "mmmmmn"
//!
//! let mut positions = [second.to_string(), first.to_string(), between.to_string()];
//! positions.sort();
//! assert_eq!(positions, ["mmmmm!", "mmmmmn", "mmmmn!"]);
//! # Ok::<(), staquia::Error>(())
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]
// `Rejected` gives the values back on failure instead of boxing them
#![allow(clippy::result_large_err)]

mod algorithms;
mod error;
mod list;
mod number;
mod position;
mod staquia;
#[cfg(test)]
mod test_support;

pub use error::{CreateError, Direction, Error, LexicalError, PositionError, Rejected};
pub use list::{Item, Iter, LexicalItem, LexicalList, Replaced};
pub use position::Position;
pub use staquia::{Options, Staquia};

/// The maximum length of a position (`ln + lz`) of any system.
///
/// Positions live on the stack in an array of this size, so they never allocate.
pub const MAX_LENGTH: usize = 32;

/// Runs the examples of the README as doc tests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
