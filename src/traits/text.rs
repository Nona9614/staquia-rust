/// Service to be used to handle the text from a `NumberSystem`
pub trait Text {
    /// Builds a character using the passed number system.
    /// * `code` - The code to be converted to `char`
    fn get_char(&self, code: &u32) -> char;
    /// Builds a character from the system is returned as a code number
    /// * `value` - The character that is going to be converted to code
    fn get_code(&self, value: &char) -> u32;
    /// Builds a character from the system is returned as a code number `i32` value
    /// * `value` - The character that is going to be converted to code
    fn get_code_i32(&self, value: &char) -> i32;
    /// Checks if the character is the `zero` character
    /// * `value` - The character that will be checked
    fn is_zero(&self, value: &char) -> bool;
    /// Given an expected size, concats all missing leading zeroes
    /// * `value` - The original `number`
    /// * `size` - The max size of padding
    fn pad(&self, value: &mut String, size: &usize);
    /// Gets the room (space) between two characters
    /// * `after` - The left hand-side of the operation
    /// * `before` - The right hand-side of the operation
    fn room(&self, after: &char, before: &char) -> u32;
    /// Removes all leading zeroes from the string
    /// * `value` - The `number` that will get removed the leading `zeroes`
    fn trim(&self, value: &mut String);
}
