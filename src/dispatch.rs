use crate::*;

pub fn bitset_dispatch<T: BitSet>(container: &mut [T], index: usize, b: bool) {
    let bits = core::mem::size_of::<T>() * 8;
    container[index / bits].bitset(index % bits, b);
}

pub fn bitget_dispatch<T: BitGet>(container: &[T], index: usize) -> bool {
    let bits = core::mem::size_of::<T>() * 8;
    container[index / bits].bitget(index % bits)
}

pub fn bittogle_dispatch<T: BitTogle>(container: &mut [T], index: usize) {
    let bits = core::mem::size_of::<T>() * 8;
    container[index / bits].bittogle(index % bits);
}
