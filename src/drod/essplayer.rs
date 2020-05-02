use std::usize;

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

use super::stat::{PlayerStat, ProbeStat};
use super::Level;

#[derive(Clone, Copy, Debug)]
pub struct EssPlayer {
    pub(super) stat: PlayerStat,
    pub(super) neighbors: BitSet,
    pub(super) visited: BitSet,
    pub(super) last_visit: usize,
}

impl EssPlayer {
    // TODO replace with with_stat?
    fn new() -> Self {
        Self {
            stat: PlayerStat::default(),
            neighbors: BitSet::new(),
            visited: BitSet::new(),
            last_visit: usize::MAX,
        }
    }

    pub fn with_stat(stat: PlayerStat) -> Self {
        Self {
            stat,
            ..EssPlayer::new()
        }
    }

    pub(super) fn previous_visited(&self) -> BitSet {
        todo!()
    }

    pub(super) fn visit(&mut self, id: usize, level: &Level, probe: &ProbeStat) {
        todo!()
    }

    pub(super) fn enter(&mut self, level: &Level) {
        self.neighbors.set_bit(level.entrance, true);
    }
}
