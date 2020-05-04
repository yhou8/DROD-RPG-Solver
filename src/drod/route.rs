use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as RoomSet;

use super::{Level, Player};

#[derive(Clone, Debug)]
pub(super) struct Route {
    pub(super) player: Player,
    level: Rc<Level>,
    trace: Vec<u8>,
    neighbors: RoomSet,
    visited: RoomSet,
    pub(super) previous_visited: RoomSet,
}

impl Route {
    pub(super) fn new(player: Player, level: Rc<Level>) -> Self {
        let neighbors = RoomSet::from_integer(1 << level.entrance);
        Self {
            player,
            level,
            trace: Vec::new(),
            neighbors,
            visited: RoomSet::new(),
            previous_visited: RoomSet::new(),
        }
    }

    pub(super) fn visit(&mut self, room_id: u8, level: &Level) {
        let idx = room_id as usize;
        assert!(self.neighbors.get_bit(idx));
        let probe = level.vertex(room_id).probe(&self.player.into());
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

impl Display for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut trace_str = String::new();
        let mut first = true;
        for id in &self.trace {
            if first {
                first = false;
            } else {
                trace_str += ", ";
            }
            trace_str += &self.level.vertex(*id).name;
        }
        write!(f, "{}\n\nTrace: {}", self.player, trace_str)
    }
}
