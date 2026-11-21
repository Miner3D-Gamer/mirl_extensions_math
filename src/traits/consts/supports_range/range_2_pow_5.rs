use crate::*;
use mirl_extensions_core::*;
#[allow(missing_docs)] // TODO: Docs
/// Get any number between 0 and 31
///
/// An extended version of the [`One`](crate::math::One)/[`Zero`](crate::math::Zero) traits covering all numbers from 0 to 31
pub const trait Numbers2Pow5: SupportsRange2Pow5 + [const] Numbers2Pow4 {
    fn num_16() -> Self;
    fn num_17() -> Self;
    fn num_18() -> Self;
    fn num_19() -> Self;
    fn num_20() -> Self;
    fn num_21() -> Self;
    fn num_22() -> Self;
    fn num_23() -> Self;
    fn num_24() -> Self;
    fn num_25() -> Self;
    fn num_26() -> Self;
    fn num_27() -> Self;
    fn num_28() -> Self;
    fn num_29() -> Self;
    fn num_30() -> Self;
    fn num_31() -> Self;
}
const impl<T: SupportsRange2Pow5 + const Numbers2Pow4 + const core::ops::Add<Output = T>>
    Numbers2Pow5 for T
{
    fn num_16() -> Self {
        Self::num_15() + Self::num_1()
    }
    fn num_17() -> Self {
        Self::num_16() + Self::num_1()
    }
    fn num_18() -> Self {
        Self::num_17() + Self::num_1()
    }
    fn num_19() -> Self {
        Self::num_18() + Self::num_1()
    }
    fn num_20() -> Self {
        Self::num_19() + Self::num_1()
    }
    fn num_21() -> Self {
        Self::num_20() + Self::num_1()
    }
    fn num_22() -> Self {
        Self::num_21() + Self::num_1()
    }
    fn num_23() -> Self {
        Self::num_22() + Self::num_1()
    }
    fn num_24() -> Self {
        Self::num_23() + Self::num_1()
    }
    fn num_25() -> Self {
        Self::num_24() + Self::num_1()
    }
    fn num_26() -> Self {
        Self::num_25() + Self::num_1()
    }
    fn num_27() -> Self {
        Self::num_26() + Self::num_1()
    }
    fn num_28() -> Self {
        Self::num_27() + Self::num_1()
    }
    fn num_29() -> Self {
        Self::num_28() + Self::num_1()
    }
    fn num_30() -> Self {
        Self::num_29() + Self::num_1()
    }
    fn num_31() -> Self {
        Self::num_30() + Self::num_1()
    }
}
