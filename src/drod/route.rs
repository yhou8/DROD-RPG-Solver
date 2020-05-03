use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as RoomSet;

use super::{Level, Player};

#[derive(Clone, Debug)]
pub(super) struct Route {
    pub(super) player: Player,
    trace: Vec<u8>,
    neighbors: RoomSet,
    visited: RoomSet,
    pub(super) previous_visited: RoomSet,
}

impl Route {
    pub(super) fn new() -> Self {
        Self {
            player: Player::default(),
            trace: Vec::new(),
            neighbors: RoomSet::new(),
            visited: RoomSet::new(),
            previous_visited: RoomSet::new(),
        }
    }

    pub(super) fn with_player(player: Player) -> Self {
        Self {
            player,
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
        let probe = level.vertex(room_id).to_probe_stat(&self.player.into());
        assert!(self.player.dominate(&probe.req));
        self.player += probe.diff;
        self.trace.push(room_id);
        self.neighbors |= level.neighbors[idx];
        self.neighbors &= !level.excluded_neighbors[idx];
        self.neighbors &= !self.visited;
        self.previous_visited = self.visited;
        self.visited.set_bit(idx, true);
    }
}
