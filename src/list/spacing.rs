//! Giving positions to many entries at once.

use crate::{Position, PositionError, Staquia};

/// Gives a position to each run of entries without one: between two positions the gap is bisected, at the
/// start and at the end the positions are stacked. Returns `false` if there is no room.
pub(super) fn fill(system: &Staquia, positions: &mut [Option<Position>]) -> bool {
    fn bisect(
        system: &Staquia,
        positions: &mut [Option<Position>],
        from: usize,
        to: usize,
        left: &Position,
        right: &Position,
    ) -> bool {
        if from >= to {
            return true;
        }
        let middle = (from + to) >> 1;
        let Ok(core) = system.core(left, right) else {
            return false;
        };
        positions[middle] = Some(core);
        bisect(system, positions, from, middle, left, &core)
            && bisect(system, positions, middle + 1, to, &core, right)
    }

    let mut i = 0;
    while i < positions.len() {
        if positions[i].is_some() {
            i += 1;
            continue;
        }
        // The run of entries without position is [i, j)
        let mut j = i;
        while j < positions.len() && positions[j].is_none() {
            j += 1;
        }
        let left = i.checked_sub(1).and_then(|k| positions[k]);
        let right = positions.get(j).copied().flatten();
        match (left, right) {
            (Some(left), Some(right)) => {
                if !bisect(system, positions, i, j, &left, &right) {
                    return false;
                }
            }
            (None, Some(right)) => {
                let mut reference = right;
                for k in (i..j).rev() {
                    let Ok(previous) = system.previous(&reference) else {
                        return false;
                    };
                    positions[k] = Some(previous);
                    reference = previous;
                }
            }
            (left, None) => {
                let mut reference = match left {
                    Some(left) => system.next(&left),
                    None => system.start(),
                };
                let run = &mut positions[i..j];
                let count = run.len();
                for (k, slot) in run.iter_mut().enumerate() {
                    let Ok(position) = reference else {
                        return false;
                    };
                    *slot = Some(position);
                    if k + 1 < count {
                        reference = system.next(&position);
                    }
                }
            }
        }
        i = j;
    }
    true
}

/// Gives every entry a new position, stacked from the start position.
pub(super) fn respace(
    system: &Staquia,
    positions: &mut [Option<Position>],
) -> Result<(), PositionError> {
    let mut reference = system.start();
    let count = positions.len();
    for (i, slot) in positions.iter_mut().enumerate() {
        let position = reference?;
        *slot = Some(position);
        if i + 1 < count {
            reference = system.next(&position);
        }
    }
    Ok(())
}
