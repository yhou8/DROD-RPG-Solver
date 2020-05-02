use rust_dense_bitset::DenseBitSet as BitSet;

use super::{Level, PlayerStat};

#[derive(Clone, Debug)]
pub(super) struct Player {
    pub(super) stat: PlayerStat,
    // level: Level,
    trace: Vec<i32>,
    neighbors: BitSet,
    visited: BitSet,
    pub(super) previous_visited: BitSet,
}

impl Player {
    pub(super) fn new() -> Self {
        Self {
            stat: PlayerStat::default(),
            // level: Level::new(),
            trace: Vec::new(),
            neighbors: BitSet::new(),
            visited: BitSet::new(),
            previous_visited: BitSet::new(),
        }
    }
}
