use crate::enums::HalfMode;
use crate::implementations::numbers::Number;
pub use crate::implementations::text::NumberSystem;
pub use crate::traits::algorithms::Algorithms;

/// A collection of algorithms to calculate number values    
impl Algorithms for NumberSystem {
    /// Resolves the following value of the given number
    /// * `value` - The input `number`
    /// * `trim` - If set to `true` trims the input value before resolve
    /// `returns` The resolved next `number`
    fn after(&self, value: &Number, trim: bool) -> Number {
        // Creates a copy of the current number
        let mut number = value.clone();
        // Adds one to the start value
        number.update(&0, 1);
        // If trim is set, removes all leading zeroes
        if trim {
            number.compact();
        }
        // Adds extra space in case of overflow
        number.expand(1);
        // Resolves the overflow
        self.resolve_overflow(&mut number);
        // If the expanded final digit is 0, remove it again
        if number.is_zero(&number.upper_bound) {
            number.pop();
        }
        // Returns the generated number
        number
    }
    /// Resolves the previous value of the given number    
    /// * `system` - The `INumberSystem` reference
    /// * `value` - The input `number`
    /// * `trim` - If set to `true` trims the input value before resolve
    /// `returns` The resolved previous `number`
    fn before(&self, value: &Number, trim: bool) -> Number {
        // Creates a copy of the current number
        let mut number = value.clone();
        // Removes one to the start value
        number.update(&0, -1);
        // If trim is set, removes all leading zeroes
        if trim {
            number.compact();
        }
        // Adds extra space in case of overflow
        number.expand(1);
        // Carry values until the number is not negative
        let base = self.get_base_i32();
        let mut _v: &i32;
        for n in number.lower_bound..=number.upper_bound {
            _v = number.get(&n);
            if *_v < 0 {
                number.update(&n, base);
                number.update(&(n + 1), -1);
            } else {
                break;
            }
        }
        // If the expanded final digit is 0, remove it again
        if *number.get(&number.upper_bound) <= 0 {
            number.pop();
        }
        // Returns the generated number
        number
    }
    /// Because an odd base is being used, to get the mid position for any number
    /// the equation q = (nf + n0) / 2, where:
    /// - `n0` represents the start string converted from the nth base to the 10th base
    /// - `n1` represents the end string converted from the nth base to the 10th base
    /// * `before` - In the evaluation of two strings `s1 &lt; s2` must be `s1`
    /// * `after` - In the evaluation of two strings `s1 &lt; s2` must be `s2`
    /// * `round` - Rounds the result to the nearest upper integer if set to `true`
    /// `returns` The value q - ( q mod 2 )
    /// When q mod 2 is different from 0 then the mid character
    /// from the set of the nth base, should be placed at the end of the output.
    fn between(&self, before: &Number, after: &Number, round: bool) -> Number {
        // Builds the numbers
        let mut before_codes: Number = before.clone();
        let mut after_codes: Number = after.clone();
        let mut repeated: Number = Default::default();
        self.compact_numbers(
            &mut before_codes,
            &mut after_codes,
            &mut repeated,
            HalfMode::Between,
        );

        // Makes the calculation to get the mid value
        let mut value_codes = self.half(&before_codes, &after_codes, round);
        // Stacks the repated values
        value_codes.stack(&repeated);
        // Returns the generated string
        value_codes
    }
    /// This system function will equalize the length of `value1` and `value2`
    /// adding the zero character to the left of the shorter string if needed, then will
    /// find the point where `CharacterBefore &lt;&gt; CharacterAfter`.
    /// Upon the `room` between these:
    /// - If the `room` &gt; 1 will only find the value between them
    /// - If `room` &lt;= 1 will use and optimized version of the Between function
    /// <br>
    /// #### Params
    /// * `value1` - Must be any value such that `value1 &lt; value2` or `value2 &lt; value1`
    /// * `value2` - Must be any value such that `value1 &lt; value2` or `value2 &lt; value1`
    /// `returns` The `string` that represents the diverged value
    fn diverge(&self, value1: &Number, value2: &Number) -> Number {
        // Builds the numbers found
        let mut before_codes: Number = value1.clone();
        let mut after_codes: Number = value2.clone();
        let mut repeated: Number = Default::default();
        self.compact_numbers(
            &mut before_codes,
            &mut after_codes,
            &mut repeated,
            HalfMode::Diverge,
        );

        // Makes the calculation to get the mid value
        let mut value_codes = self.half(&before_codes, &after_codes, true);

        // Cleans the number
        match NumberSystem::clean(&value_codes, &before_codes, &after_codes) {
            Some(v) => {
                value_codes = v;
            }
            _ => {}
        }
        // Stacks the repated values
        value_codes.stack(&repeated);
        // Returns the generated string
        value_codes
    }
    /// Ignores the zero values from the left until a `number` is found,
    /// then pass any repeated values to the <paramref name="repeated"/> `string`
    /// and finally stores the different values as `code`
    /// * `number1` - The first `Number` generated
    /// * `number2` - The second `Number` generated
    /// * `repeated` - The recovered `string` of repeated values
    /// * `value1` - The first `string` to be converterd
    /// * `value2` - The second `string` to be converterd
    /// Both of the numbers will contain a `Number.LowerBound` value of -1 with
    /// a value of `0`.
    fn compact_numbers(
        &self,
        value1: &mut Number,
        value2: &mut Number,
        repeated: &mut Number,
        mode: HalfMode,
    ) {
        // These arrays will parse the string representing a nth base number where
        // the numbers will have the same size including zeroes (to the right for
        // diverge or left for between) to the one that needs it
        // i.e. yz ~ zzy --> yz! ~ zzy

        // Based on the half type reorder the items
        match mode {
            HalfMode::Between => {
                let diff = value1.upper_bound - value2.upper_bound;
                // Pads to the left with zeroes
                if diff < 0 {
                    value1.expand(-diff);
                }
                // Pads to the left with zeroes
                else if diff > 0 {
                    value2.expand(diff);
                }
            }
            HalfMode::Diverge => {
                let diff = value1.upper_bound - value2.upper_bound;
                // Pads to the right with zeroes
                if diff < 0 {
                    value1.extend(-diff);
                }
                // Pads to the right with zeroes
                else if diff > 0 {
                    value2.extend(diff);
                }
            }
        }

        // Ensure value1 < value2
        if value1 > value2 {
            let temp = value1.clone();
            *value1 = value2.clone();
            *value2 = temp;
        }

        // The containing repeated values
        let mut repeated_codes = Vec::new();

        // Removes any repeated values until their differ
        let mut code1: i32;
        let mut code2: i32;
        let mut index = value1.upper_bound;
        while index > -1 {
            code1 = *value1.get(&index);
            code2 = *value2.get(&index);
            if code1 == code2 {
                // Removes the repetead digit
                value1.pop();
                value2.pop();
                // Appends the new found repeated number
                repeated_codes.push(code1);
                index -= 1;
            } else {
                break;
            }
        }

        // Update the repeated values
        repeated.upper_bound = (repeated_codes.len() as i32) - 1;
        let mut ix: i32 = repeated.upper_bound;
        for code in repeated_codes {
            repeated.insert(&ix, code);
            ix -= 1;
        }
    }
    /// Evaluates two values to check which one goes before, gets the repeated values and
    /// after and then transforms them to code.
    /// * `before_codes` - The before codes
    /// * `after_codes` - The after codes
    /// * `round` - Defines if the value should be rounded upwards
    fn half(&self, before_codes: &Number, after_codes: &Number, round: bool) -> Number {
        // Here is where al resolved codes from the main diverge algorith will be placed.
        // Also, system will be selfaware of how long is his set
        let mut value_codes = Number::new(-1, before_codes.upper_bound);

        // The 'middle point' between two numbers can be found like:
        // let middle = (|p1 - p2| / 2) + p1, thus symplyfing: middle = (p1 + p2) / 2
        // system will be the number that will be being pointed to get the 'average weight'
        // from the strings to be evaluated.

        // Used for iterations
        let mut i: i32;
        // Will store the trailing value each time an overflow takes place
        let mut trailing = 0f32;
        // Will store the floating number value from the average
        let mut q: f32;
        // Will store the floating number value from raw division
        let mut d: f32;
        // Will store the remaining value from the average
        let mut m: f32;
        // The base as a float value
        let base = self.base as f32;

        // Iterate down to the extra indexes
        let mut _offset = value_codes.upper_bound + value_codes.lower_bound;
        for _i in value_codes.lower_bound..=value_codes.upper_bound {
            // Calculates the inverse value
            i = _offset - _i;
            // Gets the after and before code
            let previous_code = after_codes.get(&i);
            let next_code = before_codes.get(&i);
            // Gets the '(p1 + p2) / 2' value (plus the trailing if exists)
            d = ((*previous_code + *next_code) as f32) / 2f32;
            q = d + trailing;
            // Gets the remainder from the previous equation
            m = q - q.floor();
            // Removes the remainder to the original value
            q -= m;
            // Gets the trail to be added to the following nth number (n-1)
            trailing = m * base;
            // Stores the result
            value_codes.insert(&i, q as i32);
        }

        // If the last index was not used will be removed
        let has_decimals = !value_codes.is_zero(&-1);

        // Creates a new number that will contain the substractions
        // system will store the room of each evaluation to determine later if there
        // is need to round the result
        let mut substraction_codes = Number::new(value_codes.lower_bound, value_codes.upper_bound);

        // As what is expected is only the space between the numbers, the substraction must be
        // an absolute value
        let mut up_codes = (*after_codes).clone();
        let mut down_codes = (*before_codes).clone();
        // Adds extra number in the upper bound to avoid overflow
        substraction_codes.push(0);
        up_codes.push(0);
        down_codes.push(0);

        // Iterate up to apply the substraction algorithm
        for i in 0..=value_codes.upper_bound {
            // If the difference from the above number is smaller than the previous
            // carry until are values are lined or the final number is reached, example:
            //      c     !     ! -
            //      !     !     z =
            //      ---------------
            //      c     !   (-z)
            //      ---------------
            //      c   (!-a)(a!-z)
            //      ---------------
            //      c   (-a)    a
            //      ---------------
            //      (c-a)(!a-a) a
            //      ---------------
            //      b     z     a
            let mut ix = i;
            // Values recovered
            let mut substraction_code: i32;
            let mut up_code: &i32;
            let mut down_code: &i32;
            // Iterates until all carry is resolved
            loop {
                // Gets the values
                up_code = up_codes.get(&ix);
                down_code = down_codes.get(&ix);
                substraction_code = *up_code - *down_code;
                substraction_codes.insert(&ix, substraction_code);
                // If any of these conditions are accomplished break
                if substraction_code >= 0 || ix > value_codes.upper_bound {
                    break;
                }
                // Gets the following number to get carry
                let following_index = ix + 1;
                // Reduces that number by one
                up_codes.update(&following_index, -1);
                // Sums the base in the down number
                substraction_codes.update(&ix, self.get_base_i32());
                // Goes to next number
                ix += 1;
            }
        }

        // If the substraction result is not at least bigger than 2 it means it cannot be rounded
        // as doing so will cause the number to look like the after code
        let mut is_roundable = false;
        let mut one_codes = Number::new(
            substraction_codes.lower_bound,
            substraction_codes.upper_bound,
        );
        one_codes.insert(&0, 1);

        i = substraction_codes.upper_bound;
        while i > -1 {
            let substraction_code = substraction_codes.get(&i);
            let one_code = one_codes.get(&i);
            if substraction_code != one_code {
                is_roundable = substraction_code > one_code;
                break;
            }
            i -= 1;
        }

        // If note has decimals remove the last code
        if !has_decimals {
            value_codes.unsafe_shift();
        }

        // If the number it has enough space between the values, the round flag is passed
        // and has remianing values, it will round it upwards
        if has_decimals && round && is_roundable {
            value_codes.unsafe_shift();
            value_codes.update(&0, 1);
        }

        // If any number is above its base it has an 'overflow' therfore, that extra values
        // can be pushed to the n+1 position, for system case we can be sure that any overflow is only
        // above 1 times the base as the average function cannot cause to go beyond that
        let mut overflow;
        for i in value_codes.lower_bound..=value_codes.upper_bound {
            overflow = (*value_codes.get(&i) as f32) / base;
            if overflow >= 1f32 {
                value_codes.update(&i, -self.get_base_i32());
                value_codes.update(&(i + 1), 1);
            }
        }

        value_codes
    }

    /// Checks when a value is between two `string` values such that:
    /// `before` &lt; `value` &lt; `after`
    /// * `value` - The value that will be checked to be beween the `numbers`
    /// * `before` - The lower bound `number`
    /// * `after` - The upper bound `number`
    /// `returns` the cleaned number
    fn clean(value: &Number, before: &Number, after: &Number) -> Option<Number> {
        let mut number = Number::new(0, -1);

        let mut n: i32;
        let mut m: i32;
        let _offset = value.lower_bound + value.upper_bound;
        let mut cleaned = false;
        for _n in value.lower_bound..=value.upper_bound {
            n = _offset - _n;
            let mut a = false; // Flag - After difference with value is bigger
            let mut b = false; // Flag - Before difference with value is bigger

            // Goes from the upper bound to n to check that  before < value < after
            for _m in n..=value.upper_bound {
                m = value.upper_bound - _m + n;
                let value_code = value.get(&m);
                let after_code = after.get(&m);
                let before_code = before.get(&m);

                b = b || (*before_code < *value_code);
                a = a || (*value_code < *after_code);
            }

            // If the difference is before < value < after end collection
            // and builds a number from n to the last length
            if a && b {
                for _m in n..=value.upper_bound {
                    m = value.upper_bound - _m + n;
                    let value_code = value.get(&m);
                    number.insert(&(m - n), *value_code);
                    number.upper_bound += 1;
                }
                cleaned = true;
                break;
            }
        }

        if cleaned { Some(number) } else { None }
    }
    /// If the number has overflows it will resolve them
    fn resolve_overflow(&self, number: &mut Number) {
        // If any number is above its base it has an 'overflow' therfore, that extra values
        // can be pushed to the n+1 position, for system case we can be sure that any overflow is only
        // above 1 times the base as the average function cannot cause to go beyond that
        let base = (self.base) as i32;
        for n in number.lower_bound..number.upper_bound {
            if number.has_overflow(&base, &n) {
                number.update(&n, -base);
                number.update(&(n + 1), 1);
            }
        }
    }
}
