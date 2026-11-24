//! The port of the JavaScript `tests/specs/integration-tests/lexical-list.spec.ts` (108 tests). The tests keep
//! the names of the JavaScript methods.
//!
//! Rust differences: ids can't be `null` or `undefined` (the tests of invalid ids check the closest
//! behavior), `at` takes a `usize`, `sort` is `sort_by` over the values, `destroy` is `clear`, and the list
//! owns the values (the "wrapped object" is `item.value()`).

use staquia::{Item, LexicalError, LexicalItem, LexicalList, Position, Staquia};

#[derive(Debug, Clone, PartialEq)]
struct Value {
    id: u32,
    position: String,
}

impl Item for Value {
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

type List = LexicalList<Value>;

fn system() -> Staquia {
    Staquia::create('!', 'a', 'z', 4, 12, 0.15).expect("valid system")
}

fn value(id: u32) -> Value {
    Value {
        id,
        position: String::new(),
    }
}

fn stored(id: u32, position: &str) -> Value {
    Value {
        id,
        position: position.to_owned(),
    }
}

fn new() -> List {
    LexicalList::new(system())
}

/// A list with these ids pushed in order (JavaScript `#create`).
fn create(ids: &[u32]) -> List {
    let mut list = new();
    for id in ids {
        list.push(value(*id)).unwrap();
    }
    list
}

/// The position string `offset` places after the start (JavaScript `#position`).
fn position(offset: usize) -> String {
    let system = system();
    let mut position = system.start().unwrap();
    for _ in 0..offset {
        position = system.next(&position).unwrap();
    }
    position.to_string()
}

/// The JavaScript `LexicalErrorType` of an error.
fn kind<K>(error: &LexicalError<K>) -> &'static str {
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

/// The operation failed with an error of this type.
fn assert_lexical_error<T, E: Into<LexicalError<u32>>>(result: Result<T, E>, expected: &str) {
    match result {
        Ok(_) => panic!("A failure of type '{expected}' was expected but the operation succeeded."),
        Err(error) => assert_eq!(kind(&error.into()), expected),
    }
}

fn ids(items: &[&LexicalItem<Value>]) -> Vec<u32> {
    items.iter().map(|item| *item.id()).collect()
}

fn positions(list: &List) -> Vec<String> {
    list.iter()
        .map(|item| item.position().to_string())
        .collect()
}

/// The list has these ids in this order, linked in both directions, with strictly increasing positions
/// written into the values (JavaScript `assert_links`).
fn assert_links(list: &List, expected: &[u32]) {
    assert_eq!(
        list.len(),
        expected.len(),
        "The size of the list does not match."
    );
    if expected.is_empty() {
        assert!(
            list.first().is_none(),
            "An empty list must not have a first item."
        );
        assert!(
            list.last().is_none(),
            "An empty list must not have a last item."
        );
        assert_eq!(list.iter().count(), 0);
        return;
    }
    let (first, last) = (list.first().unwrap(), list.last().unwrap());
    assert_eq!(first.id(), &expected[0], "The first item does not match.");
    assert_eq!(
        last.id(),
        expected.last().unwrap(),
        "The last item does not match."
    );
    assert!(
        first.previous().is_none(),
        "The first item must not have a previous item."
    );
    assert!(
        last.next().is_none(),
        "The last item must not have a next item."
    );
    // Forward traversal
    let mut forward = Vec::new();
    let mut walker = list.first();
    while let Some(item) = walker {
        if forward.len() > expected.len() {
            break;
        }
        forward.push(*item.id());
        walker = item.next().and_then(|id| list.find(id));
    }
    assert_eq!(forward, expected, "Forward traversal does not match.");
    // Backward traversal
    let mut backward = Vec::new();
    let mut walker = list.last();
    while let Some(item) = walker {
        if backward.len() > expected.len() {
            break;
        }
        backward.push(*item.id());
        walker = item.previous().and_then(|id| list.find(id));
    }
    backward.reverse();
    assert_eq!(backward, expected, "Backward traversal does not match.");
    // Iteration and index access
    let items: Vec<&LexicalItem<Value>> = list.iter().collect();
    assert_eq!(ids(&items), expected, "The iteration does not match.");
    for (index, id) in expected.iter().enumerate() {
        assert_eq!(
            list.at(index).map(|item| *item.id()),
            Some(*id),
            "The item at index {index} does not match."
        );
    }
    for item in &items {
        assert_eq!(
            item.value().position,
            item.position().to_string(),
            "The value must hold the position of its item."
        );
    }
    // Positions must be strictly increasing, both as strings (lexical order) and as positions
    for pair in items.windows(2) {
        let (left, right) = (
            pair[0].position().to_string(),
            pair[1].position().to_string(),
        );
        assert!(
            left < right,
            "The position string '{left}' (id: {}) must sort before '{right}' (id: {}).",
            pair[0].id(),
            pair[1].id()
        );
        assert!(
            pair[0].position() < pair[1].position(),
            "The position '{left}' must be less than '{right}'."
        );
    }
}

/// Sorts by the stored position strings, the JavaScript `sort((a, b) => a.position < b.position ? -1 : 1)`.
fn sort_by_position(list: &mut List) {
    list.sort_by(|a, b| a.position.cmp(&b.position)).unwrap();
}

// Empty list

/// A new list should be empty.
#[test]
fn lexical_list_new_is_empty() {
    let list = new();
    assert_links(&list, &[]);
    assert!(list.selected().is_none());
    assert!(list.at(0).is_none());
}

// Push

/// Should push a single item to an empty list.
#[test]
fn lexical_list_push_item_to_empty_list() {
    let mut list = new();
    let item = list.push(value(1)).unwrap();
    assert_eq!(item.id(), &1);
    assert_eq!(item.position().to_string(), position(0));
    assert_eq!(
        item.value().position,
        position(0),
        "The position must be written back into the value."
    );
    assert!(item.previous().is_none());
    assert!(item.next().is_none());
    assert_eq!(list.first().unwrap().id(), &1);
    assert_eq!(list.last().unwrap().id(), &1);
    assert_links(&list, &[1]);
}

/// Should push multiple items, linking them in both directions.
#[test]
fn lexical_list_push_multiple_items() {
    let list = create(&[1, 2, 3]);
    let [item1, item2, item3] = [1, 2, 3].map(|id| list.find(&id).unwrap());
    assert_eq!((item1.previous(), item1.next()), (None, Some(&2)));
    assert_eq!((item2.previous(), item2.next()), (Some(&1), Some(&3)));
    assert_eq!((item3.previous(), item3.next()), (Some(&2), None));
    assert_eq!(item1.position().to_string(), position(0));
    assert_eq!(item2.position().to_string(), position(1));
    assert_eq!(item3.position().to_string(), position(2));
    assert_links(&list, &[1, 2, 3]);
}

/// Should ignore the incoming position and always append at the tail.
#[test]
fn lexical_list_push_ignores_incoming_position() {
    let mut list = create(&[1]);
    let item = list.push(stored(2, "aaaa")).unwrap();
    assert_eq!(item.position().to_string(), position(1));
    assert_links(&list, &[1, 2]);
}

/// Should push many items keeping the list consistent.
#[test]
fn lexical_list_push_many_items() {
    let ids: Vec<u32> = (1..=50).collect();
    assert_links(&create(&ids), &ids);
}

/// Should reject items with a null or undefined id. Rust ids can't be null: a missing id is never found
/// and the list is not changed.
#[test]
fn lexical_list_push_invalid_id() {
    let list = create(&[1]);
    assert!(list.find(&0).is_none());
    assert!(!list.contains(&0));
    assert_links(&list, &[1]);
}

// Unshift

/// Should unshift a single item to an empty list.
#[test]
fn lexical_list_unshift_item_to_empty_list() {
    let mut list = new();
    let item = list.unshift(value(1)).unwrap();
    assert_eq!(item.id(), &1);
    assert_eq!(item.position().to_string(), position(0));
    assert_eq!(
        item.value().position,
        position(0),
        "The position must be written back into the value."
    );
    assert!(item.previous().is_none());
    assert!(item.next().is_none());
    assert_eq!(list.first().unwrap().id(), &1);
    assert_eq!(list.last().unwrap().id(), &1);
    assert_links(&list, &[1]);
}

/// Should unshift multiple items, linking them in both directions.
#[test]
fn lexical_list_unshift_multiple_items() {
    let mut list = new();
    for id in [1, 2, 3] {
        list.unshift(value(id)).unwrap();
    }
    let [item1, item2, item3] = [1, 2, 3].map(|id| list.find(&id).unwrap());
    assert_eq!((item3.previous(), item3.next()), (None, Some(&2)));
    assert_eq!((item2.previous(), item2.next()), (Some(&3), Some(&1)));
    assert_eq!((item1.previous(), item1.next()), (Some(&2), None));
    assert_links(&list, &[3, 2, 1]);
}

/// Should ignore the incoming position and always insert at the head.
#[test]
fn lexical_list_unshift_ignores_incoming_position() {
    let mut list = create(&[1]);
    let expected = system()
        .previous(list.find(&1).unwrap().position())
        .unwrap()
        .to_string();
    let item = list.unshift(stored(2, "zzzz")).unwrap();
    assert_eq!(item.position().to_string(), expected);
    assert_links(&list, &[2, 1]);
}

/// Should unshift many items keeping the list consistent.
#[test]
fn lexical_list_unshift_many_items() {
    let mut list = new();
    let ids: Vec<u32> = (1..=50).collect();
    for id in &ids {
        list.unshift(value(*id)).unwrap();
    }
    let reversed: Vec<u32> = ids.iter().rev().copied().collect();
    assert_links(&list, &reversed);
}

/// Should mix push and unshift keeping the list consistent.
#[test]
fn lexical_list_unshift_and_push() {
    let mut list = new();
    list.push(value(1)).unwrap();
    list.unshift(value(2)).unwrap();
    list.push(value(3)).unwrap();
    list.unshift(value(4)).unwrap();
    assert_links(&list, &[4, 2, 1, 3]);
}

/// Should keep unshifting after the first item is deleted or moved.
#[test]
fn lexical_list_unshift_after_mutations() {
    let mut list = create(&[1, 2, 3]);
    list.delete(&1).unwrap();
    list.unshift(value(4)).unwrap();
    assert_links(&list, &[4, 2, 3]);
    list.move_to(&3, None, Some(&4)).unwrap();
    list.unshift(value(5)).unwrap();
    assert_links(&list, &[5, 3, 4, 2]);
}

/// Should accept unshifted items after the list was emptied.
#[test]
fn lexical_list_unshift_after_emptied() {
    let mut list = create(&[1]);
    list.delete(&1).unwrap();
    list.unshift(value(2)).unwrap();
    list.unshift(value(3)).unwrap();
    assert_links(&list, &[3, 2]);
}

/// Should reject unshifted items with a null or undefined id. Rust ids can't be null: unshifting an existing
/// id is the only invalid id, and it leaves the list unchanged.
#[test]
fn lexical_list_unshift_invalid_id() {
    let mut list = create(&[1]);
    assert_lexical_error(list.unshift(value(1)), "duplicate-id");
    assert_links(&list, &[1]);
}

// Head

/// `head()` is the position right before the first item (JavaScript `#assert_head`).
fn assert_head(list: &List) {
    let first = *list.first().unwrap().position();
    let head = list.head().unwrap();
    assert_eq!(
        head,
        system().previous(&first).unwrap(),
        "The head must be the previous position of the first item."
    );
    assert!(head < first, "The head must be less than the first item.");
    assert!(
        head.to_string() < first.to_string(),
        "The head string must sort before the first item."
    );
}

/// The head of an empty list should be the start position.
#[test]
fn lexical_list_head_of_empty_list() {
    assert_eq!(new().head().unwrap().to_string(), position(0));
}

/// The head should be the position before the first item.
#[test]
fn lexical_list_head_before_first() {
    assert_head(&create(&[1, 2]));
}

/// The head should be before a single item.
#[test]
fn lexical_list_head_single_item() {
    assert_head(&create(&[1]));
}

/// The head should be based on the positions given to set.
#[test]
fn lexical_list_head_after_set() {
    let mut list = new();
    list.set(vec![stored(1, "bbbb"), stored(2, "cccc")])
        .unwrap();
    assert_head(&list);
}

/// The head should follow the new first item after a delete.
#[test]
fn lexical_list_head_after_delete() {
    let mut list = create(&[1, 2, 3]);
    list.delete(&1).unwrap();
    assert_links(&list, &[2, 3]);
    assert_head(&list);
}

/// The head should follow the new first item after moves.
#[test]
fn lexical_list_head_after_move() {
    let mut list = create(&[1, 2, 3]);
    list.move_to(&3, None, Some(&1)).unwrap();
    assert_links(&list, &[3, 1, 2]);
    assert_head(&list);
    list.move_to(&3, Some(&2), None).unwrap();
    assert_links(&list, &[1, 2, 3]);
    assert_head(&list);
}

/// The head should not modify the list.
#[test]
fn lexical_list_head_is_read_only() {
    let list = create(&[1, 2, 3]);
    let before = positions(&list);
    list.head().unwrap();
    list.head().unwrap();
    assert_links(&list, &[1, 2, 3]);
    assert_eq!(positions(&list), before);
}

// Tail

/// `tail()` is the position right after the last item (JavaScript `#assert_tail`).
fn assert_tail(list: &List) {
    let last = *list.last().unwrap().position();
    let tail = list.tail().unwrap();
    assert_eq!(
        tail,
        system().next(&last).unwrap(),
        "The tail must be the next position of the last item."
    );
    assert!(last < tail, "The tail must be greater than the last item.");
    assert!(
        last.to_string() < tail.to_string(),
        "The tail string must sort after the last item."
    );
}

/// The tail of an empty list should be the start position.
#[test]
fn lexical_list_tail_of_empty_list() {
    assert_eq!(new().tail().unwrap().to_string(), position(0));
}

/// The tail should be the position after the last item.
#[test]
fn lexical_list_tail_after_last() {
    let list = create(&[1, 2]);
    assert_eq!(list.tail().unwrap().to_string(), position(2));
    assert_tail(&list);
}

/// The tail should be after a single item.
#[test]
fn lexical_list_tail_single_item() {
    assert_tail(&create(&[1]));
}

/// The tail should be based on the positions given to set.
#[test]
fn lexical_list_tail_after_set() {
    let mut list = new();
    list.set(vec![stored(1, "bbbb"), stored(2, "cccc")])
        .unwrap();
    assert_tail(&list);
}

/// The tail should follow the new last item after a delete.
#[test]
fn lexical_list_tail_after_delete() {
    let mut list = create(&[1, 2, 3]);
    list.delete(&3).unwrap();
    assert_links(&list, &[1, 2]);
    assert_tail(&list);
}

/// The tail should follow the new last item after moves.
#[test]
fn lexical_list_tail_after_move() {
    let mut list = create(&[1, 2, 3]);
    list.move_to(&1, Some(&3), None).unwrap();
    assert_links(&list, &[2, 3, 1]);
    assert_tail(&list);
    list.move_to(&1, None, Some(&2)).unwrap();
    assert_links(&list, &[1, 2, 3]);
    assert_tail(&list);
}

/// The tail should not modify the list.
#[test]
fn lexical_list_tail_is_read_only() {
    let list = create(&[1, 2, 3]);
    let before = positions(&list);
    list.tail().unwrap();
    list.tail().unwrap();
    assert_links(&list, &[1, 2, 3]);
    assert_eq!(positions(&list), before);
}

// Find

/// Should find existing items and return undefined for missing ones.
#[test]
fn lexical_list_find() {
    let list = create(&[1, 2, 3]);
    assert_eq!(list.find(&2).map(|item| *item.id()), Some(2));
    assert!(list.find(&4).is_none());
}

/// Should select an item and clear the selection with a missing id.
#[test]
fn lexical_list_select() {
    let mut list = create(&[1, 2, 3]);
    list.select(&2);
    assert_eq!(list.selected().map(|item| *item.id()), Some(2));
    list.select(&4);
    assert!(list.selected().is_none());
    list.select(&1);
    list.deselect();
    assert!(list.selected().is_none());
}

// At

/// Should return the item at the given index.
#[test]
fn lexical_list_at() {
    let list = create(&[10, 20, 30]);
    assert_eq!(list.at(0).map(|item| *item.id()), Some(10));
    assert_eq!(list.at(1).map(|item| *item.id()), Some(20));
    assert_eq!(list.at(2).map(|item| *item.id()), Some(30));
    // JavaScript also accepts numeric strings, in Rust the index is a `usize`
    assert_eq!(
        "1".parse()
            .ok()
            .and_then(|index| list.at(index))
            .map(|item| *item.id()),
        Some(20)
    );
}

/// Should return undefined for invalid or out of bounds indexes.
#[test]
fn lexical_list_at_out_of_bounds() {
    let list = create(&[10, 20, 30]);
    assert!(list.at(3).is_none());
    assert!(list.at(usize::MAX).is_none());
    assert!(
        "abc"
            .parse::<usize>()
            .ok()
            .and_then(|index| list.at(index))
            .is_none()
    );
    assert!(
        "-1".parse::<usize>()
            .ok()
            .and_then(|index| list.at(index))
            .is_none()
    );
}

// Delete

/// Should fail to delete an item that does not exist.
#[test]
fn lexical_list_delete_missing() {
    let mut list = create(&[1, 2]);
    assert_lexical_error(list.delete(&3), "not-found");
    assert_links(&list, &[1, 2]);
}

/// Should fail to delete using a null id. Rust ids can't be null: the closest is an id that was never added.
#[test]
fn lexical_list_delete_null_id() {
    let mut list = create(&[1, 2]);
    assert_lexical_error(list.delete(&0), "not-found");
    assert_links(&list, &[1, 2]);
}

/// Should delete the only item leaving the list empty.
#[test]
fn lexical_list_delete_only_item() {
    let mut list = create(&[1]);
    list.delete(&1).unwrap();
    assert_links(&list, &[]);
}

/// Should delete the first item and promote the next one as first.
#[test]
fn lexical_list_delete_first() {
    let mut list = create(&[1, 2, 3]);
    list.delete(&1).unwrap();
    assert_links(&list, &[2, 3]);
}

/// Should delete the last item and promote the previous one as last.
#[test]
fn lexical_list_delete_last() {
    let mut list = create(&[1, 2, 3]);
    list.delete(&3).unwrap();
    assert_links(&list, &[1, 2]);
}

/// Should delete a middle item and link its neighbors.
#[test]
fn lexical_list_delete_middle() {
    let mut list = create(&[1, 2, 3]);
    list.delete(&2).unwrap();
    assert_links(&list, &[1, 3]);
}

/// Should delete every item one by one.
#[test]
fn lexical_list_delete_all() {
    let mut list = create(&[1, 2, 3, 4, 5]);
    let mut remaining = vec![1, 2, 3, 4, 5];
    for id in [3, 1, 5, 2, 4] {
        list.delete(&id).unwrap();
        remaining.retain(|other| *other != id);
        assert_links(&list, &remaining);
    }
}

/// Should not delete the same item twice.
#[test]
fn lexical_list_delete_twice() {
    let mut list = create(&[1, 2]);
    list.delete(&1).unwrap();
    assert_lexical_error(list.delete(&1), "not-found");
    assert_links(&list, &[2]);
}

/// Should keep appending after the new last item once the tail is deleted.
#[test]
fn lexical_list_push_after_delete() {
    let mut list = create(&[1, 2, 3]);
    list.delete(&3).unwrap();
    let item = list.push(value(4)).unwrap();
    assert_eq!(item.position().to_string(), position(2));
    assert_links(&list, &[1, 2, 4]);
}

/// Should accept new items after the list was emptied.
#[test]
fn lexical_list_push_after_emptied() {
    let mut list = create(&[1, 2]);
    list.delete(&1).unwrap();
    list.delete(&2).unwrap();
    list.push(value(3)).unwrap();
    list.push(value(4)).unwrap();
    assert_links(&list, &[3, 4]);
}

// Move

/// Should fail to move an item that does not exist.
#[test]
fn lexical_list_move_missing() {
    let mut list = create(&[1, 2]);
    assert_lexical_error(list.move_to(&3, Some(&1), Some(&2)), "not-found");
    assert_links(&list, &[1, 2]);
}

/// Should fail to move when no neighbor is provided.
#[test]
fn lexical_list_move_without_neighbors() {
    let mut list = create(&[1, 2]);
    assert_lexical_error(list.move_to(&1, None, None), "position");
    assert_links(&list, &[1, 2]);
}

/// Should fail to move when a provided neighbor does not exist.
#[test]
fn lexical_list_move_missing_neighbor() {
    let mut list = create(&[1, 2, 3]);
    assert_lexical_error(list.move_to(&1, Some(&5), Some(&6)), "not-found");
    assert_lexical_error(list.move_to(&1, Some(&3), Some(&6)), "not-found");
    assert_lexical_error(list.move_to(&3, Some(&5), Some(&1)), "not-found");
    assert_links(&list, &[1, 2, 3]);
}

/// Should fail to move an item next to itself.
#[test]
fn lexical_list_move_next_to_itself() {
    let mut list = create(&[1, 2, 3]);
    assert_lexical_error(list.move_to(&2, Some(&2), Some(&3)), "not-adjacent");
    assert_lexical_error(list.move_to(&2, Some(&1), Some(&2)), "not-adjacent");
    assert_links(&list, &[1, 2, 3]);
}

/// Should fail to move between two items that are not adjacent.
#[test]
fn lexical_list_move_not_adjacent() {
    let mut list = create(&[1, 2, 3, 4]);
    let before = positions(&list);
    assert_lexical_error(list.move_to(&4, Some(&1), Some(&3)), "not-adjacent");
    assert_lexical_error(list.move_to(&1, Some(&4), Some(&2)), "not-adjacent");
    assert_lexical_error(list.move_to(&2, Some(&3), Some(&1)), "not-adjacent");
    assert_links(&list, &[1, 2, 3, 4]);
    assert_eq!(
        positions(&list),
        before,
        "A failed move must not change any position."
    );
}

/// Should fail to move to an end when the single neighbor is not at that end.
#[test]
fn lexical_list_move_single_neighbor_not_at_end() {
    let mut list = create(&[1, 2, 3]);
    assert_lexical_error(list.move_to(&3, Some(&1), None), "not-adjacent");
    assert_lexical_error(list.move_to(&1, None, Some(&3)), "not-adjacent");
    assert_links(&list, &[1, 2, 3]);
}

/// Should accept neighbors that are only adjacent once the item is unlinked.
#[test]
fn lexical_list_move_adjacent_after_unlink() {
    let mut list = create(&[1, 2, 3, 4]);
    list.move_to(&2, Some(&3), Some(&4)).unwrap();
    assert_links(&list, &[1, 3, 2, 4]);
    let mut other = create(&[1, 2, 3]);
    other.move_to(&1, Some(&3), None).unwrap();
    assert_links(&other, &[2, 3, 1]);
}

/// Should not change anything when moving an item to its current place.
#[test]
fn lexical_list_move_same_place() {
    let mut list = create(&[1, 2, 3]);
    let position = list.find(&2).unwrap().position().to_string();
    list.move_to(&2, Some(&1), Some(&3)).unwrap();
    list.move_to(&1, None, Some(&2)).unwrap();
    list.move_to(&3, Some(&2), None).unwrap();
    assert_eq!(
        list.find(&2).unwrap().position().to_string(),
        position,
        "The position must not change."
    );
    assert_links(&list, &[1, 2, 3]);
}

/// Should treat null neighbor ids as no neighbor (`None` in Rust).
#[test]
fn lexical_list_move_null_neighbors() {
    let mut list = create(&[1, 2, 3]);
    list.move_to(&3, None, Some(&1)).unwrap();
    assert_links(&list, &[3, 1, 2]);
    list.move_to(&3, Some(&2), None).unwrap();
    assert_links(&list, &[1, 2, 3]);
}

/// Should move the last item to the front.
#[test]
fn lexical_list_move_last_to_front() {
    let mut list = create(&[1, 2, 3]);
    let item = list.move_to(&3, None, Some(&1)).unwrap();
    assert_eq!((item.previous(), item.next()), (None, Some(&1)));
    assert_links(&list, &[3, 1, 2]);
}

/// Should move the first item to the end.
#[test]
fn lexical_list_move_first_to_end() {
    let mut list = create(&[1, 2, 3]);
    let item = list.move_to(&1, Some(&3), None).unwrap();
    assert_eq!((item.previous(), item.next()), (Some(&3), None));
    assert_links(&list, &[2, 3, 1]);
}

/// Should move the first item between two others.
#[test]
fn lexical_list_move_first_to_middle() {
    let mut list = create(&[1, 2, 3]);
    list.move_to(&1, Some(&2), Some(&3)).unwrap();
    assert_links(&list, &[2, 1, 3]);
}

/// Should move the last item between two others.
#[test]
fn lexical_list_move_last_to_middle() {
    let mut list = create(&[1, 2, 3]);
    list.move_to(&3, Some(&1), Some(&2)).unwrap();
    assert_links(&list, &[1, 3, 2]);
}

/// Should move a middle item to both ends.
#[test]
fn lexical_list_move_middle_to_ends() {
    let mut list = create(&[1, 2, 3, 4]);
    list.move_to(&2, None, Some(&1)).unwrap();
    assert_links(&list, &[2, 1, 3, 4]);
    list.move_to(&3, Some(&4), None).unwrap();
    assert_links(&list, &[2, 1, 4, 3]);
}

/// Should swap two items in a list of two.
#[test]
fn lexical_list_move_swap_pair() {
    let mut list = create(&[1, 2]);
    list.move_to(&2, None, Some(&1)).unwrap();
    assert_links(&list, &[2, 1]);
    list.move_to(&2, Some(&1), None).unwrap();
    assert_links(&list, &[1, 2]);
}

/// Should keep the list consistent after many consecutive moves.
#[test]
fn lexical_list_move_many() {
    let mut list = create(&[1, 2, 3, 4, 5]);
    let mut expected = vec![1, 2, 3, 4, 5];
    let moves: [(u32, Option<u32>, Option<u32>); 6] = [
        (5, None, Some(1)),
        (1, Some(4), None),
        (3, Some(5), Some(2)),
        (2, None, Some(5)),
        (4, Some(5), Some(3)),
        (1, Some(2), Some(5)),
    ];
    for (id, previous, next) in moves {
        list.move_to(&id, previous.as_ref(), next.as_ref()).unwrap();
        expected.retain(|other| *other != id);
        let index = previous.map_or(0, |previous| {
            expected
                .iter()
                .position(|other| *other == previous)
                .unwrap()
                + 1
        });
        expected.insert(index, id);
        assert_links(&list, &expected);
    }
}

/// Should insert repeatedly at the same gap without breaking the order.
#[test]
fn lexical_list_move_same_gap() {
    let ids: Vec<u32> = (1..=12).collect();
    let mut list = create(&ids);
    let mut expected = ids.clone();
    for _ in 0..10 {
        let last = *list.last().unwrap().id();
        let (first, second) = (expected[0], expected[1]);
        list.move_to(&last, Some(&first), Some(&second)).unwrap();
        expected.retain(|other| *other != last);
        expected.insert(1, last);
        assert_links(&list, &expected);
    }
}

/// Should keep the selection pointing to the moved item.
#[test]
fn lexical_list_move_keeps_selection() {
    let mut list = create(&[1, 2, 3]);
    list.select(&1);
    list.move_to(&1, Some(&3), None).unwrap();
    assert_eq!(list.selected().map(|item| *item.id()), Some(1));
    assert_eq!(
        list.selected().map(|item| *item.id()),
        list.last().map(|item| *item.id())
    );
}

// Set

/// Should build the list from an array of items.
#[test]
fn lexical_list_set() {
    let mut list = new();
    list.set(vec![
        stored(1, "aaaa"),
        stored(2, "bbbb"),
        stored(3, "cccc"),
    ])
    .unwrap();
    assert_links(&list, &[1, 2, 3]);
    assert_eq!(list.find(&2).unwrap().position().to_string(), "bbbb!");
}

/// Should return no affected items and the list itself on trusted input.
#[test]
fn lexical_list_set_trusted_result() {
    let mut list = new();
    let affected = list
        .set(vec![stored(1, "aaaa"), stored(2, "bbbb")])
        .unwrap();
    assert!(affected.is_empty());
    assert_eq!(list.len(), 2);
}

/// Should reject positions out of order keeping the current content.
#[test]
fn lexical_list_set_trusted_not_ordered() {
    let mut list = create(&[1, 2]);
    let values = vec![stored(7, "aaaa"), stored(8, "cccc"), stored(9, "bbbb")];
    let rejected = list.set(values.clone()).unwrap_err();
    assert_eq!(kind(&rejected.error), "not-ordered");
    assert_links(&list, &[1, 2]);
    assert_eq!(rejected.value, values, "The values must not be modified.");
}

/// Should reject equal positions when trusted.
#[test]
fn lexical_list_set_trusted_equal() {
    let mut list = create(&[1]);
    assert_lexical_error(
        list.set(vec![stored(7, "bbbb"), stored(8, "bbbb!!")]),
        "not-ordered",
    );
    assert_links(&list, &[1]);
}

/// Should reject empty or invalid positions when trusted keeping the current content.
#[test]
fn lexical_list_set_trusted_invalid_position() {
    let mut list = create(&[1]);
    let values = vec![stored(7, "aaaa"), stored(8, "")];
    let rejected = list.set(values.clone()).unwrap_err();
    assert_eq!(kind(&rejected.error), "position");
    assert_lexical_error(
        list.set(vec![stored(7, "aaaa"), stored(8, "bbbbzzzzzzzzzz")]),
        "position",
    );
    assert_links(&list, &[1]);
    assert_eq!(
        rejected.value[0].position, "aaaa",
        "The values must not be modified."
    );
}

/// Should reject positions with characters of another system when trusted.
#[test]
fn lexical_list_set_trusted_foreign_characters() {
    let mut list = create(&[1]);
    assert_lexical_error(
        list.set(vec![stored(7, "aaaa"), stored(8, "¼¼¼¼¿!")]),
        "position",
    );
    assert_links(&list, &[1]);
}

/// Should repair positions with characters of another system when untrusted.
#[test]
fn lexical_list_set_untrusted_foreign_characters() {
    let mut list = new();
    let affected = ids(&list
        .set_untrusted(vec![
            stored(1, "¼¼¼¼¼!"),
            stored(2, "bbbb"),
            stored(3, "aaAa"),
        ])
        .unwrap());
    assert_eq!(affected, [1, 3]);
    assert_links(&list, &[2, 1, 3]);
}

/// Should not affect any item on sorted untrusted input.
#[test]
fn lexical_list_set_untrusted_sorted() {
    let mut list = new();
    let affected = ids(&list
        .set_untrusted(vec![stored(1, "aaaa"), stored(2, "bbbb")])
        .unwrap());
    assert!(affected.is_empty());
    assert_links(&list, &[1, 2]);
}

/// Should sort untrusted input by position without changing any position.
#[test]
fn lexical_list_set_untrusted_out_of_order() {
    let mut list = new();
    let rows = vec![
        stored(1, "cccc"),
        stored(2, "aaaa"),
        stored(3, "dddd"),
        stored(4, "bbbb"),
    ];
    let affected = ids(&list.set_untrusted(rows).unwrap());
    assert!(affected.is_empty());
    assert_links(&list, &[2, 4, 1, 3]);
}

/// Should keep the first equal position and repair the others right after it.
#[test]
fn lexical_list_set_untrusted_equal() {
    let mut list = new();
    let rows = vec![
        stored(1, "aaaa"),
        stored(2, "bbbb"),
        stored(3, "bbbb"),
        stored(4, "bbbb!!"),
        stored(5, "cccc"),
    ];
    let affected = list.set_untrusted(rows).unwrap();
    assert_eq!(ids(&affected), [3, 4]);
    for item in &affected {
        assert_eq!(item.value().position, item.position().to_string());
    }
    assert_links(&list, &[1, 2, 3, 4, 5]);
}

/// Should append empty and invalid positions at the end in the incoming order.
#[test]
fn lexical_list_set_untrusted_missing_positions() {
    let mut list = new();
    let rows = vec![
        stored(1, ""),
        stored(2, "bbbb"),
        stored(3, "bbbbzzzzzzzzzz"),
        stored(4, "aaaa"),
    ];
    let affected = ids(&list.set_untrusted(rows).unwrap());
    assert_eq!(affected, [1, 3]);
    assert_links(&list, &[4, 2, 1, 3]);
}

/// Should give positions from the start when none is valid.
#[test]
fn lexical_list_set_untrusted_no_positions() {
    let mut list = new();
    let affected = ids(&list
        .set_untrusted(vec![stored(1, ""), stored(2, ""), stored(3, "")])
        .unwrap());
    assert_eq!(affected, [1, 2, 3]);
    assert_links(&list, &[1, 2, 3]);
    assert_eq!(list.first().unwrap().position().to_string(), position(0));
}

/// Should repair many equal positions keeping them short.
#[test]
fn lexical_list_set_untrusted_many_equal() {
    let mut rows: Vec<Value> = (0..=30).map(|id| stored(id, "bbbb")).collect();
    rows.push(stored(31, "bbbc"));
    let expected: Vec<u32> = rows.iter().map(|row| row.id).collect();
    let mut list = new();
    assert_eq!(list.set_untrusted(rows).unwrap().len(), 30);
    assert_links(&list, &expected);
}

/// Should space every position again when there is no room to repair.
#[test]
fn lexical_list_set_untrusted_respace() {
    let mut list = new();
    let affected = ids(&list
        .set_untrusted(vec![stored(1, "zzzz"), stored(2, "zzzz")])
        .unwrap());
    assert_links(&list, &[1, 2]);
    assert_eq!(list.first().unwrap().position().to_string(), position(0));
    assert_eq!(affected, [1, 2]);
}

/// Should still reject invalid or duplicated ids when untrusted. Rust ids can't be null, so only the
/// duplicated ids are checked.
#[test]
fn lexical_list_set_untrusted_invalid_ids() {
    let mut list = create(&[1]);
    assert_lexical_error(
        list.set_untrusted(vec![stored(7, "aaaa"), stored(7, "bbbb")]),
        "duplicate-id",
    );
    assert_links(&list, &[1]);
}

/// Should replace the previous content when setting a new array.
#[test]
fn lexical_list_set_replaces() {
    let mut list = create(&[1, 2, 3]);
    list.set(vec![stored(7, "aaaa"), stored(8, "bbbb")])
        .unwrap();
    assert_links(&list, &[7, 8]);
    assert!(list.find(&1).is_none());
}

/// Should empty the list when setting an empty array.
#[test]
fn lexical_list_set_empty() {
    let mut list = create(&[1, 2, 3]);
    list.set(Vec::new()).unwrap();
    assert_links(&list, &[]);
}

/// Should reject an array with a null or undefined id keeping the current content. Rust ids can't be null:
/// the closest is a row that can't be trusted, which also keeps the content.
#[test]
fn lexical_list_set_invalid_id() {
    let mut list = create(&[1, 2]);
    assert_lexical_error(list.set(vec![stored(7, "aaaa"), stored(8, "")]), "position");
    assert_links(&list, &[1, 2]);
}

/// Should allow push, delete and move after setting an array.
#[test]
fn lexical_list_set_then_mutate() {
    let mut list = new();
    list.set(vec![stored(1, "aaaa"), stored(2, "bbbb")])
        .unwrap();
    list.push(value(3)).unwrap();
    assert_links(&list, &[1, 2, 3]);
    list.move_to(&3, None, Some(&1)).unwrap();
    assert_links(&list, &[3, 1, 2]);
    list.delete(&1).unwrap();
    assert_links(&list, &[3, 2]);
}

/// Should clear every item from the list.
#[test]
fn lexical_list_clear() {
    let mut list = create(&[1, 2, 3]);
    list.clear();
    assert_links(&list, &[]);
    list.push(value(4)).unwrap();
    assert_links(&list, &[4]);
}

// Sort

/// Should sort an empty list without errors.
#[test]
fn lexical_list_sort_empty() {
    let mut list = new();
    assert!(list.sort_by(|a, b| a.id.cmp(&b.id)).unwrap().is_empty());
    assert_links(&list, &[]);
}

/// Should not affect any item when the list is already sorted.
#[test]
fn lexical_list_sort_already_sorted() {
    let mut list = create(&[1, 2, 3]);
    assert!(list.sort_by(|a, b| a.id.cmp(&b.id)).unwrap().is_empty());
    assert_links(&list, &[1, 2, 3]);
}

/// Should reverse the list, relinking items and reporting changed positions.
#[test]
fn lexical_list_sort_reverse() {
    let mut list = create(&[1, 2, 3]);
    let affected = ids(&list.sort_by(|a, b| b.id.cmp(&a.id)).unwrap());
    assert_eq!(affected, [3, 1]);
    assert_links(&list, &[3, 2, 1]);
    for (index, id) in [3, 2, 1].iter().enumerate() {
        assert_eq!(
            list.find(id).unwrap().position().to_string(),
            position(index)
        );
    }
}

/// Should normalize positions after moves when sorting by position.
#[test]
fn lexical_list_sort_after_moves() {
    let mut list = create(&[1, 2, 3, 4]);
    list.move_to(&4, Some(&1), Some(&2)).unwrap();
    list.move_to(&1, Some(&3), None).unwrap();
    assert_links(&list, &[4, 2, 3, 1]);
    sort_by_position(&mut list);
    assert_links(&list, &[4, 2, 3, 1]);
    for (index, id) in [4, 2, 3, 1].iter().enumerate() {
        assert_eq!(
            list.find(id).unwrap().position().to_string(),
            position(index)
        );
    }
}

/// Should not change the list nor the objects when the system has no room to sort.
#[test]
fn lexical_list_sort_without_room() {
    // N has one digit from "a" to "c": sorting has room for "b!" and "c!" only
    let mut list = LexicalList::new(Staquia::create('!', 'a', 'c', 1, 3, 0.0).unwrap());
    list.set(vec![stored(1, "a!"), stored(2, "b!"), stored(3, "c!")])
        .unwrap();
    assert_lexical_error(list.sort_by(|a, b| b.id.cmp(&a.id)), "position");
    assert_links(&list, &[1, 2, 3]);
    assert_eq!(positions(&list), ["a!", "b!", "c!"]);
}

/// Should sort exactly as many items as the system has room for.
#[test]
fn lexical_list_sort_up_to_the_room() {
    for (first, last, ln) in [('a', 'c', 1), ('a', 'c', 2), ('a', 'd', 2), ('a', 'b', 3)] {
        let system = Staquia::create('!', first, last, ln, 3, 0.0).unwrap();
        // The room: the positions stacked one by one from the start
        let mut room = 1;
        let mut position = system.start().unwrap();
        while let Ok(next) = system.next(&position) {
            position = next;
            room += 1;
        }
        let create = |count: u32| {
            let mut list = LexicalList::new(system);
            for id in 1..=count {
                list.push(value(id)).unwrap();
            }
            list
        };
        let mut fits = create(room);
        fits.sort_by(|a, b| b.id.cmp(&a.id)).unwrap();
        assert_eq!(
            fits.first().map(|item| *item.id()),
            Some(room),
            "{room} items from {first} to {last} with ln {ln}."
        );

        // One more item fits with `unshift` (which stacks before the start) but not in a sort
        let mut unshifted = create(room);
        unshifted.unshift(value(room + 1)).unwrap();
        let before = positions(&unshifted);
        assert_lexical_error(unshifted.sort_by(|a, b| b.id.cmp(&a.id)), "position");
        assert_eq!(
            positions(&unshifted),
            before,
            "{} items from {first} to {last} with ln {ln}.",
            room + 1
        );
    }
}

// Rebalance

/// Should rebalance keeping the order and report only the changed positions.
#[test]
fn lexical_list_rebalance() {
    let mut list = create(&[1, 2, 3]);
    list.move_to(&3, Some(&1), Some(&2)).unwrap();
    assert_links(&list, &[1, 3, 2]);
    let affected = ids(&list.rebalance().unwrap());
    // The first item keeps the start position so it must not be reported.
    assert_eq!(affected, [3, 2]);
    assert_links(&list, &[1, 3, 2]);
    for (index, id) in [1, 3, 2].iter().enumerate() {
        assert_eq!(
            list.find(id).unwrap().position().to_string(),
            position(index)
        );
    }
}

/// Should not change the list nor the objects when the system has no room to rebalance.
#[test]
fn lexical_list_rebalance_without_room() {
    let mut list = LexicalList::new(Staquia::create('!', 'a', 'c', 1, 3, 0.0).unwrap());
    list.set(vec![stored(1, "a!"), stored(2, "b!"), stored(3, "c!")])
        .unwrap();
    assert_lexical_error(list.rebalance(), "position");
    assert_links(&list, &[1, 2, 3]);
    assert_eq!(positions(&list), ["a!", "b!", "c!"]);
}

// Destroy

/// Should release every reference when destroyed (`clear` in Rust, the list frees its values when dropped).
#[test]
fn lexical_list_destroy() {
    let mut list = create(&[1, 2]);
    list.select(&1);
    list.clear();
    assert!(list.first().is_none());
    assert!(list.last().is_none());
    assert!(list.selected().is_none());
}

// Falsy ids

/// Should delete the item after an item with id 0.
#[test]
fn lexical_list_delete_after_zero_id() {
    let mut list = create(&[0, 1, 2]);
    list.delete(&1).unwrap();
    assert_links(&list, &[0, 2]);
}

/// Should delete the item before an item with id 0.
#[test]
fn lexical_list_delete_before_zero_id() {
    let mut list = create(&[1, 2, 0]);
    list.delete(&2).unwrap();
    assert_links(&list, &[1, 0]);
}

/// Should move an item next to an item with id 0.
#[test]
fn lexical_list_move_around_zero_id() {
    let mut list = create(&[0, 1, 2]);
    list.move_to(&2, Some(&0), Some(&1)).unwrap();
    assert_links(&list, &[0, 2, 1]);
    list.move_to(&0, Some(&1), None).unwrap();
    assert_links(&list, &[2, 1, 0]);
}

/// Should work with string ids, including the empty string.
#[test]
fn lexical_list_string_ids() {
    struct Named {
        id: String,
        position: String,
    }
    impl Item for Named {
        type Id = String;
        fn id(&self) -> &String {
            &self.id
        }
        fn position(&self) -> &str {
            &self.position
        }
        fn set_position(&mut self, position: &Position) {
            self.position = position.to_string();
        }
    }
    let id = |id: &str| id.to_owned();
    let mut list: LexicalList<Named> = LexicalList::new(system());
    for name in ["", "a", "b", "c"] {
        list.push(Named {
            id: id(name),
            position: String::new(),
        })
        .unwrap();
    }
    list.delete(&id("a")).unwrap();
    list.move_to(&id(""), Some(&id("b")), Some(&id("c")))
        .unwrap();
    list.unshift(Named {
        id: id("d"),
        position: String::new(),
    })
    .unwrap();
    let order: Vec<&str> = list.iter().map(|item| item.id().as_str()).collect();
    assert_eq!(order, ["d", "b", "", "c"]);
    assert_eq!(list.first().unwrap().id(), "d");
    assert_eq!(list.last().unwrap().id(), "c");
    assert_eq!(list.find(&id("")).unwrap().previous(), Some(&id("b")));
    assert_eq!(list.find(&id("")).unwrap().next(), Some(&id("c")));
}

// Persistence

/// Should write the new position back into the wrapped object on move.
#[test]
fn lexical_list_move_writes_back() {
    let mut list = create(&[1, 2, 3]);
    let position = list
        .move_to(&3, Some(&1), Some(&2))
        .unwrap()
        .position()
        .to_string();
    assert_eq!(list.find(&3).unwrap().value().position, position);
}

/// Should write the new positions back into the wrapped objects on sort.
#[test]
fn lexical_list_sort_writes_back() {
    let mut list = create(&[1, 2, 3]);
    list.sort_by(|a, b| b.id.cmp(&a.id)).unwrap();
    for item in &list {
        assert_eq!(item.value().position, item.position().to_string());
    }
}

/// Should only change the position of the moved item.
#[test]
fn lexical_list_move_changes_single_position() {
    let mut list = create(&[1, 2, 3, 4, 5]);
    let stored = |list: &List| {
        (1..=5)
            .map(|id| list.find(&id).unwrap().value().position.clone())
            .collect::<Vec<_>>()
    };
    let moves: [(u32, Option<u32>, Option<u32>); 4] = [
        (5, Some(1), Some(2)), // 1 5 2 3 4
        (1, Some(4), None),    // 5 2 3 4 1
        (3, Some(4), Some(1)), // 5 2 4 3 1
        (2, None, Some(5)),    // 2 5 4 3 1
    ];
    for (id, previous, next) in moves {
        let before = stored(&list);
        list.move_to(&id, previous.as_ref(), next.as_ref()).unwrap();
        let after = stored(&list);
        for (index, other) in (1..=5).enumerate() {
            if other == id {
                assert_ne!(
                    after[index], before[index],
                    "The moved item (id: {id}) must change its position."
                );
            } else {
                assert_eq!(
                    after[index], before[index],
                    "The item (id: {other}) must keep its position when moving the item (id: {id})."
                );
            }
        }
    }
}

/// Should rebuild the same list from the stored positions sorted as strings.
#[test]
fn lexical_list_rebuild_from_stored_positions() {
    let mut list = create(&[1, 2, 3, 4, 5, 6]);
    list.move_to(&6, Some(&1), Some(&2)).unwrap(); // 1 6 2 3 4 5
    list.move_to(&2, None, Some(&1)).unwrap(); // 2 1 6 3 4 5
    list.unshift(value(7)).unwrap(); // 7 2 1 6 3 4 5
    list.delete(&4).unwrap(); // 7 2 1 6 3 5
    list.move_to(&7, Some(&3), Some(&5)).unwrap(); // 2 1 6 3 7 5
    list.move_to(&1, Some(&6), Some(&3)).unwrap(); // 2 6 1 3 7 5
    let expected = [2, 6, 1, 3, 7, 5];
    assert_links(&list, &expected);
    let mut records: Vec<Value> = list.iter().map(|item| item.value().clone()).collect();
    records.sort_by(|a, b| a.position.cmp(&b.position));
    let mut reloaded = new();
    reloaded.set(records).unwrap();
    assert_links(&reloaded, &expected);
}

// Threshold

/// Should fail without changes when a gap is exhausted and recover after sorting.
#[test]
fn lexical_list_threshold_and_rebalance() {
    let mut list = create(&[1, 2, 3]);
    let mut order = vec![1, 2, 3];
    let mut failure = None;
    for _ in 0..500 {
        let last = *list.last().unwrap().id();
        let first = *list.first().unwrap().id();
        let second = *list.first().unwrap().next().unwrap();
        let before = positions(&list);
        match list.move_to(&last, Some(&first), Some(&second)) {
            Err(error) => {
                assert_links(&list, &order);
                assert_eq!(
                    positions(&list),
                    before,
                    "A failed move must not change any position."
                );
                failure = Some(error);
                break;
            }
            Ok(_) => {
                order.retain(|id| *id != last);
                order.insert(1, last);
                assert_links(&list, &order);
            }
        }
    }
    let failure = failure.expect("The gap was never exhausted.");
    assert_eq!(kind(&failure), "position");
    sort_by_position(&mut list);
    assert_links(&list, &order);
    let last = *list.last().unwrap().id();
    let first = *list.first().unwrap().id();
    let second = *list.first().unwrap().next().unwrap();
    list.move_to(&last, Some(&first), Some(&second)).unwrap();
}

/// Should fail without changes when pushing past the end of N and recover after sorting.
#[test]
fn lexical_list_push_past_n_end() {
    let mut list = new();
    list.set(vec![stored(1, "zzzy")]).unwrap();
    list.push(value(2)).unwrap();
    assert_eq!(list.last().unwrap().position().to_string(), "zzzz!");
    assert_lexical_error(list.push(value(3)), "position");
    assert_links(&list, &[1, 2]);
    sort_by_position(&mut list);
    list.push(value(3)).unwrap();
    assert_links(&list, &[1, 2, 3]);
}

/// Should fail without changes when unshifting past the start of N and recover after sorting.
#[test]
fn lexical_list_unshift_past_n_start() {
    let mut list = new();
    list.set(vec![stored(1, "!!!a")]).unwrap();
    list.unshift(value(2)).unwrap();
    assert_eq!(list.first().unwrap().position().to_string(), "!!!!!");
    assert_lexical_error(list.unshift(value(3)), "position");
    assert_links(&list, &[2, 1]);
    sort_by_position(&mut list);
    list.unshift(value(3)).unwrap();
    assert_links(&list, &[3, 2, 1]);
}

// Randomized

/// The seeded random generator of the JavaScript test (mulberry32).
struct Random(i32);

impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6d2b79f5);
        let seed = self.0;
        let mut t = (seed ^ ((seed as u32) >> 15) as i32).wrapping_mul(1 | seed);
        t = t.wrapping_add((t ^ ((t as u32) >> 7) as i32).wrapping_mul(61 | t)) ^ t;
        f64::from((t ^ ((t as u32) >> 14) as i32) as u32) / 4_294_967_296.0
    }

    fn pick(&mut self, values: &[u32]) -> u32 {
        values[(self.next() * values.len() as f64) as usize]
    }
}

/// Should keep the list consistent against a reference array on random operations.
#[test]
fn lexical_list_randomized() {
    let mut random = Random(0x5eed);
    let mut list = new();
    let mut reference: Vec<u32> = Vec::new();
    let mut next_id = 0;
    for step in 0..400 {
        let roll = random.next();
        let operation;
        if reference.len() < 2 || roll < 0.25 {
            operation = "push";
            list.push(value(next_id)).unwrap();
            reference.push(next_id);
            next_id += 1;
        } else if roll < 0.45 {
            operation = "unshift";
            list.unshift(value(next_id)).unwrap();
            reference.insert(0, next_id);
            next_id += 1;
        } else if roll < 0.6 {
            operation = "delete";
            let id = random.pick(&reference);
            list.delete(&id).unwrap();
            reference.retain(|other| *other != id);
        } else {
            operation = "move";
            let id = random.pick(&reference);
            let others: Vec<u32> = reference
                .iter()
                .copied()
                .filter(|other| *other != id)
                .collect();
            let gap = (random.next() * (others.len() + 1) as f64) as usize;
            let previous = gap.checked_sub(1).map(|k| others[k]);
            let next = others.get(gap).copied();
            match list.move_to(&id, previous.as_ref(), next.as_ref()) {
                Err(error) => {
                    assert_eq!(
                        kind(&error),
                        "position",
                        "Step {step}: unexpected '{error}'."
                    );
                    assert_links(&list, &reference);
                    sort_by_position(&mut list);
                }
                Ok(_) => {
                    reference = others[..gap]
                        .iter()
                        .copied()
                        .chain([id])
                        .chain(others[gap..].iter().copied())
                        .collect();
                }
            }
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            assert_links(&list, &reference)
        }));
        assert!(result.is_ok(), "Step {step} ({operation}) broke the list.");
    }
}

// Integrity

/// Should reject pushing an id that already exists.
#[test]
fn lexical_list_push_duplicate_id() {
    let mut list = create(&[1, 2]);
    assert_lexical_error(list.push(value(1)), "duplicate-id");
    assert_links(&list, &[1, 2]);
}

/// Should reject unshifting an id that already exists.
#[test]
fn lexical_list_unshift_duplicate_id() {
    let mut list = create(&[1, 2]);
    assert_lexical_error(list.unshift(value(2)), "duplicate-id");
    assert_links(&list, &[1, 2]);
}

/// Should reject setting an array with duplicated ids keeping the current content.
#[test]
fn lexical_list_set_duplicate_id() {
    let mut list = create(&[1, 2]);
    assert_lexical_error(
        list.set(vec![stored(7, "aaaa"), stored(7, "bbbb")]),
        "duplicate-id",
    );
    assert_links(&list, &[1, 2]);
}

/// Should accept an id again once its item was deleted.
#[test]
fn lexical_list_reuse_deleted_id() {
    let mut list = create(&[1, 2]);
    list.delete(&1).unwrap();
    list.push(value(1)).unwrap();
    assert_links(&list, &[2, 1]);
}

/// Should clear the selection when the selected item is deleted.
#[test]
fn lexical_list_delete_clears_selection() {
    let mut list = create(&[1, 2]);
    list.select(&2);
    list.delete(&2).unwrap();
    assert!(list.selected().is_none());
}

/// Should keep the selection when another item is deleted.
#[test]
fn lexical_list_delete_keeps_other_selection() {
    let mut list = create(&[1, 2]);
    list.select(&2);
    list.delete(&1).unwrap();
    assert_eq!(list.selected().map(|item| *item.id()), Some(2));
}

/// Should clear the selection when the list is cleared or replaced.
#[test]
fn lexical_list_clear_and_set_clear_selection() {
    let mut list = create(&[1, 2]);
    list.select(&1);
    list.clear();
    assert!(
        list.selected().is_none(),
        "clear() must remove the selection."
    );
    list.push(value(1)).unwrap();
    list.select(&1);
    list.set(vec![stored(1, "aaaa")]).unwrap();
    assert!(
        list.selected().is_none(),
        "set() must remove the selection."
    );
}
