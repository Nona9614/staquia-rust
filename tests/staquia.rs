//! The port of the JavaScript `tests/specs/integration-tests/staquia.spec.ts` (34 tests): the public API of
//! `Staquia`. The tests keep the names of the JavaScript methods.
//!
//! The library contract is that positions sorted as plain strings keep the same order as the list, so every
//! generated position must respect the string order and the `Ord` of `Position`.

use staquia::{Position, PositionError, Staquia};

/// ln = 4, lz = 12 → lmax = 16, fϰ = 0.15 → lϰ = 14.
fn system() -> Staquia {
    Staquia::create('!', 'a', 'z', 4, 12, 0.15).expect("valid system")
}

fn forge(string: &str) -> Position {
    system().forge(string).expect("valid position")
}

/// `left` is strictly before `right` both as strings (what a database sorts) and as positions.
fn assert_before(left: &Position, right: &Position) {
    let (l, r) = (left.to_string(), right.to_string());
    assert!(l < r, "The position string '{l}' must sort before '{r}'.");
    assert!(
        left < right,
        "The position '{l}' must compare as less than '{r}'."
    );
    assert!(
        right > left,
        "The position '{r}' must compare as greater than '{l}'."
    );
}

/// The result failed with an overflow.
fn assert_overflow(result: Result<Position, PositionError>, label: &str) {
    match result {
        Ok(position) => panic!("{label} must fail with an overflow but returned '{position}'."),
        Err(error) => assert!(
            error.is_overflow(),
            "{label} must fail with an overflow but got '{error}'."
        ),
    }
}

/// The result failed because of a position from another system.
fn assert_mismatch(result: Result<Position, PositionError>, label: &str) {
    match result {
        Ok(position) => {
            panic!("{label} must fail with a system mismatch but returned '{position}'.")
        }
        Err(error) => assert!(
            error.is_mismatch(),
            "{label} must fail with a system mismatch but got '{error}'."
        ),
    }
}

/// The result failed because of a character that is not a digit.
fn assert_invalid(string: &str, index: usize) {
    match system().forge(string) {
        Ok(position) => panic!("forge('{string}') must fail but returned '{position}'."),
        Err(PositionError::InvalidCharacter { index: actual, .. }) => {
            assert_eq!(
                actual, index,
                "forge('{string}') must fail at the index {index}."
            );
        }
        Err(error) => {
            panic!("forge('{string}') must fail with an invalid character but got '{error}'.")
        }
    }
}

/// The system positions use only its characters and round-trip through `forge`.
fn assert_system(system: &Staquia, characters: &str, label: &str) {
    let start = system.start().unwrap();
    let next = system.next(&start).unwrap();
    for position in [start, next, system.core(&start, &next).unwrap()] {
        let string = position.to_string();
        assert!(
            string.chars().all(|ch| characters.contains(ch)),
            "{label}: the position '{string}' uses characters from another system."
        );
        let forged = system.forge(&string).unwrap();
        assert_eq!(
            forged.to_string(),
            string,
            "{label}: '{string}' does not round-trip."
        );
        assert_eq!(forged, position, "{label}: '{string}' does not round-trip.");
    }
}

/// Narrows the gap between two adjacent positions from one side until the threshold.
fn narrow_until_threshold(from_left: bool) {
    let system = system();
    let mut left = system.start().unwrap();
    let mut right = system.next(&left).unwrap();
    for i in 0..500 {
        match system.core(&left, &right) {
            Ok(core) => {
                assert_before(&left, &core);
                assert_before(&core, &right);
                if from_left { left = core } else { right = core }
            }
            Err(error) => {
                assert!(
                    error.is_dangerous() || error.is_overflow(),
                    "The narrowing must stop at the threshold, got '{error}'."
                );
                assert!(
                    i > 0,
                    "The system must allow at least one insertion between adjacent positions."
                );
                return;
            }
        }
    }
    panic!("The gap was narrowed 500 times without reaching the threshold.");
}

const ALPHABET: &str = "!abcdefghijklmnopqrstuvwxyz";
const DIGITS: &str = "#0123456789";

// Forge

/// Should pad a 3 character string to the right with zeroes.
#[test]
fn staquia_forge_pads_three_characters() {
    assert_eq!(forge("abc").to_string(), "abc!!");
}

/// Should pad a 2 character string to the right with zeroes.
#[test]
fn staquia_forge_pads_two_characters() {
    assert_eq!(forge("xy").to_string(), "xy!!!");
}

/// Should pad a single character string to the right with zeroes.
#[test]
fn staquia_forge_pads_one_character() {
    assert_eq!(forge("b").to_string(), "b!!!!");
}

/// Should keep the string order of the forged strings.
#[test]
fn staquia_forge_keeps_order() {
    let strings = ["abc", "abcs", "b", "bbbb", "mmmm!m", "mmmma", "xy", "zzzz"];
    let positions: Vec<Position> = strings.iter().map(|s| forge(s)).collect();
    for pair in positions.windows(2) {
        assert_before(&pair[0], &pair[1]);
    }
}

/// Should forge back the same position from its string representation.
#[test]
fn staquia_forge_round_trip() {
    let system = system();
    let start = system.start().unwrap();
    let next = system.next(&start).unwrap();
    for position in [
        start,
        next,
        system.previous(&start).unwrap(),
        system.core(&start, &next).unwrap(),
    ] {
        let forged = system.forge(&position.to_string()).unwrap();
        assert_eq!(forged.to_string(), position.to_string());
        assert_eq!(
            forged, position,
            "The forged '{forged}' must be equal to '{position}'."
        );
    }
}

/// Should reject characters that are not digits of the system.
#[test]
fn staquia_forge_rejects_foreign_characters() {
    assert_invalid("¼¼¼¼¿!", 0); // From another system
    assert_invalid("abcD", 3); // Uppercase is below 'a'
    assert_invalid("ab{c", 2); // Right after the last character 'z'
    assert_invalid("mmmm!m😀", 6); // Outside the Basic Multilingual Plane
}

/// Should reject characters between the zero and the first character.
#[test]
fn staquia_forge_rejects_characters_between_zero_and_first() {
    assert_invalid("a\"aa", 1);
    assert_invalid("aa`a", 2);
    assert_invalid("aaa0", 3);
}

/// Should accept the zero, the first and the last characters.
#[test]
fn staquia_forge_accepts_edge_characters() {
    assert_eq!(forge("!azz!a").to_string(), "!azz!a");
}

// Compare

/// Should compare positions with different z lengths by their string order.
#[test]
fn staquia_compare_different_z_lengths() {
    assert_before(&forge("mmmm!m"), &forge("mmmma"));
    assert_before(&forge("mmmm!"), &forge("mmmm!m"));
    assert_before(&forge("mmmma"), &forge("mmmmaa"));
    assert_before(&forge("mmmm!z"), &forge("mmmma!"));
}

/// Should consider trailing zeroes as equal positions.
#[test]
fn staquia_compare_trailing_zeroes() {
    assert_eq!(forge("mmmma"), forge("mmmma!!"));
}

// Next & previous

/// Should generate strictly increasing positions with next.
#[test]
fn staquia_next_is_increasing() {
    let system = system();
    let mut position = system.start().unwrap();
    for _ in 0..100 {
        let next = system.next(&position).unwrap();
        assert_before(&position, &next);
        position = next;
    }
}

/// Should generate strictly decreasing positions with previous.
#[test]
fn staquia_previous_is_decreasing() {
    let system = system();
    let mut position = system.start().unwrap();
    for _ in 0..100 {
        let previous = system.previous(&position).unwrap();
        assert_before(&previous, &position);
        position = previous;
    }
}

/// Should return to the same position when going next and then previous.
#[test]
fn staquia_next_previous_round_trip() {
    let system = system();
    let start = system.start().unwrap();
    assert_eq!(
        system.previous(&system.next(&start).unwrap()).unwrap(),
        start
    );
    assert_eq!(
        system.next(&system.previous(&start).unwrap()).unwrap(),
        start
    );
}

// N edges

/// Should keep the leading zeroes of N when going next.
#[test]
fn staquia_next_keeps_leading_zeroes() {
    let system = system();
    let next = |s: &str| system.next(&forge(s)).unwrap().to_string();
    assert_eq!(next("!!!a"), "!!!b!");
    assert_eq!(next("!!ab"), "!!ac!");
    assert_eq!(next("!!!z"), "!!a!!");
    assert_eq!(next("!abc"), "!abd!");
}

/// Should keep the leading zeroes of N when going previous.
#[test]
fn staquia_previous_keeps_leading_zeroes() {
    let system = system();
    let previous = |s: &str| system.previous(&forge(s)).unwrap().to_string();
    assert_eq!(previous("!!ab"), "!!aa!");
    assert_eq!(previous("!!a!"), "!!!z!");
    assert_eq!(previous("!!!b"), "!!!a!");
    assert_eq!(previous("a!!!"), "!zzz!");
}

/// Should reach the last and first values of N without errors.
#[test]
fn staquia_n_edges_reachable() {
    let system = system();
    assert_eq!(system.next(&forge("zzzy")).unwrap().to_string(), "zzzz!");
    assert_eq!(
        system.previous(&forge("!!!a")).unwrap().to_string(),
        "!!!!!"
    );
}

/// Should fail with an overflow when going next from the last value of N.
#[test]
fn staquia_next_overflow() {
    let system = system();
    assert_overflow(system.next(&forge("zzzz")), "next('zzzz')");
    assert_overflow(system.next(&forge("zzzzm")), "next('zzzzm')");
}

/// Should fail with an overflow when going previous from the first value of N.
#[test]
fn staquia_previous_overflow() {
    let system = system();
    assert_overflow(system.previous(&forge("!!!!")), "previous('!!!!')");
    assert_overflow(system.previous(&forge("!!!!a")), "previous('!!!!a')");
}

/// Should never wrap around when walking next until the end of N.
#[test]
fn staquia_next_walk_to_end() {
    let system = system();
    let mut position = forge("zzyr");
    for _ in 0..100 {
        match system.next(&position) {
            Ok(next) => {
                assert_before(&position, &next);
                position = next;
            }
            Err(error) => {
                assert!(
                    error.is_overflow(),
                    "next('{position}') must fail with an overflow."
                );
                assert_eq!(
                    position.to_string(),
                    "zzzz!",
                    "The overflow must happen only at the last value of N."
                );
                return;
            }
        }
    }
    panic!("The end of N was never reached.");
}

/// Should never wrap around when walking previous until the start of N.
#[test]
fn staquia_previous_walk_to_start() {
    let system = system();
    let mut position = forge("!!ah");
    for _ in 0..100 {
        match system.previous(&position) {
            Ok(previous) => {
                assert_before(&previous, &position);
                position = previous;
            }
            Err(error) => {
                assert!(
                    error.is_overflow(),
                    "previous('{position}') must fail with an overflow."
                );
                assert_eq!(
                    position.to_string(),
                    "!!!!!",
                    "The overflow must happen only at the first value of N."
                );
                return;
            }
        }
    }
    panic!("The start of N was never reached.");
}

// Core

/// Should generate a position strictly between two adjacent positions.
#[test]
fn staquia_core_between_adjacent() {
    let system = system();
    let start = system.start().unwrap();
    let next = system.next(&start).unwrap();
    let core = system.core(&start, &next).unwrap();
    assert_before(&start, &core);
    assert_before(&core, &next);
}

/// Should keep the order while narrowing a gap from the right until the threshold.
#[test]
fn staquia_core_narrow_from_right() {
    narrow_until_threshold(false);
}

/// Should keep the order while narrowing a gap from the left until the threshold.
#[test]
fn staquia_core_narrow_from_left() {
    narrow_until_threshold(true);
}

/// Should return the error of a failed result passed as an argument. Rust passes positions, not results:
/// chaining with `and_then` (or `?`) gives back the same error.
#[test]
fn staquia_core_failed_argument() {
    let system = system();
    let start = system.start().unwrap();
    let failed = system.next(&forge("zzzz"));
    let error = failed.unwrap_err();
    let as_next = failed.and_then(|next| system.core(&start, &next));
    assert_eq!(
        as_next,
        Err(error),
        "A failed next argument must fail the core."
    );
    let as_previous = failed.and_then(|previous| system.core(&previous, &start));
    assert_eq!(
        as_previous,
        Err(error),
        "A failed previous argument must fail the core."
    );
}

/// Should generate the same position regardless of the arguments order.
#[test]
fn staquia_core_argument_order() {
    let system = system();
    let start = system.start().unwrap();
    let next = system.next(&system.next(&start).unwrap()).unwrap();
    assert_eq!(
        system.core(&start, &next).unwrap().to_string(),
        system.core(&next, &start).unwrap().to_string()
    );
}

// Systems

/// Should print each system positions with its own characters.
#[test]
fn staquia_systems_independent() {
    let alphabet = Staquia::create('!', 'a', 'z', 4, 12, 0.15).unwrap();
    let digits = Staquia::create('#', '0', '9', 4, 12, 0.15).unwrap();
    let before = alphabet.start().unwrap().to_string();
    assert_system(&alphabet, ALPHABET, "alphabet");
    assert_system(&digits, DIGITS, "digits");
    // Creating more systems after must not change the previous ones
    Staquia::create('~', 'A', 'Z', 3, 8, 0.15).unwrap_err();
    let _ = Staquia::default();
    assert_system(&alphabet, ALPHABET, "alphabet");
    assert_system(&digits, DIGITS, "digits");
    assert_eq!(alphabet.start().unwrap().to_string(), before);
}

/// Should keep printing an existing position the same after creating other systems.
#[test]
fn staquia_systems_existing_position() {
    let alphabet = Staquia::create('!', 'a', 'z', 4, 12, 0.15).unwrap();
    let position = alphabet.forge("abcd").unwrap();
    let _ = Staquia::default();
    assert_eq!(position.to_string(), "abcd!");
    assert_eq!(format!("{position}"), "abcd!");
}

/// Should not modify the shared Position class. In Rust a position prints with the signature it carries,
/// so positions of different systems are the same type and never affect each other.
#[test]
fn staquia_systems_base_untouched() {
    let alphabet = Staquia::create('!', 'a', 'z', 4, 12, 0.15).unwrap();
    let digits = Staquia::create('#', '0', '9', 4, 12, 0.15).unwrap();
    let positions: [Position; 2] = [alphabet.start().unwrap(), digits.start().unwrap()];
    assert_eq!(positions.map(|p| p.to_string()), ["mmmm!", "4444#"]);
}

/// Should create positions that are instances of Position. In Rust: plain `Position` values, printed with
/// `Display` (the string is not a stored field).
#[test]
fn staquia_systems_instance_of() {
    let position: Position = system().start().unwrap();
    assert_eq!(format!("{position:?}"), "Position(\"mmmm!\")");
    assert_eq!(position.len(), 5);
}

/// Should support a custom position class shared by many systems without modifying it. In Rust: a trait
/// of your own adds methods to the positions of every system.
#[test]
fn staquia_systems_custom_position() {
    trait Custom {
        fn custom(&self) -> String;
    }
    impl Custom for Position {
        fn custom(&self) -> String {
            format!("custom {self}")
        }
    }
    let alphabet = Staquia::create('!', 'a', 'z', 4, 12, 0.15).unwrap();
    let digits = Staquia::create('#', '0', '9', 4, 12, 0.15).unwrap();
    assert_system(&alphabet, ALPHABET, "alphabet");
    assert_system(&digits, DIGITS, "digits");
    assert_eq!(alphabet.start().unwrap().custom(), "custom mmmm!");
    assert_eq!(digits.start().unwrap().custom(), "custom 4444#");
}

/// Should accept positions from another system with the same configuration.
#[test]
fn staquia_systems_same_configuration() {
    let first = Staquia::create('!', 'a', 'z', 4, 12, 0.15).unwrap();
    let second = Staquia::create('!', 'a', 'z', 4, 12, 0.15).unwrap();
    let start = first.start().unwrap();
    assert_eq!(second.next(&start).unwrap().to_string(), "mmmn!");
    assert_eq!(second.previous(&start).unwrap().to_string(), "mmml!");
    second.core(&start, &second.next(&start).unwrap()).unwrap();
    // The default system is the same configuration every time it is created
    Staquia::default()
        .next(&Staquia::default().start().unwrap())
        .unwrap();
}

/// Should reject positions from a system with a different configuration.
#[test]
fn staquia_systems_mismatch() {
    let alphabet = Staquia::create('!', 'a', 'z', 4, 12, 0.15).unwrap();
    let digits = Staquia::create('#', '0', '9', 4, 12, 0.15).unwrap();
    let longer = Staquia::create('!', 'a', 'z', 5, 12, 0.15).unwrap();
    let own = alphabet.start().unwrap();
    let foreign = digits.start().unwrap();
    assert_mismatch(alphabet.next(&foreign), "next");
    assert_mismatch(alphabet.previous(&foreign), "previous");
    assert_mismatch(alphabet.core(&own, &foreign), "core(own, foreign)");
    assert_mismatch(alphabet.core(&foreign, &own), "core(foreign, own)");
    // The same characters with a different `n` length give the digits another meaning
    assert_mismatch(longer.next(&own), "next with a different n length");
}

// Sync checks

/// Should agree with the errors returned by forge at every length boundary. A position longer than the
/// maximum can't exist in Rust, so the overflow length is checked through the error.
#[test]
fn staquia_sync_checks_agree_with_errors() {
    let system = system();
    for length in [13, 14, 15, 16, 17] {
        let string = format!("mmmm{}", "a".repeat(length - 4));
        match system.forge(&string) {
            Ok(position) => {
                assert!(
                    !system.has_overflow(&position),
                    "hasOverflow with the length {length}."
                );
                assert!(
                    !system.is_threshold_reached(&position),
                    "isThresholdReached with the length {length}."
                );
            }
            Err(PositionError::ThresholdReached { position, .. }) => {
                assert!(
                    !system.has_overflow(&position),
                    "hasOverflow with the length {length}."
                );
                assert!(
                    system.is_threshold_reached(&position),
                    "isThresholdReached with the length {length}."
                );
            }
            Err(error) => {
                assert_eq!(
                    error,
                    PositionError::Overflow { length, limit: 16 },
                    "The length {length}."
                );
                assert!(length > 16);
            }
        }
    }
}

/// Should not consider a position with the max length an overflow.
#[test]
fn staquia_sync_checks_max_length() {
    let system = system();
    let error = system
        .forge(&format!("mmmm{}", "a".repeat(12)))
        .unwrap_err();
    let PositionError::ThresholdReached { position, .. } = error else {
        panic!("A position with the length lmax only reaches the threshold, got '{error}'.")
    };
    assert!(!system.has_overflow(&position));
    assert!(system.is_threshold_reached(&position));
}
