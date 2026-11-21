use crate::*;
use mirl_extensions_core::*;
#[allow(missing_docs)] // TODO: Docs
/// Get any number between 0 and 7
///
/// An extended version of the [`One`](crate::math::One)/[`Zero`](crate::math::Zero) traits covering all numbers from 0 to 7
pub const trait Numbers2Pow3:  SupportsRange2Pow3 + [const] Numbers2Pow2 {
    fn num_4() -> Self;
    fn num_5() -> Self;
    fn num_6() -> Self;
    fn num_7() -> Self;
}
const impl<T: SupportsRange2Pow3 + const Numbers2Pow2 + const core::ops::Add<Output = T>>
    Numbers2Pow3 for T
{
    fn num_4() -> Self {
        Self::num_3() + Self::num_1()
    }
    fn num_5() -> Self {
        Self::num_4() + Self::num_1()
    }
    fn num_6() -> Self {
        Self::num_5() + Self::num_1()
    }
    fn num_7() -> Self {
        Self::num_6() + Self::num_1()
    }
}
