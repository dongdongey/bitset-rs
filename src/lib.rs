#![no_std]

pub trait BitSet {
    fn bitset(&mut self, index: usize, b: bool);
}
pub trait BitGet {
    fn bitget(&self, index: usize) -> bool;
}
pub trait BitTogle {
    fn bittogle(&mut self, index: usize);
}

mod for_container;
mod impl_macro;

#[cfg(test)]
mod test;
