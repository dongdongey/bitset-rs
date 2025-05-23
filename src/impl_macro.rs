use crate::*;
use core::mem::size_of;

macro_rules! define_bitset_impl {
    ($t:ty) => {
        impl BitSet for $t {
            fn bitset(&mut self, index: usize, b: bool) {
                let init: Self = 1 << (size_of::<Self>() * 8 - 1);
                let set: Self = init >> index;
                *self |= set;
                if !b {
                    *self = *self ^ set;
                }
            }
        }
        impl BitGet for $t{
            fn bitget(&self, index: usize) -> bool {
                let init: Self = 1 as $t << (size_of::<Self>() * 8 - 1);
                let set: Self = init >> index;
                self & set != 0
            }
        }
        impl BitTogle for $t{
            fn bittogle(&mut self, index:usize){
                let init: Self = 1 as $t << (size_of::<Self>() * 8 - 1);
                let set: Self = init >> index;
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
