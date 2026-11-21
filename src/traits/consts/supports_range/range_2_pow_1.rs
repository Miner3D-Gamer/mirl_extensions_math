use mirl_extensions_core::*;
#[allow(missing_docs)] // TODO: Docs
/// Get any number between 0 and 1
///
/// An extended version of the [`One`](crate::math::One)/[`Zero`](crate::math::Zero) traits covering all numbers from 0 to 31
pub const trait Numbers2Pow1: SupportsRange2Pow1 {
    fn num_0() -> Self;
    fn num_1() -> Self;
}
const impl<T: SupportsRange2Pow1 + const One + const Zero + const core::ops::Add<Output = T>>
    Numbers2Pow1 for T
{
    fn num_0() -> Self {
        Self::zero()
    }
    fn num_1() -> Self {
        Self::one()
    }
}
