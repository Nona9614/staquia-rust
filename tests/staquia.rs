//! The public API of `Staquia` and `Position`, ported from the JavaScript `staquia.spec.ts`.

use std::collections::HashSet;

use staquia::{CreateError, Direction, MAX_LENGTH, Options, Position, PositionError, Staquia};

fn system() -> Staquia {
    Staquia::create('!', 'a', 'z', 4, 12, 0.15).expect("valid system")
}

fn forge(string: &str) -> Position {
    system().forge(string).expect("valid position")
}

/// `left` is strictly before `right` both as strings (what a database sorts) and as positions.
fn assert_before(left: &Position, right: &Position) {
    assert!(
        left.to_string() < right.to_string(),
        "'{left}' must sort before '{right}' as a string"
    );
    assert!(left < right, "'{left}' must be less than '{right}'");
    assert!(right > left, "'{right}' must be greater than '{left}'");
}

#[test]
fn forge_pads_n_with_zeroes_to_the_right() {
    assert_eq!(forge("abc").to_string(), "abc!!");
    assert_eq!(forge("xy").to_string(), "xy!!!");
    assert_eq!(forge("b").to_string(), "b!!!!");
    assert_eq!(forge("").to_string(), "!!!!!");
}

#[test]
fn forge_keeps_the_string_order() {
    let strings = ["abc", "abcs", "b", "bbbb", "mmmm!m", "mmmma", "xy", "zzzz"];
    let positions: Vec<Position> = strings.iter().map(|s| forge(s)).collect();
    for pair in positions.windows(2) {
        assert_before(&pair[0], &pair[1]);
    }
}

#[test]
fn forge_round_trips() {
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
        assert_eq!(forged, position);
    }
}

#[test]
fn forge_rejects_characters_that_are_not_digits() {
    let system = system();
    for (string, character, index) in [
        ("¼¼¼¼¿!", '¼', 0),
        ("abcD", 'D', 3),
        ("ab{c", '{', 2),
        ("mmmm!m😀", '😀', 6),
        ("a\"aa", '"', 1),
        ("aa`a", '`', 2),
        ("aaa0", '0', 3),
    ] {
        assert_eq!(
            system.forge(string),
            Err(PositionError::InvalidCharacter { character, index }),
            "{string}"
        );
    }
    assert_eq!(system.forge("!azz!a").unwrap().to_string(), "!azz!a");
}

#[test]
fn compares_different_z_lengths_by_their_string_order() {
    assert_before(&forge("mmmm!m"), &forge("mmmma"));
    assert_before(&forge("mmmm!"), &forge("mmmm!m"));
    assert_before(&forge("mmmma"), &forge("mmmmaa"));
    assert_before(&forge("mmmm!z"), &forge("mmmma!"));
}

#[test]
fn trailing_zeroes_are_equal_with_the_same_hash() {
    let (left, right) = (forge("mmmma"), forge("mmmma!!"));
    assert_eq!(left, right);
    let set: HashSet<Position> = [left, right].into_iter().collect();
    assert_eq!(set.len(), 1);
}

#[test]
fn next_and_previous_are_strictly_monotonic_and_inverse() {
    let system = system();
    let start = system.start().unwrap();
    let mut position = start;
    for _ in 0..200 {
        let next = system.next(&position).unwrap();
        assert_before(&position, &next);
        position = next;
    }
    let mut position = start;
    for _ in 0..200 {
        let previous = system.previous(&position).unwrap();
        assert_before(&previous, &position);
        position = previous;
    }
    assert_eq!(
        system.previous(&system.next(&start).unwrap()).unwrap(),
        start
    );
    assert_eq!(
        system.next(&system.previous(&start).unwrap()).unwrap(),
        start
    );
}

#[test]
fn next_and_previous_keep_the_leading_zeroes_of_n() {
    let system = system();
    let next = |s: &str| system.next(&forge(s)).unwrap().to_string();
    let previous = |s: &str| system.previous(&forge(s)).unwrap().to_string();
    assert_eq!(next("!!!a"), "!!!b!");
    assert_eq!(next("!!ab"), "!!ac!");
    assert_eq!(next("!!!z"), "!!a!!");
    assert_eq!(next("!abc"), "!abd!");
    assert_eq!(previous("!!ab"), "!!aa!");
    assert_eq!(previous("!!a!"), "!!!z!");
    assert_eq!(previous("!!!b"), "!!!a!");
    assert_eq!(previous("a!!!"), "!zzz!");
    assert_eq!(next("zzzy"), "zzzz!");
    assert_eq!(previous("!!!a"), "!!!!!");
}

#[test]
fn next_and_previous_fail_at_the_edges_of_n() {
    let system = system();
    for string in ["zzzz", "zzzzm"] {
        let position = forge(string);
        assert_eq!(
            system.next(&position),
            Err(PositionError::EdgeReached {
                position,
                direction: Direction::Next
            })
        );
    }
    for string in ["!!!!", "!!!!a"] {
        let position = forge(string);
        let error = system.previous(&position).unwrap_err();
        assert_eq!(
            error,
            PositionError::EdgeReached {
                position,
                direction: Direction::Previous
            }
        );
        assert!(error.is_overflow());
    }
}

#[test]
fn walking_never_wraps_around() {
    let system = system();
    let mut position = forge("zzyr");
    loop {
        match system.next(&position) {
            Ok(next) => {
                assert_before(&position, &next);
                position = next;
            }
            Err(error) => {
                assert!(error.is_overflow());
                assert_eq!(position.to_string(), "zzzz!");
                break;
            }
        }
    }
    let mut position = forge("!!ah");
    loop {
        match system.previous(&position) {
            Ok(previous) => {
                assert_before(&previous, &position);
                position = previous;
            }
            Err(error) => {
                assert!(error.is_overflow());
                assert_eq!(position.to_string(), "!!!!!");
                break;
            }
        }
    }
}

#[test]
fn core_narrows_a_gap_until_the_threshold() {
    let system = system();
    for from_left in [true, false] {
        let mut left = system.start().unwrap();
        let mut right = system.next(&left).unwrap();
        let mut insertions = 0;
        loop {
            match system.core(&left, &right) {
                Ok(core) => {
                    assert_before(&left, &core);
                    assert_before(&core, &right);
                    insertions += 1;
                    if from_left { left = core } else { right = core }
                }
                Err(error) => {
                    assert!(error.is_dangerous() || error.is_overflow(), "{error}");
                    assert!(insertions > 0);
                    break;
                }
            }
            assert!(insertions < 500, "the gap never reached the threshold");
        }
    }
}

#[test]
fn core_ignores_the_order_of_its_arguments() {
    let system = system();
    let start = system.start().unwrap();
    let next = system.next(&system.next(&start).unwrap()).unwrap();
    assert_eq!(
        system.core(&start, &next).unwrap().to_string(),
        system.core(&next, &start).unwrap().to_string()
    );
}

#[test]
fn threshold_reached_keeps_the_valid_position() {
    let system = system();
    // ln + lz = 16, the threshold is ceil(0.85 * 16) = 14
    assert_eq!(system.threshold_length(), 14);
    let error = system.forge("mmmmabcdefghij").unwrap_err();
    let PositionError::ThresholdReached {
        position,
        length,
        threshold,
    } = error
    else {
        panic!("{error}")
    };
    assert_eq!(
        (position.to_string().as_str(), length, threshold),
        ("mmmmabcdefghij", 14, 14)
    );
    assert!(system.is_threshold_reached(&position));
    assert!(!system.has_overflow(&position));
    assert_eq!(
        system.forge("mmmmabcdefghijklm"),
        Err(PositionError::Overflow {
            length: 17,
            limit: 16
        })
    );
}

#[test]
fn positions_of_other_systems_cannot_be_mixed() {
    let alphabet = system();
    let digits = Staquia::create('#', '0', '9', 4, 12, 0.15).unwrap();
    let position = digits.start().unwrap();
    assert_eq!(position.to_string(), "4444#");
    assert!(!alphabet.owns(&position));
    assert_eq!(
        alphabet.next(&position),
        Err(PositionError::SystemMismatch { position })
    );
    assert_eq!(
        alphabet.core(&alphabet.start().unwrap(), &position),
        Err(PositionError::SystemMismatch { position })
    );
    // The same configuration is compatible
    let twin = system();
    assert!(twin.owns(&alphabet.start().unwrap()));
}

#[test]
fn each_system_prints_its_own_characters() {
    let alphabet = system();
    let digits = Staquia::create('#', '0', '9', 4, 12, 0.15).unwrap();
    for (system, allowed) in [
        (alphabet, "!abcdefghijklmnopqrstuvwxyz"),
        (digits, "#0123456789"),
    ] {
        let start = system.start().unwrap();
        let next = system.next(&start).unwrap();
        for position in [start, next, system.core(&start, &next).unwrap()] {
            let string = position.to_string();
            assert!(string.chars().all(|ch| allowed.contains(ch)), "{string}");
            assert_eq!(system.forge(&string).unwrap(), position);
        }
    }
}

#[test]
fn the_default_system_is_the_javascript_one() {
    let system = Staquia::default();
    assert_eq!(system.options(), Options::default());
    assert_eq!(system.base(), 1027);
    assert_eq!(system.start().unwrap().to_string(), "ʰʰʰʰʰ!");
    let same = Staquia::create('!', '\u{00B0}', '\u{04B1}', 5, 15, 0.15).unwrap();
    assert_eq!(same, system);
}

#[test]
fn create_with_options_changes_only_the_given_fields() {
    let system = Staquia::create_with_options(Options {
        ln: 6,
        ..Options::default()
    })
    .unwrap();
    assert_eq!((system.ln(), system.lz(), system.max_length()), (6, 15, 21));
    assert_eq!(system.start().unwrap().to_string(), "ʰʰʰʰʰʰ!");
}

#[test]
fn create_validates_the_configuration() {
    assert!(matches!(
        Staquia::create('a', '!', 'z', 5, 15, 0.15),
        Err(CreateError::Order { .. })
    ));
    assert!(matches!(
        Staquia::create('!', 'z', 'a', 5, 15, 0.15),
        Err(CreateError::Order { .. })
    ));
    assert!(matches!(
        Staquia::create('!', 'a', '😀', 5, 15, 0.15),
        Err(CreateError::Character('😀'))
    ));
    assert!(matches!(
        Staquia::create('!', 'a', '\u{E000}', 5, 15, 0.15),
        Err(CreateError::Character(_))
    ));
    assert!(matches!(
        Staquia::create('!', 'a', 'z', 0, 15, 0.15),
        Err(CreateError::Length { .. })
    ));
    assert!(matches!(
        Staquia::create('!', 'a', 'z', 5, 0, 0.15),
        Err(CreateError::Length { .. })
    ));
    assert!(matches!(
        Staquia::create('!', 'a', 'z', 20, MAX_LENGTH - 19, 0.15),
        Err(CreateError::Length { .. })
    ));
    assert!(matches!(
        Staquia::create('!', 'a', 'z', 5, 15, 1.5),
        Err(CreateError::Threshold(_))
    ));
    assert!(matches!(
        Staquia::create('!', 'a', 'z', 5, 15, f64::NAN),
        Err(CreateError::Threshold(_))
    ));
    assert!(Staquia::create('!', 'a', 'z', 20, MAX_LENGTH - 20, 0.15).is_ok());
    assert!(Staquia::create('!', '\u{E000}', '\u{FFFF}', 5, 15, 0.15).is_ok());
}

#[test]
fn from_digits_is_the_inverse_of_digits() {
    let system = system();
    let position = forge("abcdefg");
    assert_eq!(
        (position.n(), position.z()),
        (&[1, 2, 3, 4][..], &[5, 6, 7][..])
    );
    assert_eq!(system.from_digits(position.digits()).unwrap(), position);
    assert_eq!(
        system.from_digits(&[1, 30]),
        Err(PositionError::InvalidCharacter {
            character: '~',
            index: 1
        })
    );
}

#[test]
fn sorting_positions_and_strings_gives_the_same_order() {
    let system = system();
    let start = system.start().unwrap();
    let mut positions = vec![start];
    let mut left = start;
    let mut right = system.next(&start).unwrap();
    positions.push(right);
    for i in 0..10 {
        let core = system.core(&left, &right).unwrap();
        positions.push(core);
        if i % 2 == 0 {
            left = core
        } else {
            right = core
        }
        positions.push(system.previous(&positions[0]).unwrap());
    }
    let mut strings: Vec<String> = positions.iter().map(Position::to_string).collect();
    positions.sort();
    strings.sort();
    assert_eq!(
        positions
            .iter()
            .map(Position::to_string)
            .collect::<Vec<_>>(),
        strings
    );
}

#[test]
fn positions_are_small_stack_values() {
    // Digits, length and signature, no pointer
    assert!(size_of::<Position>() <= 2 * MAX_LENGTH + 24);
    let a = system().start().unwrap();
    let b = a; // Copy
    assert_eq!(a, b);
    assert_eq!(format!("{a:?}"), "Position(\"mmmm!\")");
}
