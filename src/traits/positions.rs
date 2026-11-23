use crate::structs::number::Number;
pub use crate::structs::position::Position;

/// Creates positions using a base `NumberSystem`
pub trait Positions {
    /// Creates a new position based on the current number system.
    /// * `value` - The reference `number` to create a position
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
    /// // Creates the number system
    /// let number = system.forge("nnnna");
    ///
    /// // You can create the position just like this
    /// let a = system.position_from(&number.z);
    /// ```
    ///
    /// #### Returns
    /// A new `position` instance
    fn position_from(&self, number: &Number) -> Position;
    /// Resolves the values for `n` and `z` from the position and builds some
    /// string using as reference the passed number system.
    fn string_from(&self, position: &Position) -> String;
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
    fn forge(&self, string: &str) -> Position;
    /// Creates a recommended start position to depart, to have the most possible position forges
    /// before an overflow
    /// ```rust
    /// // Necessary imports
    /// use staquia::implementations::number_system::NumberSystem;
    /// use staquia::implementations::positions::Positions;
    ///
    /// // Creates the number system
    /// let system = NumberSystem::custom('!', 'a', 'z', 4, 15, 0.15);
    ///
    /// // For instance if the middle character in the system is 'n'
    /// // and the 'n' position length is 5, the recommended position
    /// // would be "nnnnn"
    /// let nnnnn = system.start();
    /// ```
    fn start(&self) -> Position;
    /// Generates a value that lies between the current `position` and another
    /// * `previous` - The `position` that should be before the <c>core</c> value
    /// * `next` - The `position` that should be after the <c>core</c> value
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
    ///
    /// #### Returns
    /// The value that lies between the current `previous` and `next`
    fn core(&self, previous: &Position, next: &Position) -> Position;
    /// Checks if the room between two `positions` `string` <c>n</c>
    /// values is equal or less than <c>1</c>.
    /// * `previous` - The `positions` that should be previous
    /// * `next` - The `positions` that should be next
    ///
    /// #### Returns
    /// `true` if the <c>n</c> values are at less or equal to <c>1</c> number appart from each
    /// other, if not returns `false`.
    fn is_divergent(&self, previous: &Position, next: &Position) -> bool;
    /// Gets the relative next `position` from another position.
    /// * `reference` - The reference `position` to get the next value
    ///
    /// #### Example
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
    ///
    /// #### Returns
    /// The generated next `position`.
    fn next(&self, reference: &Position) -> Position;
    /// Gets the relative next `position` from another position
    /// * `reference` - The reference `position` to get the previous value
    ///
    /// #### Example
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
    ///
    /// #### Returns
    /// The generated previous `position`
    fn previous(&self, reference: &Position) -> Position;
}
