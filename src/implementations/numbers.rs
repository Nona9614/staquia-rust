use std::cmp::Ordering;
use std::collections::HashMap;

pub use crate::structs::number::Number;
use crate::{
    implementations::text::NumberSystem,
    traits::{numbers::Numbers, text::Text},
};

// Equal operators
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        let upper_bound = if self.upper_bound > other.upper_bound {
            self.upper_bound
        } else {
            other.upper_bound
        };
        let lower_bound = if self.lower_bound < other.lower_bound {
            self.lower_bound
        } else {
            other.lower_bound
        };
        let mut n: i32;
        let _offset = lower_bound + upper_bound;
        for _n in lower_bound..=upper_bound {
            n = _offset - _n;
            let a = self.get(&n);
            let b = other.get(&n);
            if *a != *b {
                return false;
            }
        }
        return true;
    }
    fn ne(&self, other: &Self) -> bool {
        let upper_bound = if self.upper_bound > other.upper_bound {
            self.upper_bound
        } else {
            other.upper_bound
        };
        let lower_bound = if self.lower_bound < other.lower_bound {
            self.lower_bound
        } else {
            other.lower_bound
        };
        let _offset = lower_bound + upper_bound;
        for n in lower_bound..=upper_bound {
            let a = self.get(&n);
            let b = other.get(&n);
            if *a != *b {
                return true;
            }
        }
        return false;
    }
}
impl Eq for Number {}

// Ordering operators
impl PartialOrd for Number {
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` &gt;= `b` for sorting
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut is_equal = true;
        let upper_bound = if self.upper_bound > other.upper_bound {
            self.upper_bound
        } else {
            other.upper_bound
        };
        let lower_bound = if self.lower_bound < other.lower_bound {
            self.lower_bound
        } else {
            other.lower_bound
        };
        let mut n: i32;
        let _offset = lower_bound + upper_bound;
        for _n in lower_bound..=upper_bound {
            n = _offset - _n;
            let a = self.get(&n);
            let b = other.get(&n);
            if *a > *b {
                return Some(Ordering::Greater);
            }

            if *a != *b {
                is_equal = false;
            }
        }

        if is_equal {
            return Some(Ordering::Equal);
        } else {
            return Some(Ordering::Less);
        }
    }
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` is <b>greater than</b> `b`
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    /// `returns` `true` if the comparison succeed, if not returns `false`
    fn gt(&self, other: &Self) -> bool {
        let upper_bound = if self.upper_bound > other.upper_bound {
            self.upper_bound
        } else {
            other.upper_bound
        };
        let lower_bound = if self.lower_bound < other.lower_bound {
            self.lower_bound
        } else {
            other.lower_bound
        };
        let mut n: i32;
        let _offset = lower_bound + upper_bound;
        for _n in lower_bound..=upper_bound {
            n = _offset - _n;
            let a = self.get(&n);
            let b = other.get(&n);
            if *a != *b {
                return *a > *b;
            }
        }
        return false;
    }
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` is <b>greater or equal to</b> `b`
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    /// `returns` `true` if the comparison succeed, if not returns `false`
    fn ge(&self, other: &Self) -> bool {
        let upper_bound = if self.upper_bound > other.upper_bound {
            self.upper_bound
        } else {
            other.upper_bound
        };
        let lower_bound = if self.lower_bound < other.lower_bound {
            self.lower_bound
        } else {
            other.lower_bound
        };
        let mut n: i32;
        for _n in lower_bound..=upper_bound {
            n = upper_bound - _n;
            let a = self.get(&n);
            let b = other.get(&n);
            if *a != *b {
                return *a >= *b;
            }
        }
        return true;
    }
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` is <b>less than</b> `b`
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    /// `returns` `true` if the comparison succeed, if not returns `false`
    fn lt(&self, other: &Self) -> bool {
        let upper_bound = if self.upper_bound > other.upper_bound {
            self.upper_bound
        } else {
            other.upper_bound
        };
        let lower_bound = if self.lower_bound < other.lower_bound {
            self.lower_bound
        } else {
            other.lower_bound
        };
        let mut n: i32;
        let _offset = lower_bound + upper_bound;
        for _n in lower_bound..=upper_bound {
            n = _offset - _n;
            let a = self.get(&n);
            let b = other.get(&n);
            if *a != *b {
                return *a < *b;
            }
        }
        return false;
    }
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` is <b>less or equal to</b> `b`
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    /// `returns` `true` if the comparison succeed, if not returns `false`
    fn le(&self, other: &Self) -> bool {
        let upper_bound = if self.upper_bound > other.upper_bound {
            self.upper_bound
        } else {
            other.upper_bound
        };
        let lower_bound = if self.lower_bound < other.lower_bound {
            self.lower_bound
        } else {
            other.lower_bound
        };
        let mut n: i32;
        let _offset = lower_bound + upper_bound;
        for _n in lower_bound..=upper_bound {
            n = _offset - _n;
            let a = self.get(&n);
            let b = other.get(&n);
            if *a != *b {
                return *a < *b;
            }
        }
        return true;
    }
}
impl Ord for Number {
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` &gt;= `b` for sorting
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    fn cmp(&self, other: &Self) -> Ordering {
        let mut is_equal = true;
        let upper_bound = self.upper_bound;
        let lower_bound = self.lower_bound;
        for n in upper_bound..=lower_bound {
            let a = self.get(&n);
            let b = other.get(&n);
            if *a > *b {
                return Ordering::Greater;
            }

            if *a != *b {
                is_equal = false;
            }
        }

        if is_equal {
            return Ordering::Equal;
        } else {
            return Ordering::Less;
        }
    }
}

impl Numbers for NumberSystem {
    /// Transforms back the number to a `string` reprsentation
    /// using the number system.
    fn string_from(&self, number: &Number) -> String {
        let mut string = String::new();
        let mut n: i32;
        for _n in number.lower_bound..=number.upper_bound {
            n = number.upper_bound - _n;
            let _option = number._map.get(&n);
            match _option {
                Some(v) => {
                    let value = (*v) as u32;
                    let ch = self.get_char(&value);
                    string.push(ch);
                }
                None => {}
            }
        }
        string
    }
    /// Creates a number from a string using the number system where each key
    /// is the character from the string reversed position `i32`.
    /// * `value` - The string to be represented as a number
    fn number_from(&self, string: &str) -> Number {
        let _chars = string.chars();
        let mut _codes = Vec::new();

        let mut _length = 0;
        for ch in _chars {
            _codes.push(self.get_code(&ch) as i32);
            _length += 1;
        }

        let mut number = Number {
            lower_bound: 0,
            upper_bound: _length - 1,
            _map: HashMap::new(),
        };

        let mut n = 0;
        for code in _codes {
            let i = _length - n - 1;
            number._map.insert(i, code);
            n += 1;
        }

        number
    }
}

/// The `Number` values are just a number representation from any based numeric system.
/// Each index represents the corresponding position in such and the value the number
/// corresponding to that position.
///
/// ```rust
/// // Necessary imports
/// use staquia::implementations::numbers::Number;
///
/// // The base 10 number 123 is equal to (3*10^2) + (2*10^1) + (1*10^0)
/// // meaning that, would be stored within the object like:
/// let mut number = Number::new(0, 2);
/// number.insert(&2, 3);
/// number.insert(&1, 2);
/// number.insert(&0, 1);
/// ```
impl Number {
    /// Creates a new instance from the bound values
    /// * `lower_bound` - The lower bound index of the number
    /// * `upper_bound` - The upper bound index of the number
    pub fn new(lower_bound: i32, upper_bound: i32) -> Self {
        Self {
            lower_bound: lower_bound,
            upper_bound: upper_bound,
            _map: HashMap::new(),
        }
    }
    /// Stacks the passed number values at the top of this number.
    /// * `other` - The number to be stack at the top
    pub fn stack(&mut self, other: &Number) {
        let mut upper_bound = self.upper_bound as i32;
        for i in other.lower_bound..=other.upper_bound {
            let value = other.get(&i);
            upper_bound += 1;
            self.insert(&upper_bound, *value);
        }
        self.upper_bound = upper_bound;
    }
    /// Removes all leading zeroes from the string.
    /// * `self` - The `number` that will get removed the leading `zeroes`
    pub fn compact(&mut self) {
        // Iterate from the upper bound to '0' to remove all leading zeroes
        let mut i: i32;
        let mut upper_bound = self.upper_bound;
        for n in 0..=self.lower_bound {
            i = self.upper_bound - n;
            if self.is_zero(&i) {
                self._map.remove(&i);
                upper_bound -= 1;
            } else {
                break;
            }
        }
        // If while removing the upper bound reached '0' force the
        // object to at least contain one simple value '0' value
        if upper_bound <= 0 {
            upper_bound = 0;
            self.insert(&0, 0);
        }
        // Updates to the new upper bound
        self.upper_bound = upper_bound;
    }
    /// Expands the number with the number of `size` zeroes to the <b>right</b>.
    /// * `size` - The `number` number of values to expand the number
    pub fn expand(&mut self, size: i32) {
        let _upper_bound: i32 = self.upper_bound;
        let _size: i32 = size + _upper_bound + 1;
        self.upper_bound = _upper_bound + size;
        for n in (_upper_bound + 1).._size {
            self._map.insert(n, 0);
        }
    }
    /// Extends the number with the number of `size` zeroes to the <b>left</b>.
    /// * `size` - The `number` number of values to expand the number
    pub fn extend(&mut self, size: i32) -> &Self {
        // Make this number as large as the new size
        let upper_bound: i32 = self.upper_bound;
        self.upper_bound = size + upper_bound;
        // Reorder already stored values
        let mut n: i32;
        let mut up: i32;
        let mut code: i32;
        for _n in 0..=upper_bound {
            n = upper_bound - _n;
            up = n + size;
            code = *self.get(&n);
            self._map.insert(up, code);
        }
        for n in 0..size {
            self._map.insert(n, 0);
        }
        // Returns this instance
        self
    }
    /// Pushes a new value in the last part of the number.
    /// * `value` - The number value to be pushed
    pub fn push(&mut self, value: i32) {
        self._map.insert(self.upper_bound + 1, value);
        self.upper_bound += 1;
    }
    /// Removes the most significant number in this object.
    pub fn pop(&mut self) {
        let _option = self._map.remove(&self.upper_bound);
        match _option {
            Some(_) => {
                self.upper_bound -= 1;
            }
            None => {}
        }
    }
    /// Removes the less significant number in this object.
    pub fn unsafe_shift(&mut self) {
        self._map.remove(&self.lower_bound);
        self.lower_bound += 1;
    }
    /// Removes the digit from the number using the key
    /// * `key` - The key of the item to be removed
    pub fn remove(&mut self, key: &i32) -> Option<i32> {
        self._map.remove(key)
    }
    /// Gets the digit from the number using the key, but if it is not found returns `0`.
    /// * `key` - The key of the item to get
    pub fn get(&self, key: &i32) -> &i32 {
        let _option = self._map.get(key);
        match _option {
            Some(v) => v,
            None => &0,
        }
    }
    /// Inserts the specified key and value to the number.
    /// * `key` - The key of the value to be added
    /// * `value` - The value to be added
    pub fn insert(&mut self, key: &i32, value: i32) {
        self._map.insert(*key, value);
    }
    /// Updates the value of the selected key summing the offset.
    /// * `key` - The key of the value to be updated
    /// * `offset` - The offset to be summed
    pub fn update(&mut self, key: &i32, offset: i32) {
        let _option = self._map.get(&key);
        match _option {
            Some(v) => {
                let value = (*v) + offset;
                self._map.insert(*key, value);
            }
            None => {
                self.insert(key, offset);
            }
        }
    }
    /// Checks if the passed `key` value is above or equal to the number system base value.
    /// * `key` - The key that may contain the overflow
    pub fn has_overflow(&self, base: &i32, key: &i32) -> bool {
        let _option = self._map.get(key);
        match _option {
            Some(v) => (*v) >= (*base),
            None => false,
        }
    }
    /// Checks if the passed `key` contains the `0` value
    /// * `key` - The key that may be `0`
    pub fn is_zero(&self, key: &i32) -> bool {
        *self.get(key) == 0
    }
    /// Checks if the number contains only a zero value and no upper neither lower bounds
    pub fn is_empty(&self) -> bool {
        *self.get(&0) == 0 && self.upper_bound == 0 && self.lower_bound == 0
    }
}

impl Clone for Number {
    /// Creates a copy of this object in a new instance
    fn clone(&self) -> Number {
        Number {
            lower_bound: self.lower_bound,
            upper_bound: self.upper_bound,
            _map: self._map.clone(),
        }
    }
}

impl Default for Number {
    // Creates a default number having an empty hash map
    fn default() -> Number {
        Number {
            lower_bound: 0,
            upper_bound: 0,
            _map: HashMap::new(),
        }
    }
}
