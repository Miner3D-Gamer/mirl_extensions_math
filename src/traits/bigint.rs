use crate::*;

// To implement a new number, one must implement these traits:
// ConstOne (One)
// ConstZero (Zero)
// Abs

impl Abs for num_bigint::BigInt {
    fn abs(self) -> Self {
        if self > Self::ZERO {
            self
        } else {
            -self
        }
    }
}

impl Abs for num_bigint::BigUint {
    fn abs(self) -> Self {
        self
    }
}
