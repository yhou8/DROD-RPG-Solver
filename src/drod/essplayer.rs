use std::u8;

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

use super::stat::{PlayerStat, ProbeStat};
use super::Level;

#[derive(Clone, Copy, Debug)]
pub struct EssPlayer {
    pub(super) stat: PlayerStat,
    pub(super) neighbors: BitSet,
    pub(super) visited: BitSet,
    pub(super) last_visit: u8,
}

impl EssPlayer {
    // TODO replace with with_stat?
    fn new() -> Self {
        Self {
            stat: PlayerStat::default(),
            neighbors: BitSet::new(),
            visited: BitSet::new(),
            last_visit: u8::MAX,
        }
    }

    pub fn with_stat(stat: PlayerStat) -> Self {
        Self {
            stat,
            ..EssPlayer::new()
        }
    }

    pub(super) fn previous_visited(&self) -> BitSet {
        let idx = self.last_visit as usize;
        let mut bitset = self.visited;
        bitset.set_bit(idx, false);
        bitset
    }

    pub(super) fn visit(&mut self, room_id: u8, level: &Level, probe: &ProbeStat) {
        let idx = room_id as usize;
        self.stat += probe.diff;
        self.neighbors |= level.neighbors[idx];
        self.neighbors &= !level.excluded_neighbors[idx];
        self.neighbors &= !self.visited;
        self.last_visit = room_id;
        self.visited.set_bit(idx, true);
    }

    pub(super) fn enter(&mut self, level: &Level) {
        let idx = level.entrance as usize;
        self.neighbors.set_bit(idx, true);
    }
}
