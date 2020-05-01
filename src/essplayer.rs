use std::usize;

use rust_dense_bitset::DenseBitSet as BitSet;

use crate::stat::PlayerStat;

#[derive(Debug)]
pub struct EssPlayer {
    stat: PlayerStat,
    neighbors: BitSet,
    visited: BitSet,
    last_visited: usize,
}

impl EssPlayer {
    pub fn new() -> Self {
        Self {
            stat: PlayerStat::default(),
            neighbors: BitSet::new(),
            visited: BitSet::new(),
            last_visited: usize::MAX,
        }
    }

    pub fn with_stat(stat: PlayerStat) -> Self {
        Self {
            stat,
            ..EssPlayer::new()
        }
    }
}
