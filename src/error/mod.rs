//! The errors of the crate, as enums with `From` implementations for `?`.

mod create;
mod lexical;
mod position;
mod rejected;

pub use create::CreateError;
pub use lexical::LexicalError;
pub use position::{Direction, PositionError};
pub use rejected::Rejected;

use core::fmt;

/// Any error of the crate, so every operation can use `?` in the same function without boxing the errors.
/// `K` is the type of the ids of a [`LexicalList`](crate::LexicalList), `()` when no list is used.
///
/// # Example
///
/// ```
/// use staquia::Staquia;
///
/// fn first_two() -> Result<[String; 2], staquia::Error> {
///     let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?; // CreateError
///     let start = system.start()?; // PositionError
///     Ok([start.to_string(), system.next(&start)?.to_string()])
/// }
///
/// assert_eq!(first_two()?, ["mmmmm!", "mmmmn!"]);
/// # Ok::<(), staquia::Error>(())
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Error<K = ()> {
    /// A system could not be created.
    Create(CreateError),
    /// A position could not be created.
    Position(PositionError),
    /// An operation of a list failed.
    Lexical(LexicalError<K>),
}

impl<K> From<CreateError> for Error<K> {
    fn from(error: CreateError) -> Self {
        Self::Create(error)
    }
}

impl<K> From<PositionError> for Error<K> {
    fn from(error: PositionError) -> Self {
        Self::Position(error)
    }
}

impl<K> From<LexicalError<K>> for Error<K> {
    fn from(error: LexicalError<K>) -> Self {
        match error {
            LexicalError::Position(error) => Self::Position(error),
            error => Self::Lexical(error),
        }
    }
}

impl<K, V> From<Rejected<K, V>> for Error<K> {
    fn from(rejected: Rejected<K, V>) -> Self {
        rejected.error.into()
    }
}

impl<K: fmt::Debug> fmt::Display for Error<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create(error) => error.fmt(f),
            Self::Position(error) => error.fmt(f),
            Self::Lexical(error) => error.fmt(f),
        }
    }
}

impl<K: fmt::Debug> std::error::Error for Error<K> {}
