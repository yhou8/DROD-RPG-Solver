use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

use super::{Level, PlayerStat};

#[derive(Clone, Debug)]
pub(super) struct Player {
    pub(super) stat: PlayerStat,
    // level: Level,
    trace: Vec<usize>,
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
        self.neighbors.set_bit(level.entrance, true)
    }

    pub(super) fn visit(&mut self, id: usize, level: &Level) {
        assert!(self.neighbors.get_bit(id));
        let probe = level.vertex(id).to_probe_stat(&self.stat.into());
        assert!(self.stat.ge(&probe.req));
        self.stat += probe.diff;
        self.trace.push(id);
        self.neighbors |= level.neighbors[id];
        self.neighbors &= !level.excluded_neighbors[id];
        self.neighbors &= !self.visited;
        self.previous_visited = self.visited;
        self.visited.set_bit(id, true);
    }
}
