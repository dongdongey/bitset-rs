use crate::*;
use core::mem::size_of;

macro_rules! define_bitset_impl {
    ($t:ty) => {
        impl BitSet for $t {
            fn bitset(&mut self, index: usize, b: bool) {
                const INIT: $t = 1 << ((size_of::<$t>() << 3) - 1);
                let set: $t = INIT >> index;
                if b {
                    *self |= set;
                } else {
                    *self &= !set;
                }
            }
        }
        impl BitGet for $t {
            fn bitget(&self, index: usize) -> bool {
                const INIT: $t = 1 << ((size_of::<$t>() << 3) - 1);
                let set: $t = INIT >> index;
                self & set != 0
            }
        }
        impl BitTogle for $t {
            fn bittogle(&mut self, index: usize) {
                const INIT: $t = 1 << ((size_of::<$t>() << 3) - 1);
                let set: $t = INIT >> index;
                *self ^= set;
            }
        }
    };
}

define_bitset_impl!(u8);
define_bitset_impl!(u16);
define_bitset_impl!(u32);
define_bitset_impl!(u64);
define_bitset_impl!(u128);
