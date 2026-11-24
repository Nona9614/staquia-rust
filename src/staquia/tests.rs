//! The port of the JavaScript `tests/specs/integration-tests/composer.test.ts` (4 tests) and
//! `tests/specs/integration-tests/forger.test.ts` (9 tests), which test the internal functions.

use super::Staquia;
use super::forger::raw;
use crate::error::PositionError;
use crate::number::Digits;
use crate::position::Position;
use crate::test_support::{alpha, letter, text};

/// The JavaScript `AlphabeticalSystem` mock: ln = 4, lz = 12, lmax = 16, lϰ = 14.
fn system() -> Staquia {
    Staquia::create('!', 'a', 'z', 4, 12, 0.15).expect("valid system")
}

/// The letters of digits, the most significant first.
fn letters(digits: &[u16]) -> String {
    digits
        .iter()
        .map(|digit| letter(i32::from(*digit)))
        .collect()
}

/// A position built from its sections, without the checks of a system (JavaScript `new Position()`).
fn position(n: &str, z: &str) -> Position {
    let digits = |s: &str| {
        s.chars()
            .map(|ch| if ch == '!' { 0 } else { ch as u16 - 96 })
            .collect::<Vec<_>>()
    };
    Position::new(
        system().signature,
        digits(n).into_iter(),
        digits(z).into_iter(),
    )
}

/// Asserts the sections of a position.
fn assert_sections(position: &Position, n: &str, z: &str) {
    assert_eq!(position.n().len(), 4, "n.upperBound");
    assert_eq!(letters(position.n()), n, "position.n");
    assert_eq!(position.z().len(), z.len(), "z.upperBound");
    assert_eq!(letters(position.z()), z, "position.z");
}

/// The characters of a number with the system characters (JavaScript `reduce`).
fn reduce(system: &Staquia, number: &Digits) -> String {
    (number.lower..=number.upper)
        .rev()
        .map(|i| system.signature.character(number.get(i) as u16))
        .collect()
}

// Composer

/// Should transform the digits to their string values.
#[test]
fn reduce_short_digits() {
    let reduced = reduce(&system(), &alpha("ldix"));
    assert_eq!(reduced.chars().count(), 4, "length");
    assert_eq!(reduced, "ldix", "string");
}

/// Should transform the long digits to their string values.
#[test]
fn reduce_long_digits() {
    let reduced = reduce(&system(), &alpha("ldixjvep"));
    assert_eq!(reduced.chars().count(), 8, "length");
    assert_eq!(reduced, "ldixjvep", "string");
}

/// Should transorm a position to a number.
#[test]
fn raw_position_to_number() {
    let number = raw(&position("ozvx", "!klwqrbvdjeyh"));
    assert_eq!((number.lower, number.upper), (0, 16));
    assert_eq!(text(&number), "ozvx!klwqrbvdjeyh");
}

/// Should transorm a position to a number and extend it with zeroes. The Rust `raw` always uses the
/// position's own length (the only one the library uses), so the zeroes are added with `extend`.
#[test]
fn raw_position_to_number_extended() {
    let mut number = raw(&position("ozvx", "!klwqrbvdjeyh"));
    number.extend(4);
    assert_eq!((number.lower, number.upper), (0, 20));
    assert_eq!(text(&number), "ozvx!klwqrbvdjeyh!!!!");
}

// Forger

/// Should forge a position from a string smaller than ln.
#[test]
fn forge_from_small_string() {
    assert_sections(&system().forge("nux").unwrap(), "nux!", "!");
}

/// Should forge a position from a string bigger than ln.
#[test]
fn forge_from_bigger_string() {
    assert_sections(&system().forge("kofeqc").unwrap(), "kofe", "qc");
}

/// Should forge a position from a string with length ln.
#[test]
fn forge_n_length_string() {
    assert_sections(&system().forge("rjld").unwrap(), "rjld", "!");
}

/// Should transform a number shorter than N length.
#[test]
fn from_shorter_than_n_number() {
    assert_sections(&system().compose(&alpha("ozv")).unwrap(), "ozv!", "!");
}

/// Should transform a number equal to N length.
#[test]
fn from_equal_to_n_number() {
    assert_sections(&system().compose(&alpha("ozvx")).unwrap(), "ozvx", "!");
}

/// Should transform a number bigger than N length.
#[test]
fn from_bigger_than_n_number() {
    assert_sections(
        &system().compose(&alpha("ozvxklwqr")).unwrap(),
        "ozvx",
        "klwqr",
    );
}

/// Should mark the number as a threshold reached warning.
#[test]
fn from_dangerous_number() {
    let error = system().compose(&alpha("ozvx!klwqrbvdj")).unwrap_err();
    let PositionError::ThresholdReached {
        position,
        length,
        threshold,
    } = error
    else {
        panic!("{error}")
    };
    assert_eq!((length, threshold), (14, 14));
    assert_sections(&position, "ozvx", "!klwqrbvdj");
}

/// Should mark the number as an overflow, with the position that caused it.
#[test]
fn from_overflowed_number() {
    let error = system().compose(&alpha("ozvx!klwqrbvdjeyh")).unwrap_err();
    let PositionError::Overflow {
        position,
        length,
        limit,
    } = error
    else {
        panic!("{error}")
    };
    assert_eq!((length, limit), (17, 16));
    assert_sections(&position, "ozvx", "!klwqrbvdjeyh");
}

/// Should parse a number with lower bound.
#[test]
fn from_number_with_lower_bound() {
    let mut number = Digits::new();
    for (index, digit) in [(3, 1), (2, 2), (1, 3), (0, 4), (-1, 5), (-2, 6)] {
        number.set(index, digit);
    }
    number.upper = 3;
    number.lower = -2;
    assert_sections(&system().compose(&number).unwrap(), "abcd", "ef");
}

/// Should agree with stacking the positions one by one: `stack_room(count)` fails exactly when the
/// `count - 1` calls to `next` from `start` fail, with the same error.
#[test]
fn stack_room_agrees_with_next() {
    for (first, last, ln) in [
        ('a', 'a', 1),
        ('a', 'c', 1),
        ('a', 'c', 2),
        ('a', 'd', 2),
        ('a', 'z', 2),
        ('a', 'b', 3),
    ] {
        let system = Staquia::create('!', first, last, ln, 3, 0.0).expect("valid system");
        let mut room: usize = 0; // the positions that fit, stacked one by one
        let mut position = system.start().expect("start");
        let edge = loop {
            room += 1;
            match system.next(&position) {
                Ok(next) => position = next,
                Err(error) => break error,
            }
        };
        for count in [
            0,
            1,
            room.saturating_sub(1),
            room,
            room + 1,
            room + 10,
            usize::MAX,
        ] {
            let expected = if count <= room { Ok(()) } else { Err(edge) };
            assert_eq!(
                system.stack_room(count),
                expected,
                "{count} positions in a system from {first} to {last} with ln {ln} (room {room})"
            );
        }
    }
}
