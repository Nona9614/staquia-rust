//! The port of the JavaScript `tests/specs/unit-tests/algorithms.test.ts` (1 test) and
//! `tests/specs/integration-tests/algorithms.test.ts` (27 tests).

use super::{after, before, between, clean, diverge, substract};
use crate::number::Digits;
use crate::test_support::{BASE, alpha, text};

/// Asserts the bounds and the letters of a result.
fn assert_number(result: &Digits, lower: i32, upper: i32, expected: &str) {
    assert_eq!(result.lower, lower, "lowerBound");
    assert_eq!(result.upper, upper, "upperBound");
    assert_eq!(text(result), expected, "number");
}

/// Unit test: should go one value after. The JavaScript version mocks the number; here the last digit
/// carries to a new one, the upper bound the mock expected.
#[test]
fn after_test() {
    let result = after(BASE, &alpha("z"), false);
    assert_eq!(result.upper, 1);
    assert_eq!(text(&result), "a!");
}

/// Should go from 'aaa' to 'aab'.
#[test]
fn after_number_aaa() {
    assert_number(&after(BASE, &alpha("aaa"), false), 0, 2, "aab");
}

/// Should go (base * n + 1) place after when starts with the last character in set.
#[test]
fn after_number_az() {
    assert_number(&after(BASE, &alpha("az"), false), 0, 1, "b!");
}

/// Should resolve multiple overflows.
#[test]
fn after_number_azzz() {
    assert_number(&after(BASE, &alpha("azzz"), false), 0, 3, "b!!!");
}

/// Should go one value before.
#[test]
fn before_number_z() {
    assert_number(&before(BASE, &alpha("z"), false), 0, 0, "y");
}

/// Should go (base * n - 1) place before when starts with 'zero'.
#[test]
fn before_number_z0() {
    assert_number(&before(BASE, &alpha("z!"), false), 0, 1, "yz");
}

/// Should resolve values that may cause overflow after substraction.
#[test]
fn between_number_yz_and_number_za() {
    assert_number(
        &between(BASE, &alpha("yz"), &alpha("za"), false),
        0,
        1,
        "z!",
    );
}

/// Should resolve hidden leading zero plus values that causes carry on substraction.
#[test]
fn between_number_akz_and_number_abyn() {
    assert_number(
        &between(BASE, &alpha("akz"), &alpha("abyn"), false),
        0,
        3,
        "!ort",
    );
}

/// Should evaluate considering leading zeroes and resolve no room.
#[test]
fn between_number_00abz_and_number_abzaa() {
    assert_number(
        &between(BASE, &alpha("!!abz"), &alpha("abzaa"), false),
        0,
        4,
        "!o!b!",
    );
}

/// Should resolve a coersive no room case and add missing leading zeroes.
#[test]
fn between_number_abyzz_and_number_abz_round() {
    assert_number(
        &between(BASE, &alpha("abyzz"), &alpha("abz"), true),
        0,
        4,
        "!o!am",
    );
}

/// Should resolve hidden leading zero.
#[test]
fn between_number_r_and_number_ri() {
    assert_number(
        &between(BASE, &alpha("r"), &alpha("ri"), false),
        -1,
        1,
        "imm",
    );
}

/// Should get the exact middle point from the system.
#[test]
fn between_number_a_and_number_z() {
    assert_number(&between(BASE, &alpha("a"), &alpha("z"), false), -1, 0, "mm");
}

/// Should get the middle value from the system rounded.
#[test]
fn between_number_a_and_number_z_round() {
    assert_number(&between(BASE, &alpha("a"), &alpha("z"), true), 0, 0, "n");
}

/// Should get the middle value from the system rounded.
#[test]
fn between_number_a_and_number_c_round() {
    assert_number(&between(BASE, &alpha("a"), &alpha("c"), true), 0, 0, "b");
}

/// Should handle complex numbers.
#[test]
fn diverge_number_0xca_and_number_0xaa() {
    assert_number(&diverge(BASE, &alpha("!xca"), &alpha("!xaa")), 0, 2, "!xb");
}

/// Should avoid overlap when there is no room and is in the limit of the system.
#[test]
fn diverge_number_yz_and_number_zzzy() {
    assert_number(&diverge(BASE, &alpha("yz"), &alpha("zzzy")), 0, 1, "zl");
}

/// Should get the middle character ignoring the offset.
#[test]
fn diverge_number_a_and_number_zz() {
    assert_number(&diverge(BASE, &alpha("a"), &alpha("zz")), 0, 0, "n");
}

/// Should ignore and concat again leading zeroes.
#[test]
fn diverge_number_00a_and_number_00z() {
    assert_number(&diverge(BASE, &alpha("!!a"), &alpha("!!z")), 0, 2, "!!n");
}

/// Should extend itself when there is no room.
#[test]
fn diverge_number_az_and_number_b0() {
    assert_number(&diverge(BASE, &alpha("az"), &alpha("b!")), -1, 1, "azm");
}

/// Should order the values before evaluation.
#[test]
fn diverge_number_ng_and_number_n() {
    assert_number(&diverge(BASE, &alpha("ng"), &alpha("n")), 0, 1, "nd");
}

/// Should ignore repeated values then concatenate them again, work against zero space and ignore extra values.
#[test]
fn diverge_number_abba_and_number_abccc() {
    assert_number(
        &diverge(BASE, &alpha("abba"), &alpha("abccc")),
        0,
        3,
        "abbo",
    );
}

/// Should ignore repeated values then concatenate them again and extends because zero space.
#[test]
fn diverge_number_nnnn_and_number_nnno() {
    assert_number(
        &diverge(BASE, &alpha("nnnn"), &alpha("nnno")),
        -1,
        3,
        "nnnnm",
    );
}

/// Should get the value before the middle value from the system.
#[test]
fn diverge_number_0_and_number_z() {
    assert_number(&diverge(BASE, &alpha("!"), &alpha("z")), 0, 0, "m");
}

/// Should get the middle value from the system.
#[test]
fn diverge_number_a_and_number_z() {
    assert_number(&diverge(BASE, &alpha("a"), &alpha("z")), 0, 0, "n");
}

/// Should clean the extra 'zz' values.
#[test]
fn clean_before_yz_value_zlzz_after_zzzy() {
    assert_number(
        &clean(alpha("zlzz"), &alpha("yz"), &alpha("zzzy")),
        0,
        1,
        "zl",
    );
}

/// Should clean the extra 'A' values.
#[test]
fn clean_before_0xaa_value_0xba_after_0xca() {
    assert_number(
        &clean(alpha("!xba"), &alpha("!xaa"), &alpha("!xca")),
        0,
        2,
        "!xb",
    );
}

/// Should not clean the value as there is no space.
#[test]
fn clean_before_az_value_azm_after_b0() {
    assert_number(
        &clean(alpha("azm"), &alpha("az"), &alpha("b!")),
        0,
        2,
        "azm",
    );
}

/// Should resolve when the left operand is smaller and result in 1.
#[test]
fn substract_mmmm_from_mmmn() {
    let (result, negative) = substract(BASE, &alpha("mmmm"), &alpha("mmmn"));
    assert!(negative);
    assert_number(&result, 0, 0, "a");
}
