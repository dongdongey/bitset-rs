#![no_std]

pub trait BitSet: BitGet {
    fn bitset(&mut self, index: usize, b: bool);
}
pub trait BitGet {
    fn bitget(&self, index: usize) -> bool;
}
pub trait BitTogle {
    fn bittogle(&mut self, index: usize);
}

pub(crate) mod dispatch;
mod for_container;
mod impl_macro;

#[cfg(test)]
mod test;
