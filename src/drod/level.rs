use std::usize;

pub struct Level {
    // TODO add graph fields
    entrance: usize,
    exit: usize,
}

impl Level {
    pub fn new() -> Self {
        Self {
            entrance: usize::MAX,
            exit: usize::MAX,
        }
    }
}
