#![no_std]
pub trait BitSet: BitGet {
    fn bitset(&mut self, index: usize, b: bool);
}
pub trait BitGet {
    fn bitget(&self, index: usize) -> bool;
}

use core::mem::size_of;

macro_rules! define_bitset_impl {
    ($t:ty) => {
        impl BitSet for $t {
            fn bitset(&mut self, index: usize, b: bool) {
                let init: Self = 1 << (size_of::<Self>() * 8 - 1);
                let set: Self = init >> index;
                *self = *self | set;
                if !b {
                    *self = *self ^ set;
                }
            }
        }
        impl BitGet for $t{
            fn bitget(&self, index: usize) -> bool {
                let init: Self = 1 as $t << (size_of::<Self>() * 8 - 1);
                let set: Self = init >> index;
                let get = self & set;
                if get == 0 {
                    return false;
                } else {
                    return true;
                }
            }
        }
    };
}

define_bitset_impl!(u8);
define_bitset_impl!(u16);
define_bitset_impl!(u32);
define_bitset_impl!(u64);
define_bitset_impl!(u128);

impl<U: BitSet> BitSet for &mut [U] {
    fn bitset(&mut self, index: usize, b: bool) {
        let bits = size_of::<U>() * 8;
        self[index / bits].bitset(index % bits, b);
    }
}

impl<U: BitGet> BitGet for &mut [U] {
    fn bitget(&self, index: usize) -> bool {
        let bits = size_of::<U>() * 8;
        self[index / bits].bitget(index % bits)
    }
}

impl<U: BitGet> BitGet for &[U] {
    fn bitget(&self, index: usize) -> bool {
        let bits = size_of::<U>() * 8;
        self[index / bits].bitget(index % bits)
    }
}

extern crate alloc;
impl<U: BitSet> BitSet for alloc::vec::Vec<U> {
    fn bitset(&mut self, index: usize, b: bool) {
        let bits = core::mem::size_of::<U>() * 8;
        self[index / bits].bitset(index % bits, b);
    }
}
impl<U: BitGet> BitGet for alloc::vec::Vec<U> {
    fn bitget(&self, index: usize) -> bool {
        let bits = core::mem::size_of::<U>() * 8;
        self[index / bits].bitget(index % bits)
    }
}
impl<T: BitSet, const N: usize> BitSet for [T; N] {
    fn bitset(&mut self, index: usize, b: bool) {
        let bits = core::mem::size_of::<T>() * 8;
        self[index / bits].bitset(index % bits, b);
    }
}

impl<T: BitGet, const N: usize> BitGet for [T; N] {
    fn bitget(&self, index: usize) -> bool {
        let bits = core::mem::size_of::<T>() * 8;
        self[index / bits].bitget(index % bits)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BitGet, BitSet};

    #[test]
    fn it_works_at_u8() {
        let mut bit = 0_u8;
        BitSet::bitset(&mut bit, 3, true);
        assert_eq!(bit, 0b0001_0000);
        assert_eq!(bit.bitget(3), true);
        BitSet::bitset(&mut bit, 3, false);
        assert_eq!(bit.bitget(3), false);
    }
    #[test]
    fn it_works_at_u16() {
        let mut bit = 0_u16;
        BitSet::bitset(&mut bit, 3, true);
        assert_eq!(bit, 0b0001_0000_0000_0000);
        assert_eq!(bit.bitget(3), true);
        BitSet::bitset(&mut bit, 3, false);
        assert_eq!(bit.bitget(3), false);
    }
    #[test]
    fn it_works_at_u32() {
        let mut bit = 0_u32;
        BitSet::bitset(&mut bit, 3, true);
        assert_eq!(bit.bitget(3), true);
        BitSet::bitset(&mut bit, 3, false);
        assert_eq!(bit.bitget(3), false);
    }
    #[test]
    fn it_works_at_u64() {
        let mut bit = 0_u64;
        BitSet::bitset(&mut bit, 3, true);
        assert_eq!(bit.bitget(3), true);
        BitSet::bitset(&mut bit, 3, false);
        assert_eq!(bit.bitget(3), false);
    }
    #[test]
    fn it_works_at_u128() {
        let mut bit = 0_u128;
        BitSet::bitset(&mut bit, 3, true);
        assert_eq!(bit.bitget(3), true);
        BitSet::bitset(&mut bit, 3, false);
        assert_eq!(bit.bitget(3), false);
    }

    #[test]
    fn it_works_at_u8_slice() {
        let mut bit = [0_u8; 3];
        bit.bitset(20, true);
        assert_eq!(bit.bitget(20), true);
        bit.bitset(20, false);
        assert_eq!(bit.bitget(20), false);
    }
    #[test]
    fn it_works_at_u8_vec() {
        use alloc::vec;
        use alloc::vec::Vec;
        let mut bit: Vec<u8> = vec![0_u8; 3];
        bit.bitset(20, true);
        assert_eq!(bit.bitget(20), true);
        bit.bitset(20, false);
        assert_eq!(bit.bitget(20), false);
    }
}
