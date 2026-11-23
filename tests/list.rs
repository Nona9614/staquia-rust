//! The `LexicalList` behavior not covered by the cross-language fixtures, ported from the JavaScript
//! `lexical-list.spec.ts`.

use staquia::{Item, LexicalError, LexicalList, Position, PositionError, Staquia};

#[derive(Debug, Clone, PartialEq)]
struct Task {
    id: u32,
    title: &'static str,
    position: String,
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

fn task(id: u32) -> Task {
    Task {
        id,
        title: "",
        position: String::new(),
    }
}

fn stored(id: u32, position: &str) -> Task {
    Task {
        id,
        title: "",
        position: position.to_owned(),
    }
}

fn system() -> Staquia {
    Staquia::create('!', 'a', 'z', 5, 15, 0.15).unwrap()
}

fn list(ids: &[u32]) -> LexicalList<Task> {
    let mut list = LexicalList::new(system());
    for id in ids {
        list.push(task(*id)).unwrap();
    }
    list
}

fn order(list: &LexicalList<Task>) -> Vec<u32> {
    list.iter().map(|item| item.value().id).collect()
}

/// The links in both directions agree with the iteration order.
fn assert_consistent(list: &LexicalList<Task>) {
    let items: Vec<_> = list.iter().collect();
    assert_eq!(items.len(), list.len());
    for (k, item) in items.iter().enumerate() {
        assert_eq!(item.previous(), k.checked_sub(1).map(|k| items[k].id()));
        assert_eq!(item.next(), items.get(k + 1).map(|next| next.id()));
        assert_eq!(item.value().position, item.position().to_string());
        if k > 0 {
            assert!(items[k - 1].position() < item.position());
        }
    }
    assert_eq!(
        list.first().map(|item| item.id()),
        items.first().map(|item| item.id())
    );
    assert_eq!(
        list.last().map(|item| item.id()),
        items.last().map(|item| item.id())
    );
}

#[test]
fn a_new_list_is_empty() {
    let list: LexicalList<Task> = LexicalList::new(system());
    assert!(list.is_empty());
    assert!(list.first().is_none() && list.last().is_none() && list.selected().is_none());
    assert_eq!(list.head().unwrap().to_string(), "mmmmm!");
    assert_eq!(list.tail().unwrap().to_string(), "mmmmm!");
}

#[test]
fn push_and_unshift_write_the_positions_into_the_values() {
    let mut list = list(&[1, 2]);
    list.unshift(task(0)).unwrap();
    let pushed = list.push(stored(3, "ignored")).unwrap();
    assert_eq!(pushed.value().position, "mmmmo!");
    let positions: Vec<&str> = list
        .iter()
        .map(|item| item.value().position.as_str())
        .collect();
    assert_eq!(positions, ["mmmml!", "mmmmm!", "mmmmn!", "mmmmo!"]);
    assert_eq!(order(&list), [0, 1, 2, 3]);
    assert_consistent(&list);
}

#[test]
fn a_rejected_value_is_given_back() {
    let mut list = list(&[1]);
    let rejected = list
        .push(Task {
            id: 1,
            title: "again",
            position: String::new(),
        })
        .unwrap_err();
    assert_eq!(rejected.error, LexicalError::DuplicateId(1));
    assert_eq!(rejected.into_value().title, "again");
    let rejected = list.unshift(task(1)).unwrap_err();
    assert_eq!(rejected.error, LexicalError::DuplicateId(1));
    // Converts into the crate error with `?`
    let error: staquia::Error<u32> = list.push(task(1)).unwrap_err().into();
    assert_eq!(error, staquia::Error::Lexical(LexicalError::DuplicateId(1)));
}

#[test]
fn find_at_and_contains() {
    let list = list(&[10, 20, 30]);
    assert_eq!(list.find(&20).unwrap().value().id, 20);
    assert!(list.find(&40).is_none());
    assert!(list.contains(&30) && !list.contains(&0));
    assert_eq!(list.at(0).unwrap().id(), &10);
    assert_eq!(list.at(2).unwrap().id(), &30);
    assert!(list.at(3).is_none());
    assert_eq!((&list).into_iter().len(), 3);
}

#[test]
fn find_mut_changes_the_value_in_place() {
    let mut list = list(&[1]);
    list.find_mut(&1).unwrap().title = "renamed";
    assert_eq!(list.find(&1).unwrap().value().title, "renamed");
}

#[test]
fn selection_follows_the_item() {
    let mut list = list(&[1, 2, 3]);
    assert_eq!(list.select(&2).unwrap().id(), &2);
    list.move_to(&2, Some(&3), None).unwrap();
    assert_eq!(list.selected().unwrap().id(), &2);
    list.delete(&1).unwrap();
    assert_eq!(list.selected().unwrap().id(), &2);
    list.delete(&2).unwrap();
    assert!(list.selected().is_none());
    assert!(list.select(&7).is_none());
    list.select(&3);
    list.deselect();
    assert!(list.selected().is_none());
    list.select(&3);
    list.clear();
    assert!(list.selected().is_none() && list.is_empty());
}

#[test]
fn delete_gives_the_value_back_and_links_the_neighbors() {
    let mut list = list(&[1, 2, 3, 4]);
    assert_eq!(list.delete(&9), Err(LexicalError::NotFound(9)));
    assert_eq!(list.delete(&2).unwrap().id, 2);
    assert_eq!(order(&list), [1, 3, 4]);
    list.delete(&1).unwrap();
    list.delete(&4).unwrap();
    assert_eq!(order(&list), [3]);
    assert_consistent(&list);
    // The id can be used again
    list.push(task(1)).unwrap();
    assert_eq!(order(&list), [3, 1]);
}

#[test]
fn move_validates_its_neighbors() {
    let mut list = list(&[1, 2, 3, 4]);
    assert_eq!(
        list.move_to(&9, Some(&1), None).unwrap_err(),
        LexicalError::NotFound(9)
    );
    assert_eq!(
        list.move_to(&1, Some(&9), None).unwrap_err(),
        LexicalError::NotFound(9)
    );
    assert_eq!(
        list.move_to(&1, None, None).unwrap_err(),
        LexicalError::MissingNeighbors(1)
    );
    assert_eq!(
        list.move_to(&1, Some(&1), Some(&2)).unwrap_err(),
        LexicalError::SelfNeighbor(1)
    );
    assert_eq!(
        list.move_to(&1, Some(&2), Some(&4)).unwrap_err(),
        LexicalError::NotAdjacent {
            previous: Some(2),
            next: Some(4)
        }
    );
    assert_eq!(
        list.move_to(&1, Some(&2), None).unwrap_err(),
        LexicalError::NotAdjacent {
            previous: Some(2),
            next: None
        }
    );
    assert_eq!(
        list.move_to(&4, None, Some(&2)).unwrap_err(),
        LexicalError::NotAdjacent {
            previous: None,
            next: Some(2)
        }
    );
    // Nothing changed
    assert_eq!(order(&list), [1, 2, 3, 4]);
    assert_consistent(&list);
}

#[test]
fn move_changes_only_the_moved_item() {
    let mut list = list(&[1, 2, 3]);
    let before: Vec<String> = list
        .iter()
        .map(|item| item.value().position.clone())
        .collect();
    // Adjacent only once 3 leaves its place
    let moved = list.move_to(&3, Some(&1), Some(&2)).unwrap();
    assert_eq!(moved.value().position, "mmmmmn");
    assert_eq!(order(&list), [1, 3, 2]);
    assert_eq!(list.find(&1).unwrap().value().position, before[0]);
    assert_eq!(list.find(&2).unwrap().value().position, before[1]);
    // To both ends
    list.move_to(&2, None, Some(&1)).unwrap();
    list.move_to(&1, Some(&3), None).unwrap();
    assert_eq!(order(&list), [2, 3, 1]);
    // To its current place: nothing changes
    let position = list.find(&3).unwrap().value().position.clone();
    assert_eq!(
        list.move_to(&3, Some(&2), Some(&1))
            .unwrap()
            .value()
            .position,
        position
    );
    assert_consistent(&list);
}

#[test]
fn an_exhausted_gap_fails_without_changes_and_recovers_after_rebalance() {
    let mut list = list(&[1, 2, 3]);
    // Moves the last item right after the first one, again and again: the same gap narrows
    let error = loop {
        let ids = order(&list);
        if let Err(error) = list.move_to(&ids[2], Some(&ids[0]), Some(&ids[1])) {
            break error;
        }
        assert_consistent(&list);
    };
    assert!(
        matches!(
            error,
            LexicalError::Position(PositionError::ThresholdReached { .. })
        ),
        "{error}"
    );
    let snapshot: Vec<(u32, String)> = list
        .iter()
        .map(|item| (item.value().id, item.value().position.clone()))
        .collect();
    let ids = order(&list);
    assert!(list.move_to(&ids[2], Some(&ids[0]), Some(&ids[1])).is_err());
    let unchanged: Vec<(u32, String)> = list
        .iter()
        .map(|item| (item.value().id, item.value().position.clone()))
        .collect();
    assert_eq!(snapshot, unchanged);
    // Rebalancing spaces the positions again, keeping the order
    let affected: Vec<u32> = list
        .rebalance()
        .unwrap()
        .iter()
        .map(|item| item.value().id)
        .collect();
    assert!(!affected.is_empty());
    assert_eq!(
        order(&list),
        snapshot.iter().map(|(id, _)| *id).collect::<Vec<_>>()
    );
    assert_eq!(list.first().unwrap().value().position, "mmmmm!");
    list.move_to(&ids[2], Some(&ids[0]), Some(&ids[1])).unwrap();
    assert_consistent(&list);
}

#[test]
fn push_and_unshift_fail_at_the_edges_of_n_and_give_the_value_back() {
    // N of one digit, from the middle "b" of "a".."c"
    let system = Staquia::create('!', 'a', 'c', 1, 4, 0.0).unwrap();
    let mut list = LexicalList::new(system);
    list.push(task(1)).unwrap(); // "b!"
    list.push(task(2)).unwrap(); // "c!"
    let rejected = list.push(task(3)).unwrap_err();
    assert!(
        matches!(
            rejected.error,
            LexicalError::Position(PositionError::EdgeReached { .. })
        ),
        "{}",
        rejected.error
    );
    assert_eq!(rejected.value.id, 3);
    list.unshift(task(4)).unwrap(); // "a!"
    list.unshift(task(5)).unwrap(); // "!!", the zero
    assert!(list.unshift(task(6)).is_err());
    let positions: Vec<&str> = list
        .iter()
        .map(|item| item.value().position.as_str())
        .collect();
    assert_eq!(positions, ["!!", "a!", "b!", "c!"]);
    assert_consistent(&list);
}

#[test]
fn set_trusts_ordered_rows_and_rejects_broken_ones_without_changes() {
    let mut list = list(&[7]);
    let affected = list
        .set(vec![
            stored(1, "mmmmm!"),
            stored(2, "mmmmn!"),
            stored(3, "n"),
        ])
        .unwrap();
    assert!(affected.is_empty());
    assert_eq!(order(&list), [1, 2, 3]);
    for (rows, error) in [
        (
            vec![stored(4, "b"), stored(5, "a")],
            LexicalError::NotOrdered {
                index: 1,
                previous: 4,
                id: 5,
            },
        ),
        (
            vec![stored(4, "a"), stored(5, "a")],
            LexicalError::NotOrdered {
                index: 1,
                previous: 4,
                id: 5,
            },
        ),
        (
            vec![stored(4, "a"), stored(4, "b")],
            LexicalError::DuplicateId(4),
        ),
        (
            vec![stored(4, "")],
            LexicalError::InvalidPosition {
                id: 4,
                reason: None,
            },
        ),
        (
            vec![stored(4, "A")],
            LexicalError::InvalidPosition {
                id: 4,
                reason: Some(PositionError::InvalidCharacter {
                    character: 'A',
                    index: 0,
                }),
            },
        ),
    ] {
        let rejected = list.set(rows.clone()).unwrap_err();
        assert_eq!(rejected.error, error);
        // The rows come back untouched
        assert_eq!(rejected.value, rows);
        assert_eq!(order(&list), [1, 2, 3]);
    }
}

#[test]
fn set_untrusted_repairs_and_reports_the_affected_rows() {
    let mut list: LexicalList<Task> = LexicalList::new(system());
    let rows = vec![
        stored(1, "c"),
        stored(2, "a"),
        stored(3, "a"),
        stored(4, ""),
        stored(5, "b"),
        stored(6, "A"),
    ];
    let mut affected: Vec<u32> = list
        .set_untrusted(rows)
        .unwrap()
        .iter()
        .map(|item| item.value().id)
        .collect();
    affected.sort_unstable();
    // 2 keeps "a", 3 is repaired after it, 4 and 6 are added at the end
    assert_eq!(order(&list), [2, 3, 5, 1, 4, 6]);
    assert_eq!(affected, [3, 4, 6]);
    assert_eq!(list.find(&2).unwrap().value().position, "a!!!!!");
    assert_consistent(&list);
    // Sorted rows are not affected
    let rows: Vec<Task> = list.iter().map(|item| item.value().clone()).collect();
    assert!(list.set_untrusted(rows).unwrap().is_empty());
    // An empty array empties the list
    assert!(list.set(Vec::new()).unwrap().is_empty());
    assert!(list.is_empty());
}

#[test]
fn sort_by_reorders_and_reports_the_changed_positions() {
    let mut list = list(&[1, 2, 3, 4]);
    assert!(list.sort_by(|a, b| a.id.cmp(&b.id)).unwrap().is_empty());
    let affected = list.sort_by(|a, b| b.id.cmp(&a.id)).unwrap().len();
    assert_eq!(affected, 4);
    assert_eq!(order(&list), [4, 3, 2, 1]);
    assert_eq!(list.first().unwrap().value().position, "mmmmm!");
    assert_consistent(&list);
    let mut empty: LexicalList<Task> = LexicalList::new(system());
    assert!(empty.rebalance().unwrap().is_empty());
}

#[test]
fn sort_by_fails_without_changes_when_there_is_no_room() {
    // N of one digit from the middle "b" of "a".."c": only "b" and "c" can be stacked
    let system = Staquia::create('!', 'a', 'c', 1, 4, 0.0).unwrap();
    let mut list = LexicalList::new(system);
    list.push(task(1)).unwrap();
    list.unshift(task(2)).unwrap();
    list.unshift(task(3)).unwrap();
    let before: Vec<(u32, String)> = list
        .iter()
        .map(|item| (item.value().id, item.value().position.clone()))
        .collect();
    assert!(list.rebalance().is_err());
    let after: Vec<(u32, String)> = list
        .iter()
        .map(|item| (item.value().id, item.value().position.clone()))
        .collect();
    assert_eq!(before, after);
    assert_consistent(&list);
}

#[test]
fn string_ids_work_including_the_empty_string() {
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
    let named = |id: &str| Named {
        id: id.to_owned(),
        position: String::new(),
    };
    let mut list = LexicalList::new(system());
    list.push(named("")).unwrap();
    list.push(named("b")).unwrap();
    list.push(named("c")).unwrap();
    list.move_to(&"c".to_owned(), None, Some(&String::new()))
        .unwrap();
    let order: Vec<&str> = list.iter().map(|item| item.id().as_str()).collect();
    assert_eq!(order, ["c", "", "b"]);
    list.delete(&String::new()).unwrap();
    assert_eq!(list.len(), 2);
}
