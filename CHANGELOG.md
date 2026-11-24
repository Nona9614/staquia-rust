# Staquia (Rust) Changelog

Select a version below to view the changelog history:

- [Staquia 0.1.0](#V0.1.0) **Current**

## V0.1.0

The first release: the Rust version of the [staquia](https://nona9614.github.io/staquia-js/) JavaScript library 0.5.1. For the same system and operations both libraries create the same positions, checked with more than 20,000 operations recorded with the JavaScript library.

- `Staquia` creates the positions: `create`, `create_with_options` and `default` (the same default system as JavaScript), `start`, `next`, `previous`, `core`, `forge` and `from_digits`.
- `Position` is a `Copy` value that never allocates. It implements `Ord`, `Eq` and `Hash` with the same order as its string, and `Display` writes the string to store.
- `LexicalList` keeps the order of your values and their positions in sync: `push`, `unshift`, `move_to`, `delete`, `set`, `set_untrusted`, `sort_by` and `rebalance`. When an operation fails, neither the list nor your values are changed.
- Errors are enums with `From` implementations, so `?` works with every operation: `CreateError`, `PositionError`, `LexicalError` and `staquia::Error`, which combines them. Nothing panics.
- Values moved into a list come back on failure in a `Rejected` (`into_value()`), for example to retry after `rebalance()`.
- No dependencies, no `unsafe`, and Rust 1.85 or later (the first version with the 2024 edition).

The differences with the JavaScript API are listed in the README ("Differences with JavaScript").
