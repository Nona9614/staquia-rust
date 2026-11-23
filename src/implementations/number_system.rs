pub use crate::structs::number_system::NumberSystem;

/// By default `Staquia` has a generic optimized number system.
/// To have settings that are meant to a specific project
/// go to the online [Staquia Documentation](https://nona9614.github.io/staquia-js).
impl NumberSystem {
    /// Builds a preconfigured number system with default values
    ///
    /// #### Example
    /// ```rust
    /// // Necessary imports
    /// use staquia::implementations::number_system::NumberSystem;
    ///
    /// // This is the same as call the `custom` function with this default values
    /// let system = NumberSystem::custom('!', '°', 'ұ', 4, 15, 0.15);
    /// ```
    pub fn new() -> Self {
        Self {
            zero: '!',
            zero_code: 0x0021,
            zero_code_i32: 0x0021i32,
            first: '\u{00B0}',
            first_code: 0x00B0,
            first_code_i32: 0x00B0i32,
            last: '\u{04B1}',
            length_n: 4,
            length_z: 12,
            length_k: 14,
            k: 0.15,
            base: 0x0403,
            length: 16usize,
        }
    }
    /// Builds a new number system from the input values
    pub fn custom(zero: char, first: char, last: char, n: usize, z: usize, k: f32) -> Self {
        let _zero = zero.clone() as u32;
        let _first = first.clone() as u32;
        let _last = last.clone() as u32;

        let length = n + z;
        let length_k = ((1.0 - k) * (length as f32)) as usize;
        let base = (_last - _first + 2) as u16;

        return NumberSystem {
            zero: zero,
            zero_code: _zero,
            zero_code_i32: _zero as i32,
            first: first,
            first_code: _first,
            first_code_i32: _first as i32,
            last: last,
            length_n: n,
            length_z: z,
            length_k: length_k,
            k: k,
            base: base,
            length: length as usize,
        };
    }
    /// Gets the base as i32
    pub fn get_base_i32(&self) -> i32 {
        self.base as i32
    }
    /// Gets the zero value as a string
    pub fn get_zero(&self) -> String {
        self.zero.to_string()
    }
    /// The unicode character that represents the middle value in the set
    pub fn get_middle(&self) -> char {
        let last_code = self.last as u32;
        let first_code = self.first as u32;
        let a_plus_b = (last_code + first_code - 1) as f32;
        let middle_point = (a_plus_b / 2f32).ceil() as u32;

        // Middle point should be added the zero char
        let middle = char::from_u32(middle_point);
        match middle {
            Some(m) => m,
            None => panic!("Error calculating the middle value"),
        }
    }
}
