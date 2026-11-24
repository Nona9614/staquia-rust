# staquia

[![crates.io](https://img.shields.io/crates/v/staquia.svg)](https://crates.io/crates/staquia)
[![docs.rs](https://img.shields.io/docsrs/staquia)](https://docs.rs/staquia)
[![MSRV](https://img.shields.io/badge/rustc-1.85+-blue.svg)](#install)
[![license](https://img.shields.io/crates/l/staquia.svg)](#license)

Short lexicographic positions for ordered lists: moving an item writes one row.

- Zero dependencies, no `unsafe`, nothing panics.
- Positions are `Copy` values that never allocate.
- The same positions as the JavaScript library, checked with more than 20,000 recorded operations.

Staquia is a configurable, zero dependency, lexicographic **position** algorithm. It solves the reinsertion and sorting problem of long lists: every item stores a short string position, and sorting those strings gives the order of the list.

When an item is moved, only **its own** position changes, the surrounding items never need to be updated. This makes it ideal for lists and tables stored in a database, where reordering one row should mean writing one row.

To have a full detailed explanation you can go to the [documentation site](https://nona9614.github.io/staquia-js/): choose **Rust** in the language selector at the top and every example is shown in Rust.

This crate is the Rust version of the [staquia](https://nona9614.github.io/staquia-js/) JavaScript library (0.5.1). For the same system and operations both create **the same positions**, so a Rust service can read and write the positions stored by a JavaScript app. The tests of the repository replay more than 20,000 operations recorded with the JavaScript library and assert the same strings.

- [Install](#install)
- [Quick start](#quick-start)
- [How positions work](#how-positions-work)
- [Creating a system](#creating-a-system)
- [Positions](#positions)
- [Errors](#errors)
- [Lexical list](#lexical-list)
- [Differences with JavaScript](#differences-with-javascript)
- [Running the tests](#running-the-tests)
- [License](#license)

## Install

```bash
cargo add staquia
```

The crate has no dependencies and no `unsafe` code, and needs Rust 1.85 or later (the first version with the 2024 edition). The API documentation is on [docs.rs](https://docs.rs/staquia), and the guides (how positions work, using them in a database, error handling) are on the [documentation site](https://nona9614.github.io/staquia-js/), with Rust examples.

## Quick start

```rust
use staquia::{Item, LexicalList, Position, Staquia};

struct Task {
    id: u32,
    title: &'static str,
    position: String,
}

impl Item for Task {
    type Id = u32;
    fn id(&self) -> &u32 { &self.id }
    fn position(&self) -> &str { &self.position }
    fn set_position(&mut self, position: &Position) { self.position = position.to_string(); }
}

fn main() -> Result<(), staquia::Error<u32>> {
    let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;
    let mut list = LexicalList::new(system);

    list.push(Task { id: 1, title: "Write", position: String::new() })?; // "mmmmm!"
    list.push(Task { id: 2, title: "Review", position: String::new() })?; // "mmmmn!"
    list.push(Task { id: 3, title: "Publish", position: String::new() })?; // "mmmmo!"

    // Moves "Publish" between "Write" and "Review": only its position changes
    let moved = list.move_to(&3, Some(&1), Some(&2))?;
    assert_eq!(moved.value().position, "mmmmmn"); // save this one row
    Ok(())
}
```

More in the `examples/` folder: `cargo run --example basic` and `cargo run --example list`.

## How positions work

A system has a zero character and a range of characters from `first` to `last`: they are the digits of a number system of base `(last - first) + 2`. A position is a number written with those digits, split in two sections:

- `N`, of a fixed length `ln`: stacking an item before or after the list adds or subtracts `1` here, so positions never get longer.
- `Z`, of up to `lz` digits: moving an item between two adjacent positions takes the middle value here, one digit longer.

Positions compare exactly as their strings, so a database sorts them with a plain `ORDER BY position`. Moving items again and again into the same gap makes positions grow: the system warns at the **threshold** length and fails at the **overflow** length, and the list can then be rebalanced (every position spaced again).

## Creating a system

```rust
use staquia::{Options, Staquia};

// The alphabet: "!" is 0, "a" is 1 and "z" is 26, a base 27 system
let alphabet = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;

// The default system, the same one as `Staquia.default()` in JavaScript:
// "!", from "°" (U+00B0) to "ұ" (U+04B1), base 1027, ln = 5, lz = 15, threshold 0.15
let system = Staquia::default();

// Only some options changed
let longer = Staquia::create_with_options(Options { ln: 6, ..Options::default() })?;
# Ok::<(), staquia::Error>(())
```

`create` validates the configuration (cheap checks) and returns a `CreateError` when:

- the characters are not `zero < first <= last`,
- a character is above `U+FFFF`, or the range includes the surrogates (`U+D800` to `U+DFFF`), so Rust, JavaScript, UTF-8 and UTF-16 compare them in the same order,
- `ln` or `lz` is `0`, or `ln + lz` is above `MAX_LENGTH` (32),
- the threshold factor is not from `0` to `1`.

A `Staquia` is a small `Copy` value.

**Why 32.** The JavaScript library has no fixed limit, but a Rust `Position` keeps its digits in a fixed array (32 digits, plus one so an overflow error can show the position that caused it), so it is a `Copy` value (84 bytes) that never allocates. 32 is more than lists need: the default system uses 20 (`ln = 5`, `lz = 15`), `N` stacks about 5.7 × 10¹⁴ items in each direction, and a longer `Z` only allows more moves into the same gap (121 with 20 digits, 242 with 32) before the list has to be rebalanced, which happens with any limit. Longer positions also cost space in the database and its index (up to 3 bytes per digit in UTF-8).

## Positions

```rust
use staquia::Staquia;

let system = Staquia::create('!', 'a', 'z', 5, 15, 0.15)?;

let start = system.start()?; // "mmmmm!": the middle digit, as much room before as after
let next = system.next(&start)?; // "mmmmn!"
let previous = system.previous(&start)?; // "mmmml!"
let core = system.core(&start, &next)?; // "mmmmmn": between both, the order of the arguments doesn't matter

// From a stored string; shorter strings are completed with zeroes to the right
let stored = system.forge("xy")?; // "xy!!!!"

// Positions implement Ord, Eq and Hash with the same order as their strings
assert!(previous < start && start < core && core < next);
assert_eq!(core.to_string(), "mmmmmn");
# Ok::<(), staquia::Error>(())
```

A `Position` is a `Copy` value that lives on the stack: its digits in a fixed array (`MAX_LENGTH` plus one, for the position of an overflow error) and the signature of the system that created it (its characters and `ln`), with no reference to the system. `Display` writes its string, the value to store. `n()`, `z()` and `digits()` give its digits, and `Staquia::from_digits` creates a position back from them.

## Errors

Nothing panics: every operation that can fail returns a `Result` with an error enum.

| Error | When |
| --- | --- |
| `PositionError::InvalidCharacter` | `forge` found a character that is not a digit of the system |
| `PositionError::Overflow` | the position would be longer than `ln + lz`: the error has that position, to log it |
| `PositionError::EdgeReached` | `next` of the last value of `N`, or `previous` of the first one |
| `PositionError::ThresholdReached` | the position reached the threshold: it is still valid (the error has it) but the list should be rebalanced |
| `PositionError::SystemMismatch` | the position was created by a system with another configuration |
| `CreateError` | `create` received an invalid configuration |
| `LexicalError<K>` | an operation of a list failed, see below |

`staquia::Error<K>` combines all of them with `From` implementations, so `?` works with every operation in the same function without boxing the errors. The helpers `is_overflow()`, `is_dangerous()`, `is_mismatch()` and `is_invalid()` mirror the JavaScript error classes.

## Lexical list

`LexicalList` keeps the order of your values and their positions in sync, as a doubly linked list of ids in a `HashMap` (no pointers, no `Rc`). Implement `Item` for your type: an id, the stored position string, and `set_position`, which receives every new position.

| Method | |
| --- | --- |
| `push(value)` / `unshift(value)` | adds at the end / start |
| `move_to(&id, previous, next)` | moves between two neighbors (`None` for an end), only the moved item changes |
| `delete(&id)` | removes the item and gives its value back |
| `set(values)` | loads rows already sorted by position, verified and not changed |
| `set_untrusted(values)` | loads rows and repairs them: sorted, duplicates and invalid positions given new ones |
| `sort_by(compare)` | sorts and spaces every position again from the start |
| `rebalance()` | spaces every position again keeping the order, to retry after a gap is exhausted |
| `head()` / `tail()` | the position a new item would get at the start / end |
| `first()`, `last()`, `find(&id)`, `find_mut(&id)`, `at(index)`, `iter()`, `len()` | reading |
| `select(&id)`, `selected()`, `deselect()` | the selected item, cleared when it is deleted |

When an operation fails, neither the list nor your values are changed. `push`, `unshift`, `set` and `set_untrusted` take ownership of the values, so on failure they return a `Rejected` with the error and the values given back (`rejected.into_value()`), for example to retry after `rebalance()`:

```rust
# use staquia::{Item, LexicalList, Position, Staquia};
# struct Task { id: u32, position: String }
# impl Item for Task {
#     type Id = u32;
#     fn id(&self) -> &u32 { &self.id }
#     fn position(&self) -> &str { &self.position }
#     fn set_position(&mut self, position: &Position) { self.position = position.to_string(); }
# }
# let mut list = LexicalList::new(Staquia::default());
# list.push(Task { id: 1, position: String::new() })?;
let task = Task { id: 2, position: String::new() };
match list.push(task) {
    Ok(item) => println!("save {}", item.value().position),
    Err(rejected) => {
        let task = rejected.into_value();
        for item in list.rebalance()? {
            println!("save {}", item.value().position);
        }
        list.push(task)?;
    }
}
# Ok::<(), staquia::Error<u32>>(())
```

The operations that change many positions (`set_untrusted`, `sort_by`, `rebalance`) return the affected items, the only rows to save.

## Differences with JavaScript

The logic and the positions are the same; the API follows Rust conventions:

- `Result<T, E>` with error enums instead of the `Ok`/`Err` classes, and `?` instead of chaining results into `next`/`core`.
- `Staquia::create_with_options(Options)` and `Staquia::default()`. The JavaScript option to replace the position and number classes has no equivalent: add methods to `Position` with a trait of your own.
- `Staquia::from_digits(&[u16])` instead of `from(IDigits)`, the digits most significant first.
- `move_to` instead of `move` (a keyword), `sort_by` with a comparison of your values, `iter()` instead of `toArray()`, `at(usize)`.
- `delete` gives the value back. `set` and `set_untrusted` are two methods instead of the `untrusted` option.
- `sort_by` and `rebalance` work like JavaScript's `sort` and `rebalance` (0.5.1): all or nothing, the room for every position is verified first, so a failure leaves the list unchanged.
- Ids can't be `null`, so there is no invalid id error, and the list can't contain a cycle, so there is no map overflow error.
- `InvalidCharacter.index` counts characters; JavaScript counts UTF-16 units. They are the same for every valid string, as systems only use characters up to `U+FFFF`.
- `Position::len()` is the length of the stored string (`"mmmmm!"` is 6); JavaScript's `position.length` is the length measured when the position was created, which doesn't count the zeroes that complete it (5). The checks (`has_overflow`, `is_threshold_reached` and the errors) use that measured length in both libraries.
- An `Overflow` keeps the position that caused it, like JavaScript. When a string or digits given to `forge` or `from_digits` are longer than one digit past the limit, the position is cut there and the error's `length` has the full length.

## Running the tests

```bash
cargo test
```

The 214 tests of the JavaScript library are ported with the same names: the ones of the internal numbers and
algorithms (`number.test.ts`, `algorithms.test.ts`, `composer.test.ts`, `forger.test.ts`) are unit tests inside
`src/`, `staquia.spec.ts` is `tests/staquia.rs` and `lexical-list.spec.ts` is `tests/list.rs`. The tests of the
Rust-only API are in `tests/rust_*.rs`.

`tests/cross_language.rs` replays the operations recorded in `tests/fixtures/*.tsv` with the JavaScript library. These fixtures (1.5 MB) are only in the repository, not in the published crate, whose other tests still run with `cargo test`. To record them again, from the `javascript/` folder of the staquia repository:

```bash
node_modules/.bin/tsx ../rust/tests/fixtures/generate.ts
```

## License

MIT (see `LICENCE`), the same as the JavaScript library.
