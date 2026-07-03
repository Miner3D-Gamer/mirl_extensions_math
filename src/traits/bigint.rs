use crate::*;

// To implement a new number, one must implement these traits:
// One (One)
// Zero (Zero)
// Abs
use mirl_extensions_core::Zero;
impl Abs for num_bigint::BigInt {
    fn abs(self) -> Self {
        if self > Self::zero() { self } else { -self }
    }
}

impl Abs for num_bigint::BigUint {
    fn abs(self) -> Self {
        self
    }
}
