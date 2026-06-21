/// Round up the current number
pub const trait Ceil {
    #[must_use]
    /// Round up the current number
    fn ceil(self) -> Self;
}

const impl Ceil for f64 {
    fn ceil(self) -> Self {
        core::f64::math::ceil(self)
    }
}

const impl Ceil for f16 {
    fn ceil(self) -> Self {
        core::intrinsics::ceilf16(self)
    }
}

const impl Ceil for f128 {
    fn ceil(self) -> Self {
        core::intrinsics::ceilf128(self)
    }
}

const impl Ceil for f32 {
    fn ceil(self) -> Self {
        core::f32::math::ceil(self)
    }
}
macro_rules! impl_ceil_int {
    ($($t:ty),*) => {
        $(
            impl const Ceil for $t {
                fn ceil(self) -> Self {
                    self
                }
            }
        )*
    };
}

impl_ceil_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
