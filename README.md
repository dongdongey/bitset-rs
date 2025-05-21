# Simple BitSet

set or get bitflag from index for unsigned intager types

using `u8` like `[bool;8]`!!

```rs
let mut bit = 0_u8;
BitSet::bitset(&mut bit, 3, true);
assert_eq!(bit, 0b00010000);
assert_eq!(bit.bitget(3), true); // you csn use method-like style
bit.bitset(3, false);
assert_eq!(bit.bitget(3), false);
```
