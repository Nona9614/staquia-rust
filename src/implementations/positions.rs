use std::{cmp::Ordering, collections::HashMap};

use crate::implementations::text::NumberSystem;
use crate::{implementations::numbers::Number, traits::algorithms::Algorithms};

use crate::traits::numbers::Numbers;
use crate::traits::text::Text;
pub use crate::{structs::position::Position, traits::positions::Positions};

// Equal operators
impl PartialEq for Position {
    /// Checks if this position has the same numberic value as the passed.
    ///
    /// #### Example
    /// ```rust
    /// // Necesssary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Create the system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // Create the positions
    /// let a1 = system.forge("a");
    /// let a2 = system.forge("a");
    /// let b = system.forge("b");
    ///
    /// // Compare the positions
    /// let _true = a1 == a2;
    /// let _false = a1 == b;
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n && self.z == other.z
    }
    /// Checks if this position has <b>not</> the same numberic value as the passed.
    ///
    /// #### Example
    /// ```rust
    /// // Necesssary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Create the system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // Create the positions
    /// let a1 = system.forge("a");
    /// let a2 = system.forge("a");
    /// let b = system.forge("b");
    ///
    /// // Compare the positions
    /// let _true = a1 != b;
    /// let _false = a1 != a2;
    /// ```
    fn ne(&self, other: &Self) -> bool {
        self.n != other.n && self.z != other.z
    }
}
impl Eq for Position {}

// Ordering operators
impl PartialOrd for Position {
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` &gt;= `b` for sorting
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let _option1 = self.n.partial_cmp(&other.n);
        match _option1 {
            Some(ordering1) => {
                let _option2 = self.n.partial_cmp(&other.z);
                match _option2 {
                    Some(ordering2) => Some(ordering2),
                    None => Some(ordering1),
                }
            }
            None => None,
        }
    }
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` is <b>greater than</b> `b`
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    /// `returns` `true` if the comparison succeed, if not returns `false`
    fn gt(&self, other: &Self) -> bool {
        if self.n == other.n {
            self.z > other.z
        } else {
            self.n > other.n
        }
    }
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` is <b>greater or equal to</b> `b`
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    /// `returns` `true` if the comparison succeed, if not returns `false`
    fn ge(&self, other: &Self) -> bool {
        if self.n >= other.n {
            true
        } else {
            self.z >= other.z
        }
    }
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` is <b>less than</b> `b`
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    /// `returns` `true` if the comparison succeed, if not returns `false`
    fn lt(&self, other: &Self) -> bool {
        if self.n == other.n {
            self.z < other.z
        } else {
            self.n < self.n
        }
    }
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` is <b>less or equal to</b> `b`
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    /// `returns` `true` if the comparison succeed, if not returns `false`
    fn le(&self, other: &Self) -> bool {
        if self.n <= other.n {
            true
        } else {
            self.z <= other.z
        }
    }
}
impl Ord for Position {
    /// Compares the `string` values of two `Position` objects
    /// and checks if `a` &gt;= `b` for sorting
    /// * `self` - Left handside of the operation `Position`
    /// * `other` - Right handside of the operation `Position`
    fn cmp(&self, other: &Self) -> Ordering {
        let _ordering1 = self.n.cmp(&other.n);
        if _ordering1 == Ordering::Equal {
            self.z.cmp(&other.z)
        } else {
            _ordering1
        }
    }
}

/// Implmentation of Position struct
impl Position {
    /// Returns the segmenet as a single number
    pub fn to_number(&self) -> Number {
        let mut number = Number {
            upper_bound: self.n.upper_bound + self.z.upper_bound + 1,
            lower_bound: 0,
            _map: HashMap::new(),
        };

        let start = self.z.upper_bound + 1;
        for n in 0..=self.z.upper_bound {
            number.insert(&n, *self.z.get(&n));
        }
        for n in 0..=self.n.upper_bound {
            number.insert(&(n + start), *self.n.get(&n));
        }

        number
    }
}

/// Implements clone trait
impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            n: self.n.clone(),
            z: self.z.clone(),
        }
    }
}

/// Create position values using the `NumberSystem`.
impl Positions for NumberSystem {
    /// Creates a new position based on the current number system
    /// * `value` - The reference `number` to create a position
    ///
    /// #### Example
    /// ```rust
    /// // Necessary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::numbers::Number;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Creates the number system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // Building number xa
    /// let mut number = Number::new(0, 1);
    /// number.insert(&0, 24);
    /// number.insert(&1, 1);
    ///
    /// // You can create the position just like this
    /// let position = system.position_from(&number);
    /// ```
    fn position_from(&self, number: &Number) -> Position {
        let upper_bound = number.upper_bound;
        let length = upper_bound + 1;
        let length_n = self.length_n as i32;

        let mut n = Number::new(0, length_n - 1);
        let mut z: Number = Default::default();

        let mut ix: i32;
        let offset = upper_bound - length_n;
        let mut iu;
        if length > length_n {
            for i in offset..upper_bound {
                ix = upper_bound - i + offset;
                let code = number.get(&ix);
                iu = length_n - i + offset - 1;
                n.insert(&iu, *code);
            }
            for i in 0..=offset {
                ix = offset - i;
                let code = number.get(&ix);
                z.insert(&ix, *code);
            }
            z.upper_bound = (upper_bound - length_n) as i32;
        } else if length == length_n {
            for i in 0..=upper_bound {
                let code = number.get(&i);
                n.insert(&i, *code);
            }
            z.insert(&0, 0);
        } else {
            for i in 0..=upper_bound {
                ix = upper_bound - i;
                let code = number.get(&ix);
                n.insert(&ix, *code);
            }
            for i in (upper_bound + 1)..length_n {
                n.insert(&i, 0);
            }
            z.insert(&0, 0);
        }

        Position { n, z }
    }
    /// Resolves the values for `n` and `z`
    /// from the position and builds some string using as
    /// reference the passed number system
    fn string_from(&self, position: &Position) -> String {
        if position.z.is_empty() {
            Numbers::string_from(self, &position.n)
        } else {
            let n = Numbers::string_from(self, &position.n);
            let z = Numbers::string_from(self, &position.z);
            n + z.as_str()
        }
    }
    /// Creates a new position based on the current number system.
    /// * `value` - The reference `string` to create a position
    ///
    /// #### Example
    /// ```rust
    /// // Necessary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Creates the number system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // You can create the position just like this
    /// let position = system.forge("nnnnzzzz");
    ///
    /// // Or the string representation from your position
    /// let string = system.string_from(&position);
    ///
    /// // You can compare the instances with the less than '<',
    /// // greater than '>' or equal '=' operators
    /// let result = system.forge("a") < system.forge("b");
    /// ```
    ///
    /// #### Returns
    /// A new `position` instance.
    fn forge(&self, string: &str) -> Position {
        // Transforms the string as a vector of characters
        let _chars = string.chars();
        let mut value = Vec::new();
        let mut length = 0;
        for ch in _chars {
            value.push(ch);
            length += 1;
        }

        let mut n: Number = Default::default();
        n.upper_bound = (self.length_n - 1) as i32;
        let mut z: Number = Default::default();

        let length_n = self.length_n;
        let mut ix: i32;
        if length > length_n {
            for i in 0..length_n {
                ix = (length_n - i - 1) as i32;
                let ch = self.get_code_i32(&value[i]);
                n.insert(&ix, ch);
            }
            for i in length_n..length {
                ix = (length - i - 1) as i32;
                let ch = self.get_code_i32(&value[i]);
                z.insert(&ix, ch);
            }
            z.upper_bound = (length - length_n - 1) as i32;
        } else if length == length_n {
            for i in 0..length {
                ix = (length - i - 1) as i32;
                let ch = self.get_code_i32(&value[i]);
                n.insert(&ix, ch);
            }
            z.insert(&0, 0);
        } else {
            for i in 0..length {
                ix = (length - i - 1) as i32;
                let ch = self.get_code_i32(&value[i]);
                n.insert(&ix, ch);
            }
            for i in length..length_n {
                ix = i as i32;
                n.insert(&ix, 0);
            }
            z.insert(&0, 0);
        }

        Position { n, z }
    }
    /// Creates a recommended start position to depart, to have the most possible position forges
    /// before an overflow.
    ///
    /// ```rust
    /// // Necessary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Creates the number system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // For instance if the middle character in the system is n
    /// // and the n position length is 5, the recommended position
    /// // would be nnnnn
    /// let nnnnn = system.start();
    /// ```
    fn start(&self) -> Position {
        let middle = self.get_middle();
        let mut n = String::new();
        for _ in 0..(self.length_n) {
            n.push(middle);
        }
        self.forge(n.as_str())
    }
    /// Generates a value that lies between the current `position` and another.
    /// * `previous` - The `position` that should be before the <c>core</c> value
    /// * `next` - The `position` that should be after the <c>core</c> value
    ///
    /// #### Example
    ///  ```rust
    /// // Necessary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Creates the number system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // Given a system like this
    /// let zero: char = '!'; // Would be like 0
    /// let first: char = 'a'; // Would be like 1
    /// let middle: char = 'n'; // Would be like 14
    /// let last: char = 'z'; // Would be like 26
    ///
    /// // Builds the positions
    /// let a = system.forge("a");
    /// let b = system.forge("b");
    /// let z = system.forge("z");
    ///
    /// // The middle character between two positions
    /// // can be seen as Math.ceil((first + last) / 2);
    /// let n = system.core(&a, &z); // Will result in "n"
    ///
    /// // When there is no room between 2 positions, the "middle" value
    /// // from the system will be attached to the last position.
    /// // This is where the "threshold" and "overflow" handling
    /// // logic will be applied
    /// let am = system.core(&a, &b); // Will result in "am"
    /// ```
    /// #### Returns
    /// The value that lies between the current `previous` and `next`.
    fn core(&self, previous: &Position, next: &Position) -> Position {
        // Set the proper weight
        let mut _previous = previous;
        let mut _next = next;
        if previous > next {
            let temp = _next.clone();
            _next = &previous.clone();
            _previous = &temp;
        }

        // If there is not enough room (above 1) use diverge,
        // if not use between
        let _final = if self.is_divergent(&previous, &next) {
            self.diverge(&previous.to_number(), &next.to_number())
        } else {
            self.between(&previous.n, &next.n, true)
        };

        self.position_from(&_final)
    }
    /// Checks if the room between two `positions` `number` <c>n</c>
    /// values is equal or less than <c>1</c>.
    /// * `previous` - The `positions` that should be previous
    /// * `next` - The `positions` that should be next
    ///
    /// #### Returns
    /// `true` if the <c>n</c> values are at less or equal to <c>1</c> number appart from each
    /// other, if not returns `false`.
    fn is_divergent(&self, previous: &Position, next: &Position) -> bool {
        if previous.n == next.n {
            true
        } else {
            let after = self.after(&previous.n, true);
            after == next.n
        }
    }
    /// Gets the relative next `position` from another position.
    /// * `reference` - The reference `position` to get the next value
    ///
    /// ```rust
    /// // Necessary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Create a new number system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // The character <c>"a"</c> is followed by <c>"b"</c> in the unicode standard
    /// let nnna = system.forge("nnna");
    /// let nnnb = system.next(&nnna);
    /// ```
    /// #### Returns
    /// The generated next `position`
    fn next(&self, reference: &Position) -> Position {
        let after = self.after(&reference.n, true);
        self.position_from(&after)
    }
    /// Gets the relative next `position` from another position.
    /// * `reference` - The reference `position` to get the previous value
    ///
    /// ```rust
    /// // Necessary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Create a new number system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // The character <c>"b"</c> is followed by <c>"a"</c> in the unicode standard
    /// let nnnb = system.forge("nnnb");
    /// let nnna = system.previous(&nnnb);
    /// ```
    /// #### Returns
    /// The generated previous `position`
    fn previous(&self, reference: &Position) -> Position {
        let before = self.before(&reference.n, true);
        self.position_from(&before)
    }
}
