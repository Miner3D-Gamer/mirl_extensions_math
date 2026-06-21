/// Take the sqrt of the given value
pub const trait Sqrt {
    #[must_use]
    /// Take the sqrt of the given value and return the result
    fn sqrt(self) -> Self;
}
impl Sqrt for f16 {
    fn sqrt(self) -> Self {
        core::intrinsics::sqrtf16(self)
    }
}
impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        core::f32::math::sqrt(self)
    }
}
impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        core::f64::math::sqrt(self)
    }
}
impl Sqrt for f128 {
    fn sqrt(self) -> Self {
        core::intrinsics::sqrtf128(self)
    }
}

macro_rules! impl_sqrt {
    ($($t:ty),*) => {
        $(
            impl Sqrt for $t {
                fn sqrt(self) -> Self {
                    // There has to be a better way
                    core::f64::math::sqrt(self as f64) as Self
                }
            }
        )*
    };
}

impl_sqrt!(i8);
impl_sqrt!(i16);
impl_sqrt!(i32);
impl_sqrt!(i64);
impl_sqrt!(i128);
impl_sqrt!(isize);
impl_sqrt!(u8);
impl_sqrt!(u16);
impl_sqrt!(u32);
impl_sqrt!(u64);
impl_sqrt!(u128);
impl_sqrt!(usize);
