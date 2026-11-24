//! Replays the operations recorded with the JavaScript library (`tests/fixtures/generate.ts`) and asserts
//! that Rust creates the same positions and the same list states.

use staquia::{Item, LexicalError, LexicalList, Position, PositionError, Staquia};

/// The range systems of the fixtures, by index (the same as `SYSTEMS` in `generate.ts`); 7 to 10 are sets, see `system`.
const SYSTEMS: [(char, char, char, usize, usize, f64); 7] = [
    ('!', 'a', 'z', 5, 15, 0.15),
    ('!', 'a', 'z', 4, 12, 0.15),
    ('!', '\u{00B0}', '\u{04B1}', 5, 15, 0.15),
    ('!', 'a', 'z', 2, 3, 0.15),
    ('0', '1', '9', 3, 6, 0.3),
    ('!', 'a', 'c', 3, 8, 0.1),
    ('!', 'a', 'z', 1, 31, 0.0),
];

fn system(index: &str) -> Staquia {
    match index.parse::<usize>().expect("system index") {
        // The presets with sets, and a short custom set
        7 => Staquia::ascii(),
        8 => Staquia::url_safe(),
        9 => Staquia::base36(),
        10 => Staquia::from_characters(&"0bcdfghjklmnpqrstvwxyz", 3, 8, 0.1).expect("valid system"),
        index => {
            let (zero, first, last, ln, lz, threshold) = SYSTEMS[index];
            Staquia::create(zero, first, last, ln, lz, threshold).expect("valid system")
        }
    }
}

/// The position of a string, also when it reached the threshold (it is still valid).
fn parse(system: &Staquia, string: &str) -> Position {
    match system.forge(string) {
        Ok(position) | Err(PositionError::ThresholdReached { position, .. }) => position,
        Err(error) => panic!("the fixture position {string:?} is not valid: {error}"),
    }
}

/// Encodes a result as `generate.ts` does.
fn encode(result: Result<Position, PositionError>) -> String {
    match result {
        Ok(position) => position.to_string(),
        Err(PositionError::ThresholdReached { position, .. }) => format!("!threshold:{position}"),
        Err(PositionError::InvalidCharacter { index, .. }) => format!("!invalid:{index}"),
        Err(PositionError::SystemMismatch { .. }) => "!mismatch".to_owned(),
        Err(error) if error.is_overflow() => "!overflow".to_owned(),
        Err(error) => panic!("unexpected error {error}"),
    }
}

/// Fails with the first differences found.
fn report(failures: &[String], total: usize) {
    assert!(
        failures.is_empty(),
        "{} of {total} operations differ from JavaScript:\n{}",
        failures.len(),
        failures
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn positions_match_javascript() {
    let fixture = include_str!("fixtures/positions.tsv");
    let mut failures = Vec::new();
    let mut total = 0;
    for (number, line) in fixture.lines().enumerate() {
        let fields: Vec<&str> = line.split('\t').collect();
        let [index, operation, a, b, expected] = fields[..] else {
            panic!("line {}: bad format", number + 1)
        };
        let system = system(index);
        let actual = match operation {
            "start" => system.start(),
            "forge" => system.forge(a),
            "next" => system.next(&parse(&system, a)),
            "previous" => system.previous(&parse(&system, a)),
            "core" => system.core(&parse(&system, a), &parse(&system, b)),
            "from" => {
                let digits: Vec<u16> = a
                    .split(',')
                    .map(|digit| digit.parse().expect("digit"))
                    .collect();
                system.from_digits(&digits)
            }
            "mismatch" => {
                let other = self::system(b).start().expect("start");
                system.core(&parse(&system, a), &other)
            }
            _ => panic!("line {}: unknown operation {operation}", number + 1),
        };
        total += 1;
        let actual = encode(actual);
        if actual != expected {
            failures.push(format!("line {}: system {index} {operation}({a:?}, {b:?}) = {actual:?}, JavaScript {expected:?}", number + 1));
        }
    }
    report(&failures, total);
}

struct Task {
    id: u32,
    position: String,
}

impl Task {
    fn new(id: u32, position: &str) -> Self {
        Self {
            id,
            position: position.to_owned(),
        }
    }
}

impl Item for Task {
    type Id = u32;

    fn id(&self) -> &u32 {
        &self.id
    }

    fn position(&self) -> &str {
        &self.position
    }

    fn set_position(&mut self, position: &Position) {
        self.position = position.to_string();
    }
}

/// The JavaScript `LexicalErrorType` of an error.
fn kind(error: &LexicalError<u32>) -> &'static str {
    match error {
        LexicalError::Position(_)
        | LexicalError::InvalidPosition { .. }
        | LexicalError::MissingNeighbors(_) => "position",
        LexicalError::NotFound(_) => "not-found",
        LexicalError::NotAdjacent { .. } | LexicalError::SelfNeighbor(_) => "not-adjacent",
        LexicalError::DuplicateId(_) => "duplicate-id",
        LexicalError::NotOrdered { .. } => "not-ordered",
    }
}

fn affected<'a>(items: impl IntoIterator<Item = &'a staquia::LexicalItem<Task>>) -> String {
    let mut ids: Vec<u32> = items.into_iter().map(|item| item.value().id).collect();
    ids.sort_unstable();
    format!(
        "ok:{}",
        ids.iter().map(u32::to_string).collect::<Vec<_>>().join(",")
    )
}

fn state(list: &LexicalList<Task>) -> String {
    list.iter()
        .map(|item| {
            // The value always holds the same position as the item
            assert_eq!(item.value().position, item.position().to_string());
            format!("{}={}", item.value().id, item.position())
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn rows(arguments: &str) -> Vec<Task> {
    if arguments.is_empty() {
        return Vec::new();
    }
    arguments
        .split(',')
        .map(|row| {
            let (id, position) = row.split_once('=').expect("id=position");
            Task::new(id.parse().expect("id"), position)
        })
        .collect()
}

fn id(text: &str) -> Option<u32> {
    (!text.is_empty()).then(|| text.parse().expect("id"))
}

#[test]
fn lists_match_javascript() {
    let fixture = include_str!("fixtures/list.tsv");
    let mut failures = Vec::new();
    let mut total = 0;
    let mut current = ("", "");
    let mut list: Option<LexicalList<Task>> = None;
    for (number, line) in fixture.lines().enumerate() {
        let fields: Vec<&str> = line.split('\t').collect();
        let [
            index,
            scenario,
            operation,
            arguments,
            expected,
            expected_state,
        ] = fields[..]
        else {
            panic!("line {}: bad format", number + 1)
        };
        if current != (index, scenario) {
            current = (index, scenario);
            list = Some(LexicalList::new(system(index)));
        }
        let Some(list) = list.as_mut() else {
            unreachable!()
        };
        let result = match operation {
            "push" => match list.push(Task::new(arguments.parse().expect("id"), "")) {
                Ok(_) => "ok".to_owned(),
                Err(rejected) => format!("err:{}", kind(&rejected.error)),
            },
            "unshift" => match list.unshift(Task::new(arguments.parse().expect("id"), "")) {
                Ok(_) => "ok".to_owned(),
                Err(rejected) => format!("err:{}", kind(&rejected.error)),
            },
            "delete" => match list.delete(&arguments.parse().expect("id")) {
                Ok(_) => "ok".to_owned(),
                Err(error) => format!("err:{}", kind(&error)),
            },
            "move" => {
                let ids: Vec<&str> = arguments.split(',').collect();
                let (item, previous, next) = (ids[0].parse().expect("id"), id(ids[1]), id(ids[2]));
                match list.move_to(&item, previous.as_ref(), next.as_ref()) {
                    Ok(_) => "ok".to_owned(),
                    Err(error) => format!("err:{}", kind(&error)),
                }
            }
            "rebalance" => match list.rebalance() {
                Ok(items) => affected(items),
                Err(error) => format!("err:{}", kind(&error)),
            },
            "set" | "set_untrusted" => {
                let rows = rows(arguments);
                let result = if operation == "set" {
                    list.set(rows)
                } else {
                    list.set_untrusted(rows)
                };
                match result {
                    Ok(items) => affected(items),
                    Err(rejected) => format!("err:{}", kind(&rejected.error)),
                }
            }
            _ => panic!("line {}: unknown operation {operation}", number + 1),
        };
        total += 1;
        let state = state(list);
        if result != expected || state != expected_state {
            failures.push(format!(
                "line {}: system {index} scenario {scenario} {operation}({arguments})\n  rust: {result} [{state}]\n  js:   {expected} [{expected_state}]",
                number + 1
            ));
        }
    }
    report(&failures, total);
}
