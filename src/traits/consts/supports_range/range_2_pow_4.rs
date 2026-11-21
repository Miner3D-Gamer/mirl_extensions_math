use crate::*;
use mirl_extensions_core::*;
#[allow(missing_docs)] // TODO: Docs
/// Get any number between 0 and 15
///
/// An extended version of the [`One`](crate::math::One)/[`Zero`](crate::math::Zero) traits covering all numbers from 0 to 15
pub const trait Numbers2Pow4: SupportsRange2Pow4 + [const] Numbers2Pow3 {
    fn num_8() -> Self;
    fn num_9() -> Self;
    fn num_10() -> Self;
    fn num_11() -> Self;
    fn num_12() -> Self;
    fn num_13() -> Self;
    fn num_14() -> Self;
    fn num_15() -> Self;
}
const impl<T: SupportsRange2Pow4 + const Numbers2Pow3 + const core::ops::Add<Output = T>>
    Numbers2Pow4 for T
{
    fn num_8() -> Self {
        Self::num_7() + Self::num_1()
    }
    fn num_9() -> Self {
        Self::num_8() + Self::num_1()
    }
    fn num_10() -> Self {
        Self::num_9() + Self::num_1()
    }
    fn num_11() -> Self {
        Self::num_10() + Self::num_1()
    }
    fn num_12() -> Self {
        Self::num_11() + Self::num_1()
    }
    fn num_13() -> Self {
        Self::num_12() + Self::num_1()
    }
    fn num_14() -> Self {
        Self::num_13() + Self::num_1()
    }
    fn num_15() -> Self {
        Self::num_14() + Self::num_1()
    }
}
