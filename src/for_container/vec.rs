use super::dispatch::*;
use crate::*;

extern crate alloc;

use alloc::vec::Vec;

impl<U: BitSet> BitSet for Vec<U> {
    fn bitset(&mut self, index: usize, b: bool) {
        let bits = core::mem::size_of::<U>() * 8;
        self[index / bits].bitset(index % bits, b);
    }
}
impl<U: BitGet> BitGet for Vec<U> {
    fn bitget(&self, index: usize) -> bool {
        bitget_dispatch(self, index)
    }
}
impl<U: BitTogle> BitTogle for Vec<U> {
    fn bittogle(&mut self, index: usize) {
        bittogle_dispatch(self, index);
    }
}
