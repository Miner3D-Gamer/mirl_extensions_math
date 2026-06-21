/// Trait providing fused multiply-add operation
pub const trait MulAdd {
    /// Computes `(self * a) + b` with only one rounding error if possible
    #[must_use]
    fn mul_add(self, a: Self, b: Self) -> Self;
}

const impl MulAdd for f16 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::intrinsics::fmaf16(self, a, b)
    }
}

const impl MulAdd for f32 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::f32::math::mul_add(self, a, b)
    }
}

const impl MulAdd for f64 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::f64::math::mul_add(self, a, b)
    }
}

const impl MulAdd for f128 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::intrinsics::fmaf128(self, a, b)
    }
}
macro_rules! impl_mul_add_plain {
    ($($t:ty),* $(,)?) => {
        $(
            impl const MulAdd for $t {
                #[inline(always)]
                fn mul_add(self, a: Self, b: Self) -> Self {
                    self * a + b
                }
            }
        )*
    };
}

impl_mul_add_plain!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);
