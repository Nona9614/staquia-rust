mod common;

use common::alphabetical_system;
use staquia::structs::number::Number;
use staquia::traits::positions::Positions;
use std::collections::HashMap;

#[test]
fn forge_small_string() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 24), (1, 21), (2, 14), (3, 0)]);

    // Act
    let position = system.forge("nux");

    // Assert
    assert_eq!(position.n.upper_bound, 3);
    assert_eq!(position.n.lower_bound, 0);
    assert_eq!(position.n._map, map);

    assert_eq!(position.z.upper_bound, 0);
    assert_eq!(position.z.lower_bound, 0);
}

#[test]
fn forge_big_string() {
    // Arrange
    let system = alphabetical_system();
    let n_map = HashMap::from([(0, 5), (1, 6), (2, 15), (3, 11)]);
    let z_map = HashMap::from([(0, 3), (1, 17)]);

    // Act
    let position = system.forge("kofeqc");

    // Assert
    assert_eq!(position.n.upper_bound, 3);
    assert_eq!(position.n.lower_bound, 0);
    assert_eq!(position.n._map, n_map);

    assert_eq!(position.z.upper_bound, 1);
    assert_eq!(position.z.lower_bound, 0);
    assert_eq!(position.z._map, z_map);
}

#[test]
fn forge_n_length_string() {
    // Arrange
    let system = alphabetical_system();
    let n_map = HashMap::from([(0, 4), (1, 12), (2, 10), (3, 18)]);

    // Act
    let position = system.forge("rjld");

    // Assert
    assert_eq!(position.n.upper_bound, 3);
    assert_eq!(position.n.lower_bound, 0);
    assert_eq!(position.n._map, n_map);

    assert_eq!(position.z.upper_bound, 0);
    assert_eq!(position.z.lower_bound, 0);
}

#[test]
fn position_from_small_number() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 24), (1, 21), (2, 0), (3, 0)]);
    let number = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(0, 24), (1, 21)]),
    };

    // Act
    let position = system.position_from(&number);

    // Assert
    assert_eq!(position.n.upper_bound, 3);
    assert_eq!(position.n.lower_bound, 0);
    assert_eq!(position.n._map, map);

    assert_eq!(position.z.upper_bound, 0);
    assert_eq!(position.z.lower_bound, 0);
}

#[test]
fn position_from_big_from() {
    // Arrange
    let system = alphabetical_system();
    let n_map = HashMap::from([(0, 5), (1, 6), (2, 15), (3, 11)]);
    let z_map = HashMap::from([(0, 3), (1, 17)]);
    let number = Number {
        upper_bound: 5,
        lower_bound: 0,
        _map: HashMap::from([(0, 3), (1, 17), (2, 5), (3, 6), (4, 15), (5, 11)]),
    };

    // Act
    let position = system.position_from(&number);

    // Assert
    assert_eq!(position.n.upper_bound, 3);
    assert_eq!(position.n.lower_bound, 0);
    assert_eq!(position.n._map, n_map);

    assert_eq!(position.z.upper_bound, 1);
    assert_eq!(position.z.lower_bound, 0);
    assert_eq!(position.z._map, z_map);
}

#[test]
fn position_from_n_length_number() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 4), (1, 12), (2, 10), (3, 18)]);
    let number = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(0, 4), (1, 12), (2, 10), (3, 18)]),
    };

    // Act
    let position = system.position_from(&number);

    // Assert
    assert_eq!(position.n.upper_bound, 3);
    assert_eq!(position.n.lower_bound, 0);
    assert_eq!(position.n._map, map);

    assert_eq!(position.z.upper_bound, 0);
    assert_eq!(position.z.lower_bound, 0);
}

#[test]
fn start() {
    // Arrang
    let system = alphabetical_system();
    let map = HashMap::from([(0, 13), (2, 13), (1, 13), (3, 13)]);

    // Act
    let position = system.start();

    // Assert
    assert_eq!(position.n.upper_bound, 3);
    assert_eq!(position.n.lower_bound, 0);
    assert_eq!(position.n._map, map);

    assert_eq!(position.z.upper_bound, 0);
    assert_eq!(position.z.lower_bound, 0);
}
