#[cfg(test)]
use crate::{BitGet, BitSet};

#[test]
fn it_works_at_u8() {
    let mut bit = 0_u8;
    BitSet::bitset(&mut bit, 3, true);
    assert_eq!(bit, 0b0001_0000);
    assert_eq!(bit.bitget(3), true);
    BitSet::bitset(&mut bit, 3, false);
    assert_eq!(bit, 0b0000_0000);

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
    extern crate std;
    let mut bit: std::vec::Vec<u8> = std::vec![0_u8; 3];
    bit.bitset(20, true);
    assert_eq!(bit.bitget(20), true);
    bit.bitset(20, false);
    assert_eq!(bit.bitget(20), false);
}
