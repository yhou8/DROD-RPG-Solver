use std::usize;

use rust_dense_bitset::DenseBitSet as BitSet;

use super::room::Room;

pub struct Level {
    pub(super) entrance: usize,
    pub(super) exit: usize,

    // TODO add graph fields
    pub(super) neighbours: Vec<BitSet>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            entrance: usize::MAX,
            exit: usize::MAX,

            neighbours: Vec::new(),
        }
    }

    // fn current_vertex(&self) -> &Room {
    //     todo!()
    // }

    pub(super) fn vertex(&self, id: usize) -> &Room {
        todo!()
    }
}
