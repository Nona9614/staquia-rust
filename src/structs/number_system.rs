/// By default `Staquia` has a generic optimized number system.
/// To have settings that are meant to a specific project
/// go to the online [Staquia Documentation](https://nona9614.github.io/staquia-js).
#[derive(Debug)]
pub struct NumberSystem {
    /// The unicode character that represents the zero value
    pub zero: char,
    /// Reference to the zero value as code point from the `NumberSystem`
    pub zero_code: u32,
    /// Reference to the zero value as code point from the `NumberSystem` as `i32`
    pub zero_code_i32: i32,
    /// The unicode character that represents the first value in the set
    pub first: char,
    /// Reference to the first value as code point from the `NumberSystem`
    pub first_code: u32,
    /// Reference to the first value as code point from the `NumberSystem` as `i32`
    pub first_code_i32: i32,
    /// The unicode character that represents the last value in the set
    pub last: char,
    /// The base of the number set
    pub base: u16,
    /// Coeficient that helps to set a threshold to determine when a position
    /// string is starting to get too long
    pub k: f32,
    /// The `n` position length
    pub length_n: usize,
    /// The `z` position length
    pub length_z: usize,
    /// Calculated from the `κ` coeficient,
    /// is the length such that a position reaches this
    /// the system will fire a warning message.
    pub length_k: usize,
    /// Maximum length that a position can have
    pub length: usize,
}
