pub use crate::structs::number_system::NumberSystem;
pub use crate::traits::text::Text;

/// Service to be used to handle the text from a `NumberSystem`
impl Text for NumberSystem {
    /// Builds a character using the passed number system.
    /// * `code` - The code to be converted to `char`
    fn get_char(&self, code: &u32) -> char {
        if *code == 0 {
            self.zero.clone()
        } else {
            let point = *code + self.first_code - 1;
            let _char = char::from_u32(point);
            match _char {
                Some(c) => c,
                None => panic!("Error calculating the char value"),
            }
        }
    }
    /// Builds a character from the system then is returned as a unicode code point `u32`
    /// * `value` - The character that is going to be converted to code
    fn get_code(&self, value: &char) -> u32 {
        if *value == self.zero {
            0
        } else {
            let _value = (*value).clone() as u32;
            _value - self.first_code + 1
        }
    }
    /// Builds a character from the system is returned as a code number `i32` value
    /// * `value` - The character that is going to be converted to code
    fn get_code_i32(&self, value: &char) -> i32 {
        if *value == self.zero {
            0i32
        } else {
            let _value = (*value).clone() as u32 as i32;
            _value - self.first_code_i32 + 1
        }
    }
    /// Checks if the character is the `zero` character.
    /// * `value` - The character that will be checked
    fn is_zero(&self, value: &char) -> bool {
        self.zero == *value
    }
    /// Given an expected size, concats all missing leading zeroes.
    /// * `value` - The original `number`
    /// * `size` - The max size of padding
    fn pad(&self, value: &mut String, size: &usize) {
        let _chars = value.chars();
        let difference = *size - _chars.count();
        if difference > 0 {
            for _ in 0..difference {
                (*value).insert(0, self.zero);
            }
        }
    }
    /// Gets the room (space) between two characters.
    /// * `after` - The left hand-side of the operation
    /// * `before` - The right hand-side of the operation
    fn room(&self, after: &char, before: &char) -> u32 {
        self.get_code(after) - self.get_code(before)
    }
    /// Removes all leading zeroes from the string.
    /// * `value` - The `number` that will get removed the leading `zeroes`
    fn trim(&self, value: &mut String) {
        let mut _string = String::new();
        let mut _chars = value.chars();
        let mut _option = None;
        let mut _collect = true;

        loop {
            break match _option {
                Some(value) => {
                    if _collect {
                        _option = _chars.next();
                        if value != self.zero {
                            _string.push(value);
                            _collect = false;
                        }
                    } else {
                        _string.push(value);
                    }
                    continue;
                }
                None => {}
            };
        }

        *value = _string;
    }
}
