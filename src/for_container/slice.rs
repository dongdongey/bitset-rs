use super::dispatch::*;
use crate::*;

impl<U: BitSet> BitSet for &mut [U] {
    fn bitset(&mut self, index: usize, b: bool) {
        bitset_dispatch(self, index, b);
    }
}

impl<U: BitGet> BitGet for &mut [U] {
    fn bitget(&self, index: usize) -> bool {
        bitget_dispatch(self, index)
    }
}
impl<U: BitTogle> BitTogle for &mut [U] {
    fn bittogle(&mut self, index: usize) {
        bittogle_dispatch(self, index);
    }
}

impl<U: BitGet> BitGet for &[U] {
    fn bitget(&self, index: usize) -> bool {
        bitget_dispatch(self, index)
    }
}
