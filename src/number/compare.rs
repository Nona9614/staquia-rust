//! The order of two numbers (JavaScript `comparator`).

use core::cmp::Ordering;

use super::Digits;

impl Digits {
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
