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
        + SaturatingAdd<Output = U>
        + SaturatingSub<Output = U>
        + core::ops::Add<U, Output = U>
        + Bounded
        + Zero
        + core::ops::Sub<U, Output = U>,
    S: Copy + core::cmp::PartialOrd + Abs + Zero + TryIntoPatch<U>,
{
    fn add_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            (value)
                .try_into_value()
                .map_or_else(|| self.add(U::max_bound()), |pos_val| self.add(pos_val))
        } else {
            (value.abs())
                .try_into_value()
                .map_or_else(|| self.add(U::max_bound()), |sub_val| self.add(sub_val))
        }
    }

    fn saturated_add_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            (value)
                .try_into_value()
                .map_or_else(|| U::max_bound(), |pos_val| self.saturating_add(pos_val))
        } else {
            (value.abs())
                .try_into_value()
                .map_or_else(|| U::zero(), |sub_val| self.saturating_sub(sub_val))
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

/// Core trait: subtraction of a signed number from an unsigned number (returning)
pub const trait SubSignLogic<T> {
    /// Simple subtraction of the possibly negative number (returning)
    #[must_use]
    fn sub_sign(&self, value: T) -> Self;

    /// Subtraction of the possibly negative number without over/underflowing (returning)
    #[must_use]
    fn saturated_sub_sign(&self, value: T) -> Self;
}

/// Trait for mutating setters, automatically implemented using `SubSignLogic`
pub const trait SubSignSetter<T>: SubSignLogic<T> {
    /// Simple subtraction of the possibly negative number (mutating)
    fn set_sub_sign(&mut self, value: T);

    /// Subtraction of the possibly negative number without over/underflowing (mutating)
    fn set_saturated_sub_sign(&mut self, value: T);
}

impl<U, S> SubSignLogic<S> for U
where
    U: Copy
        + SaturatingAdd<Output = U>
        + SaturatingSub<Output = U>
        + core::ops::Add<U, Output = U>
        + Bounded
        + Zero
        + core::ops::Sub<U, Output = U>,
    S: Copy + core::cmp::PartialOrd + Abs + Zero + TryIntoPatch<U>,
{
    fn sub_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            value
                .try_into_value()
                .map_or_else(|| self.sub(U::max_bound()), |sub_val| self.sub(sub_val))
        } else {
            value
                .abs()
                .try_into_value()
                .map_or_else(|| self.add(U::max_bound()), |add_val| self.add(add_val))
        }
    }

    fn saturated_sub_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            value
                .try_into_value()
                .map_or_else(|| U::zero(), |sub_val| self.saturating_sub(sub_val))
        } else {
            value
                .abs()
                .try_into_value()
                .map_or_else(|| U::max_bound(), |add_val| self.saturating_add(add_val))
        }
    }
}

impl<U, S> SubSignSetter<S> for U
where
    U: SubSignLogic<S>,
{
    fn set_sub_sign(&mut self, value: S) {
        *self = self.sub_sign(value);
    }

    fn set_saturated_sub_sign(&mut self, value: S) {
        *self = self.saturated_sub_sign(value);
    }
}

/// Core trait: multiplication of an unsigned number by a signed number (returning)
pub const trait MulSignLogic<T> {
    /// Simple multiplication by the possibly negative number (returning)
    #[must_use]
    fn mul_sign(&self, value: T) -> Self;

    /// Multiplication by the possibly negative number without over/underflowing (returning)
    #[must_use]
    fn saturated_mul_sign(&self, value: T) -> Self;
}

/// Trait for mutating setters, automatically implemented using `MulSignLogic`
pub const trait MulSignSetter<T>: MulSignLogic<T> {
    /// Multiplication by the possibly negative number (mutating)
    fn set_mul_sign(&mut self, value: T);

    /// Multiplication by the possibly negative number without over/underflowing (mutating)
    fn set_saturated_mul_sign(&mut self, value: T);
}

impl<U, S> MulSignLogic<S> for U
where
    U: Copy
        + SaturatingMul<Output = U>
        + core::ops::Mul<U, Output = U>
        + Bounded
        + Zero
        + std::ops::Sub<Output = U>,
    S: Copy + core::cmp::PartialOrd + Abs + Zero + TryIntoPatch<U>,
{
    fn mul_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            value
                .try_into_value()
                .map_or_else(|| self.mul(U::max_bound()), |mul_val| self.mul(mul_val))
        } else {
            value.abs().try_into_value().map_or_else(
                || U::zero().sub(self.mul(U::max_bound())),
                |mul_val| U::zero().sub(self.mul(mul_val)),
            )
        }
    }

    fn saturated_mul_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            value
                .try_into_value()
                .map_or_else(|| U::max_bound(), |mul_val| self.saturating_mul(mul_val))
        } else {
            U::zero()
        }
    }
}

impl<U, S> MulSignSetter<S> for U
where
    U: MulSignLogic<S>,
{
    fn set_mul_sign(&mut self, value: S) {
        *self = self.mul_sign(value);
    }

    fn set_saturated_mul_sign(&mut self, value: S) {
        *self = self.saturated_mul_sign(value);
    }
}

/// Core trait: division of an unsigned number by a signed number (returning)
pub const trait DivSignLogic<T> {
    /// Simple division by the possibly negative number (returning)
    #[must_use]
    fn div_sign(&self, value: T) -> Self;

    /// Division by the possibly negative number without over/underflowing (returning)
    #[must_use]
    fn saturated_div_sign(&self, value: T) -> Self;
}

/// Trait for mutating setters, automatically implemented using `DivSignLogic`
pub const trait DivSignSetter<T>: DivSignLogic<T> {
    /// Division by the possibly negative number (mutating)
    fn set_div_sign(&mut self, value: T);

    /// Division by the possibly negative number without over/underflowing (mutating)
    fn set_saturated_div_sign(&mut self, value: T);
}

impl<U, S> DivSignLogic<S> for U
where
    U: Copy + core::ops::Div<U, Output = U> + Bounded + Zero + std::ops::Sub<Output = U>,
    S: Copy + core::cmp::PartialOrd + Abs + Zero + TryIntoPatch<U>,
{
    fn div_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            value
                .try_into_value()
                .map_or_else(|| U::zero(), |div_val| *self / div_val)
        } else {
            value
                .abs()
                .try_into_value()
                .map_or_else(|| U::zero(), |div_val| U::zero().sub(*self / div_val))
        }
    }

    fn saturated_div_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            value
                .try_into_value()
                .map_or_else(|| U::zero(), |div_val| *self / div_val)
        } else {
            U::zero()
        }
    }
}

impl<U, S> DivSignSetter<S> for U
where
    U: DivSignLogic<S>,
{
    fn set_div_sign(&mut self, value: S) {
        *self = self.div_sign(value);
    }

    fn set_saturated_div_sign(&mut self, value: S) {
        *self = self.saturated_div_sign(value);
    }
}
