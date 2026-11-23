use crate::enums::HalfMode;
use crate::structs::number::Number;

/// A collection of algorithms to calculate number values    
pub trait Algorithms {
    /// Resolves the following value of the given number
    /// * `value` - The input `number` `Number`
    /// * `trim` - If set to `true` trims the input value before resolve
    /// `returns` The resolved next `number`
    fn after(&self, value: &Number, trim: bool) -> Number;
    /// Resolves the previous value of the given number    
    /// * `system` - The `INumberSystem` reference
    /// * `value` - The input string `number`
    /// * `trim` - If set to `true` trims the input value before resolve
    /// `returns` The resolved previous `number`
    fn before(&self, value: &Number, trim: bool) -> Number;
    /// Because an odd base is being used, to get the mid position for any number
    /// the equation q = (nf + n0) / 2, where:
    /// - `n0` represents the start string converted from the nth base to the 10th base
    /// - `n1` represents the end string converted from the nth base to the 10th base
    /// * `before` - In the evaluation of two strings `n1 &lt; n2` must be `n1`
    /// * `after` - In the evaluation of two strings `n1 &lt; n2` must be `n2`
    /// * `round` - Rounds the result to the nearest upper integer if set to `true`
    /// `returns` The value q - ( q mod 2 )
    /// When q mod 2 is different from 0 then the mid character
    /// from the set of the nth base, should be placed at the end of the output.
    fn between(&self, before: &Number, after: &Number, round: bool) -> Number;
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
    fn diverge(&self, value1: &Number, value2: &Number) -> Number;
    /// Checks when a value is between two `string` values such that:
    /// `before` &lt; `value` &lt; `after`
    /// * `value` - The value that will be checked to be beween the `numbers`
    /// * `before` - The lower bound `number`
    /// * `after` - The upper bound `number`
    /// `returns` the cleaned number
    fn clean(value: &Number, before: &Number, after: &Number) -> Option<Number>;
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
        number1: &mut Number,
        number2: &mut Number,
        repeated: &mut Number,
        mode: HalfMode,
    );
    /// Evaluates two values to check which one goes before, gets the repeated values and
    /// after and then transforms them to code.
    /// * `before_codes` - The before codes
    /// * `after_codes` - The after codes
    /// * `round` - Defines if the value should be rounded upwards
    fn half(&self, before_codes: &Number, after_codes: &Number, round: bool) -> Number;
    /// If the number has overflows it will resolve them
    fn resolve_overflow(&self, number: &mut Number);
}
