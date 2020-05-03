use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

use super::{Level, PlayerStat};

#[derive(Clone, Debug)]
pub(super) struct Player {
    pub(super) stat: PlayerStat,
    // level: Level,
    trace: Vec<u8>,
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

    pub(super) fn with_stat(stat: PlayerStat) -> Self {
        Self {
            stat,
            ..Player::new()
        }
    }

    pub(super) fn enter(&mut self, level: &Level) {
        let idx = level.entrance as usize;
        self.neighbors.set_bit(idx, true)
    }

    pub(super) fn visit(&mut self, room_id: u8, level: &Level) {
        let idx = room_id as usize;
        assert!(self.neighbors.get_bit(idx));
        let probe = level.vertex(room_id).to_probe_stat(&self.stat.into());
        assert!(self.stat.ge(&probe.req));
        self.stat += probe.diff;
        self.trace.push(room_id);
        self.neighbors |= level.neighbors[idx];
        self.neighbors &= !level.excluded_neighbors[idx];
        self.neighbors &= !self.visited;
        self.previous_visited = self.visited;
        self.visited.set_bit(idx, true);
    }
}
