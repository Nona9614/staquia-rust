//! The port of the JavaScript `tests/specs/unit-tests/number.test.ts` (30 tests).

use core::cmp::Ordering;

use super::Digits;
use crate::test_support::{alpha, text};

/// `{0: 1, 1: 1, 2: 1}` (JavaScript `JsonDigits.from`).
fn ones() -> Digits {
    let mut number = Digits::new();
    for i in 0..3 {
        number.set(i, 1);
    }
    number.upper = 2;
    number
}

fn greater(a: &str, b: &str) -> bool {
    alpha(a).compare(&alpha(b)) == Ordering::Greater
}
fn greater_or_equal(a: &str, b: &str) -> bool {
    alpha(a).compare(&alpha(b)) != Ordering::Less
}
fn less(a: &str, b: &str) -> bool {
    alpha(a).compare(&alpha(b)) == Ordering::Less
}
fn less_or_equal(a: &str, b: &str) -> bool {
    alpha(a).compare(&alpha(b)) != Ordering::Greater
}
fn equal(a: &str, b: &str) -> bool {
    alpha(a).compare(&alpha(b)) == Ordering::Equal
}

/// Should clone all the digits.
#[test]
fn clone() {
    let sut = ones();
    let copy = sut.duplicate();
    assert_eq!((sut.lower, sut.upper), (0, 2));
    assert_eq!((copy.lower, copy.upper), (0, 2));
    assert_eq!([copy.get(0), copy.get(1), copy.get(2)], [1, 1, 1]);
}

/// Should go pop the last digit.
#[test]
fn pop() {
    let mut sut = ones();
    sut.pop();
    assert_eq!((sut.lower, sut.upper), (0, 1));
    assert_eq!([sut.get(0), sut.get(1), sut.get(2)], [1, 1, 0]);
}

/// Should extend the number with 2 zeroes to the left.
#[test]
fn expand() {
    let mut abz = alpha("abz");
    abz.expand(2);
    assert_eq!(text(&abz), "!!abz");
}

/// Should extend the number with 2 zeroes to the right.
#[test]
fn extend() {
    let mut abz = alpha("abz");
    abz.extend(2);
    assert_eq!(text(&abz), "abz!!");
}

/// Should stack the passed number to the top of the caller.
#[test]
fn stack() {
    let top = alpha("nnnn");
    let mut number = alpha("zzzz");
    number.stack(&top);
    assert_eq!(text(&number), "nnnnzzzz");
}

/// Should remove all leading zeroes.
#[test]
fn compact_number_000z000() {
    let mut number = alpha("!!!z!!!");
    number.compact();
    assert_eq!(text(&number), "z!!!");
}

/// Should remove all leading zeroes.
#[test]
fn compact_number_000000() {
    let mut number = alpha("!!!!!!");
    number.compact();
    assert_eq!(text(&number), "!");
}

/// Should return false when the left number is not greater than the right number.
#[test]
fn greater_than_left_a_right_b() {
    assert!(!greater("a", "b"));
}

/// Should return true when the number is greater than the false number.
#[test]
fn greater_than_left_b_right_a() {
    assert!(greater("b", "a"));
}

/// Should return true when the long number is greater than the right long number.
#[test]
fn greater_than_left_cba_right_abc() {
    assert!(greater("cba", "abc"));
}

/// Should return true when the number with repeated digits is greater than the right number.
#[test]
fn greater_than_left_xyz_right_xyy() {
    assert!(greater("xyz", "xyy"));
}

/// Should return false when the left number is not greater or equal than the right number.
#[test]
fn greater_or_equal_than_left_a_right_b() {
    assert!(!greater_or_equal("a", "b"));
}

/// Should return true when the number is greater or equal than the false number.
#[test]
fn greater_or_equal_than_left_b_right_a() {
    assert!(greater_or_equal("b", "a"));
}

/// Should return true when the numbers are equal but they may contain leading zeroes.
#[test]
fn greater_or_equal_than_left_00n_right_0n() {
    assert!(greater_or_equal("!!n", "!n"));
}

/// Should return true when the long number is greater or equal than the right long number.
#[test]
fn greater_or_equal_than_left_cba_right_abc() {
    assert!(greater_or_equal("cba", "abc"));
}

/// Should return false when the left number is not less than the right number.
#[test]
fn less_than_left_b_right_a() {
    assert!(!less("b", "a"));
}

/// Should return true when the number is less than the false number.
#[test]
fn less_than_left_a_right_b() {
    assert!(less("a", "b"));
}

/// Should return true when the long number is less than the right long number.
#[test]
fn less_than_left_abc_right_cba() {
    assert!(less("abc", "cba"));
}

/// Should return true when the number with repeated digits is less than the right number.
#[test]
fn less_than_left_xyy_right_xyzr() {
    assert!(less("xyy", "xyzr"));
}

/// Should return false when the left number is not less or equal than the right number.
#[test]
fn less_or_equal_than_left_b_right_a() {
    assert!(!less_or_equal("b", "a"));
}

/// Should return true when the number is less or equal than the false number.
#[test]
fn less_or_equal_than_left_a_right_b() {
    assert!(less_or_equal("a", "b"));
}

/// Should return true when the numbers are equal but they may contain leading zeroes.
#[test]
fn less_or_equal_than_left_n_right_00n() {
    assert!(less_or_equal("n", "!!!n"));
}

/// Should return true when the long number is less or equal than the right long number.
#[test]
fn less_or_equal_than_left_abc_right_cba() {
    assert!(less_or_equal("abc", "cba"));
}

/// Should return true when the number with repeated digits is less or equal than the right number.
#[test]
fn less_or_equal_than_left_xyy_right_xyzr() {
    assert!(less_or_equal("xyy", "xyzr"));
}

/// Should return false when the left number is not equal to the right number.
#[test]
fn equal_to_left_a_right_b() {
    assert!(!equal("a", "b"));
}

/// Should return true when the number is equal to the false number.
#[test]
fn equal_to_left_n_right_n() {
    assert!(equal("n", "n"));
}

/// Should return true when the long number is equal to the right long number.
#[test]
fn equal_to_left_xyz_right_xyz() {
    assert!(equal("xyz", "xyz"));
}

/// Should return 'greater' when the left value is bigger than the right value.
#[test]
fn compare_left_tr0xu0_right_tr0kzn() {
    assert_eq!(alpha("tr!xu!").compare(&alpha("tr!kzn")), Ordering::Greater);
}

/// Should return 'equal' when the left value is equal to the right value.
#[test]
fn compare_left_0ufad_right_ufad() {
    assert_eq!(alpha("!ufad").compare(&alpha("ufad")), Ordering::Equal);
}

/// Should return 'less' when the left value is smaller than the right value.
#[test]
fn compare_left_plkg_right_plkga() {
    assert_eq!(alpha("plkg").compare(&alpha("plkga")), Ordering::Less);
}
