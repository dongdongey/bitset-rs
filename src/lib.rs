pub trait BitSet {
    fn bitset(&mut self, index: usize, b: bool);
    fn bitget(&self, index: usize) -> bool;
}
use std::mem::size_of;

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

#[cfg(test)]
mod tests {
    use crate::BitSet;

    #[test]
    fn it_works_at_u8() {
        let mut bit = 0_u8;
        BitSet::bitset(&mut bit, 3, true);
        assert_eq!(bit, 0b00010000);
        assert_eq!(bit.bitget(3), true);
        BitSet::bitset(&mut bit, 3, false);
        assert_eq!(bit.bitget(3), false);
    }
    #[test]
    fn it_works_at_u16() {
        let mut bit = 0_u16;
        BitSet::bitset(&mut bit, 3, true);
        assert_eq!(bit, 0b0001000000000000);
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
}
