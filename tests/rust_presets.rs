//! The presets and the systems from a set of characters (Rust and JavaScript create the same positions with them,
//! see `tests/cross_language.rs`).

use staquia::{CreateError, Options, PositionError, Staquia};

/// Every preset with its name.
fn presets() -> [(&'static str, Staquia); 5] {
    [
        ("default", Staquia::default()),
        ("alphabet", Staquia::alphabet()),
        ("ascii", Staquia::ascii()),
        ("url_safe", Staquia::url_safe()),
        ("base36", Staquia::base36()),
    ]
}

/// A small deterministic random generator, so the test needs no dependency.
struct Random(u64);

impl Random {
    fn below(&mut self, limit: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % limit as u64) as usize
    }
}

#[test]
fn presets_have_their_characters() {
    let summary: Vec<(u32, String)> = presets()
        .iter()
        .map(|(_, system)| {
            (
                system.base(),
                system.start().map(|p| p.to_string()).unwrap_or_default(),
            )
        })
        .collect();
    assert_eq!(
        summary,
        [
            (1027, "ʰʰʰʰʰ!".to_string()),
            (27, "mmmmm!".to_string()),
            (90, "PPPPP!".to_string()),
            (64, "VVVVV-".to_string()),
            (36, "iiiii0".to_string()),
        ]
    );
    assert_eq!(
        Staquia::create_with_options(Options::url_safe()),
        Ok(Staquia::url_safe())
    );
}

/// The strings of the positions must sort exactly as the positions, whatever the system: a database compares the
/// strings, the library compares the digits.
#[test]
fn string_order_is_position_order_for_every_preset() {
    for (name, system) in presets() {
        let mut random = Random(0x9E37_79B9_7F4A_7C15);
        let mut positions = vec![system.start().unwrap()];
        for _ in 0..2_000 {
            let index = random.below(positions.len());
            let created = match random.below(3) {
                0 => system.next(&positions[positions.len() - 1]),
                1 => system.previous(&positions[0]),
                _ if index + 1 < positions.len() => {
                    system.core(&positions[index], &positions[index + 1])
                }
                _ => system.next(&positions[positions.len() - 1]),
            };
            let position = match created {
                Ok(position) | Err(PositionError::ThresholdReached { position, .. }) => position,
                // A full gap is fine: the list would rebalance
                Err(_) => continue,
            };
            let at = positions.partition_point(|p| *p < position);
            if positions.get(at) != Some(&position) {
                positions.insert(at, position);
            }
        }
        let strings: Vec<String> = positions.iter().map(ToString::to_string).collect();
        let mut sorted = strings.clone();
        sorted.sort();
        assert_eq!(
            strings, sorted,
            "{name}: the strings must sort like the positions"
        );
        // Every string is read back as the same position
        for (position, string) in positions.iter().zip(&strings) {
            let forged = match system.forge(string) {
                Ok(forged)
                | Err(PositionError::ThresholdReached {
                    position: forged, ..
                }) => forged,
                Err(error) => panic!("{name}: {string} can't be forged: {error}"),
            };
            assert_eq!(&forged, position, "{name}: {string}");
        }
    }
}

#[test]
fn a_set_with_consecutive_digits_is_the_range() {
    let set = Staquia::from_characters(&"!abcdefghijklmnopqrstuvwxyz", 5, 15, 0.15).unwrap();
    assert_eq!(set, Staquia::alphabet());
    assert_eq!(set.options().characters, None);
    // Their positions are compatible
    let position = Staquia::alphabet().start().unwrap();
    assert!(set.owns(&position));
    assert_eq!(set.next(&position).unwrap().to_string(), "mmmmn!");
}

#[test]
fn systems_of_different_sets_are_different() {
    let url = Staquia::url_safe();
    let base36 = Staquia::base36();
    let position = url.start().unwrap();
    assert!(!base36.owns(&position));
    assert!(matches!(
        base36.next(&position),
        Err(PositionError::SystemMismatch { .. })
    ));
    // The same set with other lengths is another system too
    let longer = Staquia::create_with_options(Options {
        ln: 6,
        ..Options::url_safe()
    })
    .unwrap();
    assert!(!longer.owns(&position));
    assert_eq!(longer.start().unwrap().to_string(), "VVVVVV-");
}

#[test]
fn sets_are_validated() {
    assert_eq!(
        Staquia::from_characters(&"", 5, 15, 0.15),
        Err(CreateError::FewCharacters(0))
    );
    assert_eq!(
        Staquia::from_characters(&"!", 5, 15, 0.15),
        Err(CreateError::FewCharacters(1))
    );
    assert_eq!(
        Staquia::from_characters(&"a!", 5, 15, 0.15),
        Err(CreateError::Unsorted(1))
    );
    assert_eq!(
        Staquia::from_characters(&"!abcb", 5, 15, 0.15),
        Err(CreateError::Unsorted(4))
    );
    assert_eq!(
        Staquia::from_characters(&"!abb", 5, 15, 0.15),
        Err(CreateError::Unsorted(3))
    );
    assert_eq!(
        Staquia::from_characters(&"!a😀", 5, 15, 0.15),
        Err(CreateError::Character('😀'))
    );
    assert_eq!(
        Staquia::from_characters(&"!ac", 0, 15, 0.15),
        Err(CreateError::Length { ln: 0, lz: 15 })
    );
}

#[test]
fn characters_outside_a_set_are_invalid() {
    let system = Staquia::url_safe();
    // "." is URL safe but not a digit of the set, "/" is between "-" and "0"
    assert_eq!(
        system.forge("VV.V"),
        Err(PositionError::InvalidCharacter {
            character: '.',
            index: 2
        })
    );
    assert_eq!(
        system.forge("V/"),
        Err(PositionError::InvalidCharacter {
            character: '/',
            index: 1
        })
    );
    // A digit past the base has no character
    assert!(matches!(
        system.from_digits(&[1, 64]),
        Err(PositionError::InvalidCharacter { index: 1, .. })
    ));
    let digits = system.forge("aZ_").unwrap();
    assert_eq!(
        system.from_digits(digits.digits()).unwrap().to_string(),
        "aZ_---"
    );
}
