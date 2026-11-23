use crate::structs::number::Number;

/// Positions `string` values from a position
#[derive(Debug)]
pub struct Position {
    /// Position `n` <b>number</b> from a position
    pub n: Number,
    /// Position `z` <b>number</b> from a position
    pub z: Number,
}
