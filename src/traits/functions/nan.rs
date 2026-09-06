use mirl_extensions_core::{impl_const_trait, impl_trait};

/// Check if the current value is nan
pub trait IsNan {
    /// Check if the current value is nan
    fn is_nan(&self) -> bool;
}
/// Get the nan value of the current type
pub const trait GetNan {
    /// Get the nan value of the current type
    fn nan() -> Self;
}
impl_const_trait!(GetNan {fn nan()->Self{Self::NAN}} for f16, f32, f64, f128);

/// Get the inf value of the current type
pub const trait GetInf {
    /// Get the inf value of the current type
    fn inf() -> Self;
}
impl_const_trait!(GetInf {fn inf()->Self{Self::INFINITY}} for f16, f32, f64, f128);

/// Get the inf value of the current type
pub const trait GetNegInf {
    /// Get the inf value of the current type
    fn neg_inf() -> Self;
}
impl_const_trait!(GetNegInf {fn neg_inf()->Self{Self::NEG_INFINITY}} for f16, f32, f64, f128);

impl_trait!(IsNan {fn is_nan(&self)->bool{false}} for u8, u16, u32, u64 ,u128 ,usize ,i8 ,i16 ,i32, i64, i128, isize);

impl_trait!(IsNan {fn is_nan(&self)->bool{Self::nan().eq(self)}} for f16, f32, f64, f128);

/// Check if the current value is inf
pub trait IsInf {
    /// Check if the current value is inf
    fn is_inf(&self) -> bool;
}

impl_trait!(IsInf {fn is_inf(&self)->bool{false}} for u8, u16, u32, u64 ,u128 ,usize ,i8 ,i16 ,i32, i64, i128, isize);

impl_trait!(IsInf {fn is_inf(&self)->bool{Self::inf().eq(self)}} for f16, f32, f64, f128);
