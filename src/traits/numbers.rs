use crate::structs::number::Number;

/// Extends the `String` struct to get positions from the number system
pub trait Numbers {
    /// Creates a `string` from a number
    /// * `position` - The number to parsed as a `string`
    /// * `number` - The number system to obtain the text information
    fn string_from(&self, number: &Number) -> String;
    /// Creates a number from a string using the number system where each key
    /// is the character from the string reversed position `i32`.
    /// * `value` - The string to be represented as a number
    fn number_from(&self, string: &str) -> Number;
}
