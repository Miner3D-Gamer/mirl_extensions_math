use crate::*;
use mirl_extensions_core::*;
#[allow(missing_docs)] // TODO: Docs
/// Get any number between 0 and 3
///
/// An extended version of the [`One`](crate::math::One)/[`Zero`](crate::math::Zero) traits covering all numbers from 0 to 31
pub const trait Numbers2Pow2: SupportsRange2Pow2 + [const] Numbers2Pow1 {
    fn num_2() -> Self;
    fn num_3() -> Self;
}
const impl<T: SupportsRange2Pow2 + const Numbers2Pow1 + const core::ops::Add<Output = T>>
    Numbers2Pow2 for T
{
    fn num_2() -> Self {
        Self::num_1() + Self::num_1()
    }
    fn num_3() -> Self {
        Self::num_2() + Self::num_1()
    }
}
