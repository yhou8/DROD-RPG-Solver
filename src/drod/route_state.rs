use std::u8;

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as RoomSet;

use super::stat::{PlayerStat, ProbeStat};
use super::Level;

#[derive(Clone, Copy, Debug)]
pub struct RouteState {
    pub(super) stat: PlayerStat,
    pub(super) neighbors: RoomSet,
    pub(super) visited: RoomSet,
    pub(super) last_visit: u8,
}

impl RouteState {
    // TODO replace with with_stat?
    fn new() -> Self {
        Self {
            stat: PlayerStat::default(),
            neighbors: RoomSet::new(),
            visited: RoomSet::new(),
            last_visit: u8::MAX,
        }
    }

    pub fn with_stat(stat: PlayerStat) -> Self {
        Self {
            stat,
            ..RouteState::new()
        }
    }

    pub(super) fn previous_visited(&self) -> RoomSet {
        let idx = self.last_visit as usize;
        let mut rooms = self.visited;
        rooms.set_bit(idx, false);
        rooms
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
