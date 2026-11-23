//! The internal numbers of the algorithms.
//!
//! A port of the JavaScript `Digits` class: a number of any base stored by the index of each digit
//! (`0` is the least significant, negative indexes are fractional digits) with a `lower` and an `upper`
//! bound. The JavaScript version is a sparse `Map`; here it is a fixed array on the stack where a missing
//! digit is `0`, so every algorithm works without allocating.
//!
//! The methods keep the exact semantics of the JavaScript ones, including their quirks (for example
//! `extend` always shifts from the index `0`), because the positions must be identical in both languages.

use core::cmp::Ordering;

/// The number of slots, enough for the indexes used by the algorithms with positions of up to
/// `MAX_LENGTH` digits (from about `-MAX_LENGTH - 1` to `2 * MAX_LENGTH + 2`).
const CAPACITY: usize = 128;
/// The slot of the index `0`.
const OFFSET: i32 = 48;

/// A number of any base, see the module documentation.
pub(crate) struct Digits {
    map: [i32; CAPACITY],
    /// The index of the least significant digit.
    pub lower: i32,
    /// The index of the most significant digit.
    pub upper: i32,
}

impl Digits {
    /// An empty number: no digits and both bounds at `0` (JavaScript `create`).
    pub fn new() -> Self {
        Self { map: [0; CAPACITY], lower: 0, upper: 0 }
    }

    /// Creates the number with these digits, the most significant first, at the indexes
    /// `digits.len() - 1` down to `0`.
    pub fn from_slice(digits: &[u16]) -> Self {
        let mut number = Self::new();
        let length = digits.len() as i32;
        number.upper = length - 1;
        for (i, digit) in digits.iter().enumerate() {
            number.set(length - 1 - i as i32, i32::from(*digit));
        }
        number
    }

    #[inline]
    fn slot(key: i32) -> Option<usize> {
        let slot = key + OFFSET;
        if (0..CAPACITY as i32).contains(&slot) { Some(slot as usize) } else { None }
    }

    /// The digit at `key`, `0` if it was never set.
    #[inline]
    pub fn get(&self, key: i32) -> i32 {
        match Self::slot(key) {
            Some(slot) => self.map[slot],
            None => 0,
        }
    }

    /// Sets the digit at `key`. The bounds are not changed.
    #[inline]
    pub fn set(&mut self, key: i32, value: i32) {
        match Self::slot(key) {
            Some(slot) => self.map[slot] = value,
            // The capacity covers every index the algorithms reach with valid systems
            None => debug_assert!(value == 0, "digit index {key} out of the capacity"),
        }
    }

    /// Adds `offset` to the digit at `key`.
    #[inline]
    pub fn update(&mut self, key: i32, offset: i32) {
        self.set(key, self.get(key) + offset);
    }

    /// Removes the digit at `key` (it becomes `0`).
    #[inline]
    pub fn remove(&mut self, key: i32) {
        self.set(key, 0);
    }

    /// A copy with only the digits inside the bounds (JavaScript `clone`).
    pub fn duplicate(&self) -> Self {
        let mut number = Self::new();
        for i in self.lower..=self.upper {
            number.set(i, self.get(i));
        }
        number.lower = self.lower;
        number.upper = self.upper;
        number
    }

    /// A copy of every digit, also the ones outside the bounds (JavaScript `reference`).
    fn take(&self) -> Self {
        Self { map: self.map, lower: self.lower, upper: self.upper }
    }

    /// Removes the most significant digit.
    pub fn pop(&mut self) {
        self.remove(self.upper);
        self.upper -= 1;
    }

    /// Removes the zeroes at the left; a number of only zeroes becomes the single digit `0`.
    pub fn compact(&mut self) {
        let mut upper = self.upper;
        let mut n = self.upper;
        while n >= self.lower {
            if self.get(n) == 0 {
                self.remove(n);
                upper -= 1;
            } else {
                break;
            }
            n -= 1;
        }
        if upper < 0 {
            self.lower = 0;
            upper = 0;
            self.set(0, 0);
        }
        self.upper = upper;
    }

    /// Adds `size` zeroes at the left.
    pub fn expand(&mut self, size: i32) {
        let upper = self.upper;
        self.upper = upper + size;
        for i in upper + 1..size + upper + 1 {
            self.set(i, 0);
        }
    }

    /// Shifts the digits from the index `0` by `size` and fills the gap with zeroes.
    pub fn extend(&mut self, size: i32) {
        let upper = self.upper;
        self.upper = size + upper;
        let mut n = upper;
        while n >= 0 {
            self.set(n + size, self.get(n));
            n -= 1;
        }
        for n in 0..size {
            self.set(n, 0);
        }
    }

    /// Writes the digits of `other` at the left of this number.
    pub fn stack(&mut self, other: &Digits) {
        let mut upper = self.upper;
        for i in other.lower..=other.upper {
            upper += 1;
            self.set(upper, other.get(i));
        }
        self.upper = upper;
    }

    /// Compares two numbers aligned by their index `0`, missing digits are zeroes.
    pub fn compare(&self, right: &Digits) -> Ordering {
        let left = self;
        let min_upper = left.upper.min(right.upper);
        let max_lower = left.lower.max(right.lower);
        if left.upper != right.upper {
            let mut n = left.upper.max(right.upper);
            while n > min_upper {
                let l = if n <= left.upper { left.get(n) } else { 0 };
                let r = if n <= right.upper { right.get(n) } else { 0 };
                if l != r {
                    return l.cmp(&r);
                }
                n -= 1;
            }
        }
        let mut n = min_upper;
        while n >= max_lower {
            let (l, r) = (left.get(n), right.get(n));
            if l != r {
                return l.cmp(&r);
            }
            n -= 1;
        }
        if left.lower != right.lower {
            let lowest = left.lower.min(right.lower);
            let mut n = max_lower - 1;
            while n >= lowest {
                let l = if n >= left.lower { left.get(n) } else { 0 };
                let r = if n >= right.lower { right.get(n) } else { 0 };
                if l != r {
                    return l.cmp(&r);
                }
                n -= 1;
            }
        }
        Ordering::Equal
    }
}

/// How `compact_numbers` aligns two numbers of different lengths.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum HalfMode {
    /// Adds zeroes at the left of the shorter number (integers).
    Between,
    /// Adds zeroes at the right of the shorter number (fractions).
    Diverge,
}

/// The arithmetic of a system of base `base`, a port of the JavaScript algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Algorithms {
    pub base: i32,
}

impl Algorithms {
    /// Moves to the next index every digit equal to or bigger than the base.
    pub fn resolve_overflow(&self, number: &mut Digits) {
        let mut n = number.lower;
        while n <= number.upper {
            if number.get(n) >= self.base {
                number.update(n, -self.base);
                number.update(n + 1, 1);
            }
            n += 1;
        }
    }

    /// `value + 1`.
    pub fn after(&self, value: &Digits, trim: bool) -> Digits {
        let mut number = value.duplicate();
        number.update(0, 1);
        if trim {
            number.compact();
        }
        number.expand(1);
        self.resolve_overflow(&mut number);
        if number.get(number.upper) == 0 {
            number.pop();
        }
        number
    }

    /// `value - 1`.
    pub fn before(&self, value: &Digits, trim: bool) -> Digits {
        let mut number = value.duplicate();
        number.update(0, -1);
        if trim {
            number.compact();
        }
        number.expand(1);
        let mut n = number.lower;
        while n <= number.upper {
            if number.get(n) < 0 {
                number.update(n, self.base);
                number.update(n + 1, -1);
            } else {
                break;
            }
            n += 1;
        }
        if number.get(number.upper) <= 0 {
            number.pop();
        }
        number
    }

    /// `left - right`, as its magnitude and whether it is negative.
    pub fn substract(&self, left: &Digits, right: &Digits) -> (Digits, bool) {
        let mut result = Digits::new();
        let mut negative = false;
        let mut left = left.duplicate();
        let mut x = right.lower;
        while x <= right.upper {
            let mut value = left.get(x) - right.get(x);
            if value < 0 {
                left.update(x + 1, -1);
                value += self.base;
            }
            result.set(x, value);
            x += 1;
        }
        result.upper = x - 1;
        if left.get(x) == -1 {
            result.upper = x;
            negative = true;
            // The complement: `base^x - result`
            let mut restore = Digits::new();
            restore.set(x, 1);
            for i in 0..=x {
                let mut value = restore.get(i) - result.get(i);
                if value < 0 {
                    restore.update(i + 1, -1);
                    value += self.base;
                }
                result.set(i, value);
            }
        }
        result.compact();
        (result, negative)
    }

    /// Aligns both numbers, orders them (`number1 <= number2`) and moves their common most
    /// significant digits to `repeated`.
    pub fn compact_numbers(&self, number1: &mut Digits, number2: &mut Digits, repeated: &mut Digits, mode: HalfMode) {
        let diff = number1.upper - number2.upper;
        match mode {
            HalfMode::Between => {
                if diff < 0 {
                    number1.expand(-diff);
                } else if diff > 0 {
                    number2.expand(diff);
                }
            }
            HalfMode::Diverge => {
                if diff < 0 {
                    number1.extend(-diff);
                } else if diff > 0 {
                    number2.extend(diff);
                }
            }
        }
        if number1.compare(number2) == Ordering::Greater {
            let temp = number1.duplicate();
            *number1 = number2.take();
            *number2 = temp;
        }
        // The common digits, from the most significant
        let mut codes = [0i32; CAPACITY];
        let mut count = 0usize;
        let mut index = number1.upper;
        while index > -1 {
            let (code1, code2) = (number1.get(index), number2.get(index));
            if code1 == code2 {
                number1.pop();
                number2.pop();
                codes[count] = code1;
                count += 1;
                index -= 1;
            } else {
                break;
            }
        }
        repeated.upper = count as i32 - 1;
        let mut ix = repeated.upper;
        for code in &codes[..count] {
            repeated.set(ix, *code);
            ix -= 1;
        }
    }

    /// The middle of two aligned numbers, with one fractional digit (index `-1`) when the
    /// half is not exact, unless it can be rounded.
    pub fn half(&self, before: &Digits, after: &Digits, round: bool) -> Digits {
        let mut value = Digits::new();
        value.lower = -1;
        value.upper = before.upper;
        // The JavaScript version works with halves (`x.5`), these are doubled to stay integers
        let mut trailing2 = 0;
        let mut i = value.upper;
        while i >= value.lower {
            let q2 = after.get(i) + before.get(i) + trailing2;
            let m2 = q2.rem_euclid(2);
            value.set(i, (q2 - m2) / 2);
            trailing2 = m2 * self.base;
            i -= 1;
        }
        let has_decimals = value.get(-1) != 0;
        let (substraction, negative) = self.substract(after, before);
        let roundable = !negative && !(substraction.upper == 0 && substraction.get(0) < 3);
        if !has_decimals {
            value.remove(-1);
            value.lower = 0;
        }
        if has_decimals && round && roundable {
            value.remove(-1);
            value.lower = 0;
            value.update(0, 1);
        }
        self.resolve_overflow(&mut value);
        value
    }

    /// The middle of two integers.
    pub fn between(&self, before: &Digits, after: &Digits, round: bool) -> Digits {
        let mut before = before.duplicate();
        let mut after = after.duplicate();
        let mut repeated = Digits::new();
        self.compact_numbers(&mut before, &mut after, &mut repeated, HalfMode::Between);
        let mut value = self.half(&before, &after, round);
        value.stack(&repeated);
        value
    }

    /// The middle of two fractions, as short as possible.
    pub fn diverge(&self, value1: &Digits, value2: &Digits) -> Digits {
        let mut before = value1.duplicate();
        let mut after = value2.duplicate();
        let mut repeated = Digits::new();
        self.compact_numbers(&mut before, &mut after, &mut repeated, HalfMode::Diverge);
        let value = self.half(&before, &after, true);
        let mut value = clean(value, &before, &after);
        value.stack(&repeated);
        value
    }
}

/// Removes the least significant digits of `value` that are not needed to keep it
/// between `before` and `after`.
pub(crate) fn clean(value: Digits, before: &Digits, after: &Digits) -> Digits {
    let mut n = value.upper;
    let br = value.lower - 1;
    let mut before_index = br;
    let mut after_index = br;
    while before_index == br && after_index == br && n >= value.lower {
        let (b, v, a) = (before.get(n), value.get(n), after.get(n));
        if b < v {
            before_index = n;
        }
        if v < a {
            after_index = n;
        }
        if before_index != br || after_index != br {
            break;
        }
        n -= 1;
    }
    if before_index == br && after_index == br {
        return value;
    }
    if after_index == br {
        while after_index == br && n >= value.lower {
            if value.get(n) < after.get(n) {
                after_index = n;
                break;
            }
            n -= 1;
        }
        if after_index == br {
            return value;
        }
    } else {
        while before_index == br && n >= value.lower {
            if before.get(n) < value.get(n) {
                before_index = n;
                break;
            }
            n -= 1;
        }
        if before_index == br {
            return value;
        }
    }
    let mut number = Digits::new();
    number.upper = value.upper - n + value.lower;
    number.lower = value.lower;
    let mut i = value.upper;
    while i >= n {
        number.set(i - n + value.lower, value.get(i));
        i -= 1;
    }
    number
}
