mod common;

use common::alphabetical_system;
use staquia::{structs::number::Number, traits::numbers::Numbers};
use std::collections::HashMap;

#[test]
fn string_from_number_nuxt() {
    // Arrange
    let system = alphabetical_system();
    let number = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 21), (1, 24), (0, 20)]),
    };

    // Act
    let string = system.string_from(&number);

    // Assert
    assert_eq!(string, "nuxt");
}

#[test]
fn eq_number_nnnn_number_nnnn() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };

    // Act
    let result = left.eq(&right);

    // Assert
    assert_eq!(result, true)
}

#[test]
fn eq_number_nnnn_number_nnnz() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 26)]),
    };

    // Act
    let result = left.eq(&right);

    // Assert
    assert_eq!(result, false)
}

#[test]
fn ne_number_nnnn_number_nnnn() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };

    // Act
    let result = left.ne(&right);

    // Assert
    assert_eq!(result, false)
}

#[test]
fn ne_number_nnnn_number_nnnz() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 26)]),
    };

    // Act
    let result = left.ne(&right);

    // Assert
    assert_eq!(result, true)
}

#[test]
pub fn gt_number_yz_number_z0() {
    // Arrange
    let left = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 25), (0, 26)]),
    };
    let right = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 26), (0, 1)]),
    };

    // Act
    let result = left.gt(&right);

    // Assert
    assert_eq!(result, false);
}

#[test]
pub fn gt_number_0xca_number_0xaa() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 3), (0, 1)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 1), (0, 1)]),
    };

    // Act
    let result = left.gt(&right);

    // Assert
    assert_eq!(result, true);
}

#[test]
pub fn ge_number_0xca_number_0xaa() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 3), (0, 1)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 1), (0, 1)]),
    };

    // Act
    let result = left.ge(&right);

    // Assert
    assert_eq!(result, true);
}

#[test]
pub fn ge_number_nnnn_number_nnnn() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };

    // Act
    let result = left.ge(&right);

    // Assert
    assert_eq!(result, true);
}

#[test]
pub fn ge_number_yz_number_z0() {
    // Arrange
    let left = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 25), (0, 26)]),
    };
    let right = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 26), (0, 1)]),
    };

    // Act
    let result = left.ge(&right);

    // Assert
    assert_eq!(result, false);
}

#[test]
pub fn lt_number_yz_number_z0() {
    // Arrange
    let left = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 25), (0, 26)]),
    };
    let right = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 26), (0, 1)]),
    };

    // Act
    let result = left.lt(&right);

    // Assert
    assert_eq!(result, true);
}

#[test]
pub fn lt_number_0xca_number_0xaa() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 3), (0, 1)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 1), (0, 1)]),
    };

    // Act
    let result = left.lt(&right);

    // Assert
    assert_eq!(result, false);
}

#[test]
pub fn le_number_0xca_number_0xaa() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 3), (0, 1)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 1), (0, 1)]),
    };

    // Act
    let result = left.le(&right);

    // Assert
    assert_eq!(result, false);
}

#[test]
pub fn le_number_nnnn_number_nnnn() {
    // Arrange
    let left = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };
    let right = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };

    // Act
    let result = left.le(&right);

    // Assert
    assert_eq!(result, true);
}

#[test]
pub fn le_number_yz_number_z0() {
    // Arrange
    let left = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 25), (0, 26)]),
    };
    let right = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 26), (0, 1)]),
    };

    // Act
    let result = left.le(&right);

    // Assert
    assert_eq!(result, true);
}
