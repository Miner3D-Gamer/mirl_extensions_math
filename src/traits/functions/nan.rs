use std::marker::Destruct;

use mirl_extensions_core::{IsPositive, IsZero, NegativeOne, One, impl_const_trait, impl_trait};

/// Get the nan value of the current type
pub const trait GetNan {
    /// Get the nan value of the current type
    fn nan() -> Self;
}
#[cfg(feature = "f128")]
impl_const_trait!(GetNan {fn nan()->Self{Self::NAN}} for f128);
#[cfg(feature = "f16")]
impl_const_trait!(GetNan {fn nan()->Self{Self::NAN}} for f16);
impl_const_trait!(GetNan {fn nan()->Self{Self::NAN}} for f32, f64);

/// Check if the current value is nan
pub const trait IsNan {
    /// Check if the current value is nan
    fn is_nan(&self) -> bool;
}
// /// Check if the current value isn't nan
// pub trait IsNotNan {
//     /// Check if the current value isn't nan
//     fn is_not_nan(&self) -> bool;
// }
impl_const_trait!(IsNan {fn is_nan(&self)->bool{false}} for u8, u16, u32, u64 ,u128 ,usize ,i8 ,i16 ,i32, i64, i128, isize);

impl_trait!(IsNan {fn is_nan(&self)->bool{Self::nan().eq(self)}} for f32, f64);
#[cfg(feature = "f16")]
impl_trait!(IsNan {fn is_nan(&self)->bool{Self::nan().eq(self)}} for f16);
#[cfg(feature = "f128")]
impl_trait!(IsNan {fn is_nan(&self)->bool{Self::nan().eq(self)}} for  f128);

/// Get the inf value of the current type
pub const trait GetNegativeInfinity {
    /// Get the inf value of the current type
    fn negative_infinity() -> Self;
}
impl_const_trait!(GetNegativeInfinity {fn negative_infinity()->Self{Self::NEG_INFINITY}} for f32, f64);
#[cfg(feature = "f128")]
impl_const_trait!(GetNegativeInfinity {fn negative_infinity()->Self{Self::NEG_INFINITY}} for f128);
#[cfg(feature = "f16")]
impl_const_trait!(GetNegativeInfinity {fn negative_infinity()->Self{Self::NEG_INFINITY}} for f16);


/// Check if the current value is negative inf
pub const trait IsNegativeInfinity {
    /// Check if the current value is negative inf
    fn is_negative_infinity(&self) -> bool;
}

impl_trait!(IsNegativeInfinity {fn is_negative_infinity(&self)->bool{false}} for u8, u16, u32, u64 ,u128 ,usize ,i8 ,i16 ,i32, i64, i128, isize);

const impl<T: [const] GetNegativeInfinity + [const] PartialEq + [const] core::marker::Destruct>
    IsNegativeInfinity for T
{
    fn is_negative_infinity(&self) -> bool {
        Self::negative_infinity().eq(self)
    }
}
/// Get the inf value of the current type
pub const trait GetPositiveInfinity {
    /// Get the inf value of the current type
    fn positive_infinity() -> Self;
}
impl_const_trait!(GetPositiveInfinity {fn positive_infinity()->Self{Self::INFINITY}} for f32, f64);
#[cfg(feature = "f128")]
impl_const_trait!(GetPositiveInfinity {fn positive_infinity()->Self{Self::INFINITY}} for f128);
#[cfg(feature = "f16")]
impl_const_trait!(GetPositiveInfinity {fn positive_infinity()->Self{Self::INFINITY}} for f16);

/// Check if the current value is inf
pub const trait IsPositiveInfinity {
    /// Check if the current value is inf
    fn is_positive_infinity(&self) -> bool;
}

impl_trait!(IsPositiveInfinity {fn is_positive_infinity(&self)->bool{false}} for u8, u16, u32, u64 ,u128 ,usize ,i8 ,i16 ,i32, i64, i128, isize);

const impl<T: [const] GetPositiveInfinity + [const] PartialEq + [const] core::marker::Destruct>
    IsPositiveInfinity for T
{
    fn is_positive_infinity(&self) -> bool {
        Self::positive_infinity().eq(self)
    }
}
// impl_trait!(IsPosInf {fn is_pos_inf(&self)->bool{Self::pos_inf().eq(self)}} for f16, f32, f64, f128);

/// Check if the current value is inf
pub const trait IsInfinite {
    /// Check if the current value is inf
    fn is_infinity(&self) -> bool;
}
const impl<T: [const] IsPositiveInfinity + [const] IsNegativeInfinity> IsInfinite for T {
    fn is_infinity(&self) -> bool {
        self.is_positive_infinity() || self.is_negative_infinity()
    }
}
/// Checks if the value is between 0 and 1 (inclusive)
pub const trait IsNormalized {
    /// Checks if the value is between 0 and 1 (inclusive)
    fn is_normalized(&self) -> bool;
}

const impl<
    T: [const] IsZero
        + [const] IsPositive
        + [const] core::cmp::PartialOrd
        + [const] One
        + [const] Destruct,
> IsNormalized for T
{
    fn is_normalized(&self) -> bool {
        (self.is_zero() || self.is_positive()) && Self::one().ge(self)
    }
}

/// Checks if the value is between -1 and 1 (inclusive)
pub const trait IsSymmetricallyNormalized {
    /// Checks if the value is between -1 and 1 (inclusive)
    fn is_symmetrically_normalized(&self) -> bool;
}

const impl<T: [const] core::cmp::PartialOrd + [const] One + [const] NegativeOne + [const] Destruct>
    IsSymmetricallyNormalized for T
{
    fn is_symmetrically_normalized(&self) -> bool {
        Self::negative_one().le(self) && Self::one().ge(self)
    }
}
