use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as RoomSet;

use super::{Level, PlayerStat};

#[derive(Clone, Debug)]
pub(super) struct Route {
    pub(super) stat: PlayerStat,
    // level: Level,
    trace: Vec<u8>,
    neighbors: RoomSet,
    visited: RoomSet,
    pub(super) previous_visited: RoomSet,
}

impl Route {
    pub(super) fn new() -> Self {
        Self {
            stat: PlayerStat::default(),
            // level: Level::new(),
            trace: Vec::new(),
            neighbors: RoomSet::new(),
            visited: RoomSet::new(),
            previous_visited: RoomSet::new(),
        }
    }

    pub(super) fn with_stat(stat: PlayerStat) -> Self {
        Self {
            stat,
            ..Route::new()
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
