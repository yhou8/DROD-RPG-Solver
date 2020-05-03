use std::u8;

use rust_dense_bitset::DenseBitSet as RoomSet;

use super::room::Room;

#[derive(Debug)]
pub struct Level {
    pub(super) entrance: u8,
    pub(super) exit: u8,

    // TODO add graph fields
    pub(super) next_id: u8,
    pub(super) neighbors: Vec<RoomSet>,
    pub(super) excluded_neighbors: Vec<RoomSet>,
    vertices: Vec<Room>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            entrance: u8::MAX,
            exit: u8::MAX,

            next_id: 0,
            neighbors: Vec::new(),
            excluded_neighbors: Vec::new(),
            vertices: Vec::new(),
        }
    }

    pub(super) fn vertex(&self, room_id: u8) -> &Room {
        let idx = room_id as usize;
        &self.vertices[idx]
    }
}
