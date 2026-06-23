use mirl_extensions_conversion::TryIntoPatch;
use mirl_extensions_core::*;

use crate::*;

/// Core trait: addition of a signed number to an unsigned number (returning)
pub const trait AddSignLogic<T> {
    /// Simple addition of the possibly negative number (returning)
    #[must_use]
    fn add_sign(&self, value: T) -> Self;

    /// Addition of the possibly negative number without over/underflowing (returning)
    #[must_use]
    fn saturated_add_sign(&self, value: T) -> Self;
}

/// Trait for mutating setters, automatically implemented using `AddSignLogic`
pub const trait AddSignSetter<T>: AddSignLogic<T> {
    /// Simple addition of the possibly negative number (mutating)
    fn set_add_sign(&mut self, value: T);

    /// Addition of the possibly negative number without over/underflowing (mutating)
    fn set_saturated_add_sign(&mut self, value: T);
}

impl<U, S> AddSignLogic<S> for U
where
    U: Copy
        + WrapOps
        + SaturatingAdd<Output = U>
        + SaturatingSub<Output = U>
        + core::ops::Add<U, Output = U>
        + Bounded
        + ConstZero
        + core::ops::Sub<U, Output = U>,
    S: Copy + core::cmp::PartialOrd + Abs + ConstZero + TryIntoPatch<U>,
{
    fn add_sign(&self, value: S) -> Self {
        if value >= S::ZERO {
            (value).try_into_value().map_or_else(
                || self.wrapping_add(U::max_bound()),
                |pos_val| self.wrapping_add(pos_val),
            )
        } else {
            (value.abs()).try_into_value().map_or_else(
                || self.wrapping_sub(U::max_bound()),
                |sub_val| self.wrapping_sub(sub_val),
            )
        }
    }

    fn saturated_add_sign(&self, value: S) -> Self {
        if value >= S::ZERO {
            (value)
                .try_into_value()
                .map_or_else(|| U::max_bound(), |pos_val| self.saturating_add(pos_val))
        } else {
            (value.abs())
                .try_into_value()
                .map_or_else(|| U::ZERO, |sub_val| self.saturating_sub(sub_val))
        }
    }
}

impl<U, S> AddSignSetter<S> for U
where
    U: AddSignLogic<S>,
{
    fn set_add_sign(&mut self, value: S) {
        *self = self.add_sign(value);
    }

    fn set_saturated_add_sign(&mut self, value: S) {
        *self = self.saturated_add_sign(value);
    }
}
