use mirl_extensions_conversion::TryIntoPatch;

use crate::ConstRotationRatio;

/// Convert the current value from patch degrees to radians and from patch radians to degrees
/// T: Self
/// A: Accuracy, usually: f16, f32, f64, or f128
pub const trait AngularConversion<T, A = f64> {
    #[must_use]
    /// Convert angle degrees into angle radians
    fn to_radians(self) -> Self;
    #[must_use]
    /// Convert angle degrees into angle radians
    fn to_degrees(self) -> Self;
}
const impl<
    T: [const] Into<A> + [const] core::marker::Destruct,
    A: [const] Into<T>
        + [const] core::marker::Destruct
        + ConstRotationRatio
        + [const] std::ops::Mul<Output = A>,
> AngularConversion<T, A> for T
{
    fn to_radians(self) -> Self {
        (self.into() * A::DEGREES_TO_RADIAN_RATIO).into()
    }
    fn to_degrees(self) -> Self {
        (self.into() * A::RADIANS_TO_DEGREES_RATIO).into()
    }
}
/// Try to convert the current value from patch degrees to radians and from patch radians to degrees
/// T: Self
/// A: Accuracy, usually: f16, f32, f64, or f128
pub const trait TryAngularConversion<T, A = f64>: std::marker::Sized {
    #[must_use]
    /// Try to convert angle degrees into angle radians
    fn try_to_radians(self) -> Option<Self>;
    #[must_use]
    /// Try to convert angle degrees into angle radians
    fn try_to_degrees(self) -> Option<Self>;
}
const impl<
    T: [const] TryIntoPatch<A> + [const] core::marker::Destruct,
    A: [const] TryIntoPatch<T>
        + [const] core::marker::Destruct
        + ConstRotationRatio
        + [const] std::ops::Mul<Output = A>,
> TryAngularConversion<T, A> for T
{
    fn try_to_radians(self) -> Option<Self> {
        (self.try_into_value()? * A::DEGREES_TO_RADIAN_RATIO).try_into_value()
    }
    fn try_to_degrees(self) -> Option<Self> {
        (self.try_into_value()? * A::RADIANS_TO_DEGREES_RATIO).try_into_value()
    }
}
