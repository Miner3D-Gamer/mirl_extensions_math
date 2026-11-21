use mirl_extensions_core::*;
use crate::*;
#[allow(missing_docs)] // TODO: Docs
/// Get any number between 0 and 63
///
/// An extended version of the [`One`](crate::math::One)/[`Zero`](crate::math::Zero) traits covering all numbers from 0 to 63
pub const trait Numbers2Pow6: SupportsRange2Pow6 + [const] Numbers2Pow5 {
    fn num_32() -> Self;
    fn num_33() -> Self;
    fn num_34() -> Self;
    fn num_35() -> Self;
    fn num_36() -> Self;
    fn num_37() -> Self;
    fn num_38() -> Self;
    fn num_39() -> Self;
    fn num_40() -> Self;
    fn num_41() -> Self;
    fn num_42() -> Self;
    fn num_43() -> Self;
    fn num_44() -> Self;
    fn num_45() -> Self;
    fn num_46() -> Self;
    fn num_47() -> Self;
    fn num_48() -> Self;
    fn num_49() -> Self;
    fn num_50() -> Self;
    fn num_51() -> Self;
    fn num_52() -> Self;
    fn num_53() -> Self;
    fn num_54() -> Self;
    fn num_55() -> Self;
    fn num_56() -> Self;
    fn num_57() -> Self;
    fn num_58() -> Self;
    fn num_59() -> Self;
    fn num_60() -> Self;
    fn num_61() -> Self;
    fn num_62() -> Self;
    fn num_63() -> Self;
}
const impl<T: SupportsRange2Pow6 + const Numbers2Pow5 + const core::ops::Add<Output = T>>
    Numbers2Pow6 for T
{
    fn num_32() -> Self {
        Self::num_31() + Self::num_1()
    }
    fn num_33() -> Self {
        Self::num_32() + Self::num_1()
    }
    fn num_34() -> Self {
        Self::num_33() + Self::num_1()
    }
    fn num_35() -> Self {
        Self::num_34() + Self::num_1()
    }
    fn num_36() -> Self {
        Self::num_35() + Self::num_1()
    }
    fn num_37() -> Self {
        Self::num_36() + Self::num_1()
    }
    fn num_38() -> Self {
        Self::num_37() + Self::num_1()
    }
    fn num_39() -> Self {
        Self::num_38() + Self::num_1()
    }
    fn num_40() -> Self {
        Self::num_39() + Self::num_1()
    }
    fn num_41() -> Self {
        Self::num_40() + Self::num_1()
    }
    fn num_42() -> Self {
        Self::num_41() + Self::num_1()
    }
    fn num_43() -> Self {
        Self::num_42() + Self::num_1()
    }
    fn num_44() -> Self {
        Self::num_43() + Self::num_1()
    }
    fn num_45() -> Self {
        Self::num_44() + Self::num_1()
    }
    fn num_46() -> Self {
        Self::num_45() + Self::num_1()
    }
    fn num_47() -> Self {
        Self::num_46() + Self::num_1()
    }
    fn num_48() -> Self {
        Self::num_47() + Self::num_1()
    }
    fn num_49() -> Self {
        Self::num_48() + Self::num_1()
    }
    fn num_50() -> Self {
        Self::num_49() + Self::num_1()
    }
    fn num_51() -> Self {
        Self::num_50() + Self::num_1()
    }
    fn num_52() -> Self {
        Self::num_51() + Self::num_1()
    }
    fn num_53() -> Self {
        Self::num_52() + Self::num_1()
    }
    fn num_54() -> Self {
        Self::num_53() + Self::num_1()
    }
    fn num_55() -> Self {
        Self::num_54() + Self::num_1()
    }
    fn num_56() -> Self {
        Self::num_55() + Self::num_1()
    }
    fn num_57() -> Self {
        Self::num_56() + Self::num_1()
    }
    fn num_58() -> Self {
        Self::num_57() + Self::num_1()
    }
    fn num_59() -> Self {
        Self::num_58() + Self::num_1()
    }
    fn num_60() -> Self {
        Self::num_59() + Self::num_1()
    }
    fn num_61() -> Self {
        Self::num_60() + Self::num_1()
    }
    fn num_62() -> Self {
        Self::num_61() + Self::num_1()
    }
    fn num_63() -> Self {
        Self::num_62() + Self::num_1()
    }
}
