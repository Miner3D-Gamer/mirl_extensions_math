use mirl_extensions_core::{ConstOne, ConstZero};

use crate::NextUpDown;

/// A trait for getting the smallest value >0 and the biggest value <1
pub const trait UniformPreviousNext {
    /// Get the smallest value >0
    #[must_use]
    fn smallest_bigger_than_zero() -> Self;
    /// Get the biggest value <0
    #[must_use]
    fn biggest_smaller_than_one() -> Self;
}

impl<T: ConstZero + ConstOne + NextUpDown> UniformPreviousNext for T {
    fn biggest_smaller_than_one() -> Self {
        Self::ZERO.next_up()
    }
    fn smallest_bigger_than_zero() -> Self {
        Self::ONE.next_down()
    }
}
