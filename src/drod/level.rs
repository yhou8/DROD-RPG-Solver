use std::usize;

use rust_dense_bitset::DenseBitSet as BitSet;

use super::room::Room;

#[derive(Debug)]
pub struct Level {
    pub(super) entrance: usize,
    pub(super) exit: usize,

    // TODO add graph fields
    pub(super) next_id: usize,
    pub(super) neighbors: Vec<BitSet>,
    pub(super) excluded_neighbors: Vec<BitSet>,
    vertices: Vec<Room>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            entrance: usize::MAX,
            exit: usize::MAX,

            next_id: 0,
            neighbors: Vec::new(),
            excluded_neighbors: Vec::new(),
            vertices: Vec::new(),
        }
    }

    pub(super) fn vertex(&self, id: usize) -> &Room {
        &self.vertices[id]
    }
}
