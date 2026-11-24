//! The Rust-only API of `Staquia` and `Position` (validation, options, `Hash`, `Copy`), complementing the
//! port of the JavaScript `staquia.spec.ts` in `staquia.rs`.

use std::collections::HashSet;

use staquia::{CreateError, MAX_LENGTH, Options, Position, PositionError, Staquia};

fn system() -> Staquia {
    Staquia::create('!', 'a', 'z', 4, 12, 0.15).expect("valid system")
}

fn forge(string: &str) -> Position {
    system().forge(string).expect("valid position")
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
fn trailing_zeroes_are_equal_with_the_same_hash() {
    let (left, right) = (forge("mmmma"), forge("mmmma!!"));
    assert_eq!(left, right);
    let set: HashSet<Position> = [left, right].into_iter().collect();
    assert_eq!(set.len(), 1);
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
    let Err(PositionError::Overflow {
        position,
        length,
        limit,
    }) = system.forge("mmmmabcdefghijklm")
    else {
        panic!("not an overflow")
    };
    assert_eq!(
        (position.to_string().as_str(), length, limit),
        ("mmmmabcdefghijklm", 17, 16)
    );
    assert!(system.has_overflow(&position));
}

/// Takes the position of a result, also the valid one inside a threshold error.
fn any(result: Result<Position, PositionError>) -> Position {
    match result {
        Ok(position) | Err(PositionError::ThresholdReached { position, .. }) => position,
        Err(error) => panic!("{error}"),
    }
}

#[test]
fn an_overflow_of_core_has_the_complete_middle_position() {
    // ln + lz = 6: "mmmmma" and "mmmmmb" are adjacent and their middle needs a seventh digit
    let system = Staquia::create('!', 'a', 'z', 5, 1, 0.0).unwrap();
    let a = any(system.forge("mmmmma"));
    let b = any(system.forge("mmmmmb"));
    let error = system.core(&a, &b).unwrap_err();
    let PositionError::Overflow {
        position,
        length,
        limit,
    } = error
    else {
        panic!("{error}")
    };
    assert_eq!(
        (position.to_string().as_str(), length, limit),
        ("mmmmmam", 7, 6)
    );
    assert_eq!(error.position(), Some(&position));
    assert!(error.is_overflow());
    assert_eq!(
        error.to_string(),
        "The position \"mmmmmam\" caused an overflow as its length '7' is bigger than the allowed system length '6'."
    );
}

#[test]
fn an_overflow_of_a_long_input_is_cut_after_the_first_extra_digit() {
    let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15).unwrap();
    let error = system.forge(&"a".repeat(40)).unwrap_err();
    let PositionError::Overflow {
        position, length, ..
    } = error
    else {
        panic!("{error}")
    };
    assert_eq!((position.to_string(), length), ("a".repeat(21), 40));
    assert!(
        error
            .to_string()
            .starts_with(&format!("The position \"{}...\"", "a".repeat(21)))
    );

    let Err(PositionError::Overflow {
        position, length, ..
    }) = system.from_digits(&[1; 200])
    else {
        panic!("not an overflow")
    };
    assert_eq!((position.to_string(), length), ("a".repeat(21), 200));
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
    // Digits, lengths and signature (with a reference to a static character set), no heap
    assert!(size_of::<Position>() <= 2 * MAX_LENGTH + 24);
    let a = system().start().unwrap();
    let b = a; // Copy
    assert_eq!(a, b);
    assert_eq!(format!("{a:?}"), "Position(\"mmmm!\")");
}
