use crate::*;
use dispatch::*;
impl<U: BitSet, const N: usize> BitSet for [U; N] {
    fn bitset(&mut self, index: usize, b: bool) {
        bitset_dispatch(self, index, b);
    }
}

impl<U: BitGet, const N: usize> BitGet for [U; N] {
    fn bitget(&self, index: usize) -> bool {
        bitget_dispatch(self, index)
    }
}
impl<U: BitTogle, const N: usize> BitTogle for [U; N] {
    fn bittogle(&mut self, index: usize) {
        bittogle_dispatch(self, index);
    }
}
