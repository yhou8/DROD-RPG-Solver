// An iterator for DenseBitSet that returns the position of each enabled bit in the set

use rust_dense_bitset::BitSet as _;
use rust_dense_bitset::DenseBitSet as BitSet;

pub(super) struct BitSetIter(BitSet);

impl From<BitSet> for BitSetIter {
    fn from(bitset: BitSet) -> Self {
        Self(bitset)
    }
}

impl Iterator for BitSetIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.none() {
            None
        } else {
            let first_set = self.0.first_set() as usize;
            self.0.set_bit(first_set, false);
            Some(first_set)
        }
    }
}
