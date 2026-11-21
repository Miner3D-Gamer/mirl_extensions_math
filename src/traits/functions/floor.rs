/// Round down the current number
pub const trait Floor {
    #[must_use]
    /// Round down the current number
    fn floor(self) -> Self;
}
const impl Floor for f32 {
    fn floor(self) -> Self {
        core::f32::math::floor(self)
    }
}
const impl Floor for f64 {
    fn floor(self) -> Self {
        core::f64::math::floor(self)
    }
}
#[cfg(feature = "f128")]
const impl Floor for f128 {
    fn floor(self) -> Self {
        core::intrinsics::floor(self)
    }
}

#[cfg(feature = "f16")]
const impl Floor for f16 {
    fn floor(self) -> Self {
        core::intrinsics::floor(self)
    }
}
macro_rules! impl_floor_int {
    ($($t:ty),*) => {
        $(
            const impl Floor for $t {
                fn floor(self) -> Self {
                    self
                }
            }
        )*
    };
}

impl_floor_int!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
