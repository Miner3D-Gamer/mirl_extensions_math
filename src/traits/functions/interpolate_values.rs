use mirl_extensions_conversion::IntoPatch;
use mirl_extensions_core::*;

/// Interpolate between 2 values with Self being the progress
pub const trait InterpolateAsInterpolatorLegacy<V1, V2 = V1, Output = V1> {
    /// Interpolate between 2 values with Self being the progress
    fn interpolate_values_legacy(self, val1: V1, val2: V2) -> Output;
}
const impl<
    S: ConstZero
        + ConstOne
        + [const] core::ops::Sub<Output = S>
        + [const] core::ops::Add<Output = S>
        + [const] core::ops::Mul<Output = S>
        + Copy
        + SupportsDecimalRange0To1,
    V1: [const] Into<S>,
    V2: [const] Into<S>,
> InterpolateAsInterpolatorLegacy<V1, V2, S> for S
{
    fn interpolate_values_legacy(self, val1: V1, val2: V2) -> Self {
        (S::ONE - self) * val1.into() + self * val2.into()
    }
}
/// Interpolate between 2 values with Self being the progress
pub const trait InterpolateAsInterpolator<V1, V2 = V1, Output = V1> {
    /// Interpolate between 2 values with Self being the progress
    fn interpolate_values(self, val1: V1, val2: V2) -> Output;
}
const impl<
    S: ConstZero
        + ConstOne
        + [const] core::ops::Sub<Output = S>
        + [const] core::ops::Add<Output = S>
        + [const] core::ops::Mul<Output = S>
        + Copy
        + SupportsDecimalRange0To1,
    V1: [const] IntoPatch<S>,
    V2: [const] IntoPatch<S>,
> InterpolateAsInterpolator<V1, V2, S> for S
{
    fn interpolate_values(self, val1: V1, val2: V2) -> Self {
        (S::ONE - self) * val1.into_value() + self * val2.into_value()
    }
}

/// Interpolate between this and another value
///
/// To get this implemented for a type:
/// Pick a progress type, f64 for example
///
/// (Simple: Interpolate with self with accuracy of f64)
/// Implement `Into<f64>` and `Into<f64>` for your type
///
/// (Complex: Interpolate with T with accuracy of f64 and return O)
/// Pick a progress type, f64 for example
/// Implement `Into<f64>` for your type
/// Implement `Into<f64>` for `T`
/// Implement `FromPatchPatch<f64>` for `O`
pub const trait InterpolateBetween<P, V2 = Self, Output = Self> {
    /// Interpolate between this and another value
    fn interpolate_with(self, other: V2, progress: P) -> Output;
}
const impl<Progress: [const] InterpolateAsInterpolator<T, V2, Output>, T, V2, Output>
    InterpolateBetween<Progress, V2, Output> for T
{
    fn interpolate_with(self, other: V2, progress: Progress) -> Output {
        progress.interpolate_values(self, other)
    }
}
