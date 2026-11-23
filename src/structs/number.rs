use std::collections::HashMap;

/// The `Number` values are just a number representation from any based numeric system.
/// Each index represents the corresponding position in such and the value the number
/// corresponding to that position.
///
/// ```rust
/// // Necessary imports
/// use staquia::implementations::numbers::Number;
///
/// // The base 10 number 123 is equal to (3///10^2) + (2///10^1) + (1///10^0)
/// // meaning that, would be stored within the object like:
/// let mut number = Number::new(0, 2);
/// number.insert(&2, 3);
/// number.insert(&1, 2);
/// number.insert(&0, 1);
/// ```
#[derive(Debug)]
pub struct Number {
    /// The internal dictionary of the number
    pub _map: HashMap<i32, i32>,
    /// The maximum upper bound for this code number
    pub upper_bound: i32,
    /// Will be used later to determine when to stop on evaluating the 'criteria'.
    /// The default value is the current final index of valueCodes
    /// This will always start with a lower bound of -1 to store overflows<
    pub lower_bound: i32,
}
