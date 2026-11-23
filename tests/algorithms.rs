mod common;

use common::alphabetical_system;
use staquia::{structs::number::Number, traits::algorithms::Algorithms};
use std::collections::HashMap;

// AFTER TESTS

#[test]
pub fn after_position_aaa() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(2, 1), (1, 1), (0, 2)]);
    let position = Number {
        upper_bound: 2,
        lower_bound: 0,
        _map: HashMap::from([(2, 1), (1, 1), (0, 1)]),
    };

    // Act
    let aab = system.after(&position, false);

    // Assert
    assert_eq!(aab.upper_bound, 2);
    assert_eq!(aab.lower_bound, 0);
    assert_eq!(aab._map, map);
}

#[test]
pub fn after_position_aaz() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(2, 26), (1, 1), (0, 2)]);
    let position = Number {
        upper_bound: 2,
        lower_bound: 0,
        _map: HashMap::from([(2, 26), (1, 1), (0, 1)]),
    };

    // Act
    let aab = system.after(&position, false);

    // Assert
    assert_eq!(aab.upper_bound, 2);
    assert_eq!(aab.lower_bound, 0);
    assert_eq!(aab._map, map);
}

#[test]
pub fn after_position_a() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 2)]);
    let position = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 1)]),
    };

    // Act
    let b = system.after(&position, false);

    // Assert
    assert_eq!(b.upper_bound, 0);
    assert_eq!(b.lower_bound, 0);
    assert_eq!(b._map, map);
}

#[test]
pub fn after_position_az() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(1, 2), (0, 0)]);
    let position = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 1), (0, 26)]),
    };

    // Act
    let b0 = system.after(&position, false);

    // Assert
    assert_eq!(b0.upper_bound, 1);
    assert_eq!(b0.lower_bound, 0);
    assert_eq!(b0._map, map);
}

#[test]
pub fn after_position_azzz() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(3, 2), (2, 0), (1, 0), (0, 0)]);
    let position = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 1), (2, 26), (1, 26), (0, 26)]),
    };

    // Act
    let b000 = system.after(&position, false);

    // Assert
    assert_eq!(b000.upper_bound, 3);
    assert_eq!(b000.lower_bound, 0);
    assert_eq!(b000._map, map);
}

/// BEFORE TESTS

#[test]
pub fn after_position_aab() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(2, 1), (1, 1), (0, 1)]);
    let position = Number {
        upper_bound: 2,
        lower_bound: 0,
        _map: HashMap::from([(2, 1), (1, 1), (0, 2)]),
    };

    // Act
    let aaa = system.before(&position, false);

    // Assert
    assert_eq!(aaa.upper_bound, 2);
    assert_eq!(aaa.lower_bound, 0);
    assert_eq!(aaa._map, map);
}

#[test]
pub fn after_position_aa0() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(2, 1), (1, 0), (0, 26)]);
    let position = Number {
        upper_bound: 2,
        lower_bound: 0,
        _map: HashMap::from([(2, 1), (1, 1), (0, 0)]),
    };

    // Act
    let a0z = system.before(&position, false);

    // Assert
    assert_eq!(a0z.upper_bound, 2);
    assert_eq!(a0z.lower_bound, 0);
    assert_eq!(a0z._map, map);
}

#[test]
pub fn after_position_z0() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(1, 25), (0, 26)]);
    let position = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 26), (0, 0)]),
    };

    // Act
    let z0 = system.before(&position, false);

    // Assert
    assert_eq!(z0.upper_bound, 1);
    assert_eq!(z0.lower_bound, 0);
    assert_eq!(z0._map, map);
}

#[test]
pub fn after_position_z() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 25)]);
    let position = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 26)]),
    };

    // Act
    let z = system.before(&position, false);

    // Assert
    assert_eq!(z.upper_bound, 0);
    assert_eq!(z.lower_bound, 0);
    assert_eq!(z._map, map);
}

// BETWEEN TESTS

#[test]
pub fn between_position_yz_position_za() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(1, 26), (0, 0)]);
    let before = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 25), (0, 26)]),
    };
    let after = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 26), (0, 1)]),
    };

    // Act
    let z0 = system.between(&before, &after, false);

    // Assert
    assert_eq!(z0.upper_bound, 1);
    assert_eq!(z0.lower_bound, 0);
    assert_eq!(z0._map, map);
}

#[test]
pub fn between_position_akz_position_abyn() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(3, 0), (2, 15), (1, 18), (0, 20)]);
    let before = Number {
        upper_bound: 2,
        lower_bound: 0,
        _map: HashMap::from([(2, 1), (1, 11), (0, 26)]),
    };
    let after = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 1), (2, 2), (1, 25), (0, 14)]),
    };

    // Act
    let _0ort = system.between(&before, &after, false);

    // Assert
    assert_eq!(_0ort.upper_bound, 3);
    assert_eq!(_0ort.lower_bound, 0);
    assert_eq!(_0ort._map, map);
}

#[test]
pub fn between_position_00abz_position_abzaa() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(4, 0), (3, 15), (2, 0), (1, 2), (0, 0)]);
    let before = Number {
        upper_bound: 4,
        lower_bound: 0,
        _map: HashMap::from([(4, 0), (3, 0), (2, 1), (1, 2), (0, 26)]),
    };
    let after = Number {
        upper_bound: 4,
        lower_bound: 0,
        _map: HashMap::from([(4, 1), (3, 2), (2, 26), (1, 1), (0, 1)]),
    };

    // Act
    let _0o0b0 = system.between(&before, &after, false);

    // Assert
    assert_eq!(_0o0b0.upper_bound, 4);
    assert_eq!(_0o0b0.lower_bound, 0);
    assert_eq!(_0o0b0._map, map);
}

#[test]
pub fn between_position_abyz_position_abz_round() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(4, 0), (3, 15), (2, 0), (1, 1), (0, 13)]);
    let before = Number {
        upper_bound: 4,
        lower_bound: 0,
        _map: HashMap::from([(4, 1), (3, 2), (2, 25), (1, 26), (0, 26)]),
    };
    let after = Number {
        upper_bound: 2,
        lower_bound: 0,
        _map: HashMap::from([(2, 1), (1, 2), (0, 26)]),
    };

    // Act
    let _0o0am = system.between(&before, &after, true);

    // Assert
    assert_eq!(_0o0am.upper_bound, 4);
    assert_eq!(_0o0am.lower_bound, 0);
    assert_eq!(_0o0am._map, map);
}

#[test]
pub fn between_position_r_position_ri() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(1, 9), (0, 13), (-1, 13)]);
    let before = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 18)]),
    };
    let after = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 18), (0, 9)]),
    };

    // Act
    let imm = system.between(&before, &after, false);

    // Assert
    assert_eq!(imm.upper_bound, 1);
    assert_eq!(imm.lower_bound, -1);
    assert_eq!(imm._map, map);
}

#[test]
pub fn between_position_a_position_z() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 13), (-1, 13)]);
    let before = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 1)]),
    };
    let after = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 26)]),
    };

    // Act
    let mm = system.between(&before, &after, false);

    // Assert
    assert_eq!(mm.upper_bound, 0);
    assert_eq!(mm.lower_bound, -1);
    assert_eq!(mm._map, map);
}

#[test]
pub fn between_position_a_position_z_round() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 14)]);
    let before = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 1)]),
    };
    let after = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 26)]),
    };

    // Act
    let n = system.between(&before, &after, true);

    // Assert
    assert_eq!(n.upper_bound, 0);
    assert_eq!(n.lower_bound, 0);
    assert_eq!(n._map, map);
}

#[test]
pub fn between_position_a_position_c_round() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 2)]);
    let before = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 1)]),
    };
    let after = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 3)]),
    };

    // Act
    let n = system.between(&before, &after, true);

    // Assert
    assert_eq!(n.upper_bound, 0);
    assert_eq!(n.lower_bound, 0);
    assert_eq!(n._map, map);
}

// DIVERGE TESTS

#[test]
pub fn diverge_position_0xca_position_0xaa_round() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(2, 0), (1, 24), (0, 2)]);
    let before = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 3), (0, 1)]),
    };
    let after = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 0), (2, 24), (1, 1), (0, 1)]),
    };

    // Act
    let n = system.diverge(&before, &after);

    // Assert
    assert_eq!(n.upper_bound, 2);
    assert_eq!(n.lower_bound, 0);
    assert_eq!(n._map, map);
}

#[test]
pub fn diverge_position_a_position_zz() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 14)]);
    let before = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 1)]),
    };
    let after = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 26), (0, 26)]),
    };

    // Act
    let n = system.diverge(&before, &after);

    // Assert
    assert_eq!(n.upper_bound, 0);
    assert_eq!(n.lower_bound, 0);
    assert_eq!(n._map, map);
}

#[test]
pub fn diverge_position_00a_position_00z() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(2, 0), (1, 0), (0, 14)]);
    let before = Number {
        upper_bound: 2,
        lower_bound: 0,
        _map: HashMap::from([(2, 0), (1, 0), (0, 1)]),
    };
    let after = Number {
        upper_bound: 2,
        lower_bound: 0,
        _map: HashMap::from([(2, 0), (1, 0), (0, 26)]),
    };

    // Act
    let _00n = system.diverge(&before, &after);

    // Assert
    assert_eq!(_00n.upper_bound, 2);
    assert_eq!(_00n.lower_bound, 0);
    assert_eq!(_00n._map, map);
}

#[test]
pub fn diverge_position_az_position_b0() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(2, 1), (1, 26), (0, 13)]);
    let before = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 1), (0, 26)]),
    };
    let after = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 2), (0, 0)]),
    };

    // Act
    let azm = system.diverge(&before, &after);

    // Assert
    assert_eq!(azm.upper_bound, 2);
    assert_eq!(azm.lower_bound, 0);
    assert_eq!(azm._map, map);
}

#[test]
pub fn diverge_position_ng_position_n() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(1, 14), (0, 4)]);
    let before = Number {
        upper_bound: 1,
        lower_bound: 0,
        _map: HashMap::from([(1, 14), (0, 7)]),
    };
    let after = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 14)]),
    };

    // Act
    let nd = system.diverge(&before, &after);

    // Assert
    assert_eq!(nd.upper_bound, 1);
    assert_eq!(nd.lower_bound, 0);
    assert_eq!(nd._map, map);
}

#[test]
pub fn diverge_position_abba_position_abccc() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(3, 1), (2, 2), (1, 2), (0, 15)]);
    let before = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 1), (2, 2), (1, 2), (0, 1)]),
    };
    let after = Number {
        upper_bound: 4,
        lower_bound: 0,
        _map: HashMap::from([(4, 1), (3, 2), (2, 3), (1, 3), (0, 3)]),
    };

    // Act
    let abbo = system.diverge(&before, &after);

    // Assert
    assert_eq!(abbo.upper_bound, 3);
    assert_eq!(abbo.lower_bound, 0);
    assert_eq!(abbo._map, map);
}

#[test]
pub fn diverge_position_nnnn_position_nnno() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(4, 14), (3, 14), (2, 14), (1, 14), (0, 13)]);
    let before = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 14)]),
    };
    let after = Number {
        upper_bound: 3,
        lower_bound: 0,
        _map: HashMap::from([(3, 14), (2, 14), (1, 14), (0, 15)]),
    };

    // Act
    let nnnnm = system.diverge(&before, &after);

    // Assert
    assert_eq!(nnnnm.upper_bound, 4);
    assert_eq!(nnnnm.lower_bound, 0);
    assert_eq!(nnnnm._map, map);
}

#[test]
pub fn diverge_position_0_position_z() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 13)]);
    let before = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 0)]),
    };
    let after = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 26)]),
    };

    // Act
    let m = system.diverge(&before, &after);

    // Assert
    assert_eq!(m.upper_bound, 0);
    assert_eq!(m.lower_bound, 0);
    assert_eq!(m._map, map);
}

#[test]
pub fn diverge_position_a_position_z() {
    // Arrange
    let system = alphabetical_system();
    let map = HashMap::from([(0, 14)]);
    let before = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 1)]),
    };
    let after = Number {
        upper_bound: 0,
        lower_bound: 0,
        _map: HashMap::from([(0, 26)]),
    };

    // Act
    let n = system.diverge(&before, &after);

    // Assert
    assert_eq!(n.upper_bound, 0);
    assert_eq!(n.lower_bound, 0);
    assert_eq!(n._map, map);
}
