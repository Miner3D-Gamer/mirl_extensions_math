use mirl_extensions_conversion::IntoPatch;

/// Trait providing fused multiply-add operation
pub const trait MulAdd<Rhs = Self, Rhs2 = Self, Output = Self> {
    /// Computes `(self * a) + b` with only one rounding error if possible
    #[must_use]
    fn mul_add(self, a: Rhs, b: Rhs2) -> Self;
}

#[cfg(feature = "f16")]
const impl<Rhs: [const] IntoPatch<Self>, Rhs2: [const] IntoPatch<Self>> MulAdd<Rhs, Rhs2> for f16 {
    fn mul_add(self, a: Rhs, b: Rhs2) -> Self {
        core::intrinsics::fma(self, a.into_value(), b.into_value())
    }
}

const impl<Rhs: [const] IntoPatch<Self>, Rhs2: [const] IntoPatch<Self>> MulAdd<Rhs, Rhs2> for f32 {
    fn mul_add(self, a: Rhs, b: Rhs2) -> Self {
        core::f32::math::mul_add(self, a.into_value(), b.into_value())
    }
}

const impl<Rhs: [const] IntoPatch<Self>, Rhs2: [const] IntoPatch<Self>> MulAdd<Rhs, Rhs2> for f64 {
    fn mul_add(self, a: Rhs, b: Rhs2) -> Self {
        core::f64::math::mul_add(self, a.into_value(), b.into_value())
    }
}

#[cfg(feature = "f128")]
const impl<Rhs: [const] IntoPatch<Self>, Rhs2: [const] IntoPatch<Self>> MulAdd<Rhs, Rhs2> for f128 {
    fn mul_add(self, a: Rhs, b: Rhs2) -> Self {
        core::intrinsics::fma(self, a.into_value(), b.into_value())
    }
}
macro_rules! impl_mul_add_plain {
    ($($t:ty),* $(,)?) => {
        $(
            const impl<Rhs: [const] IntoPatch<Self>, Rhs2: [const] IntoPatch<Self>> MulAdd<Rhs, Rhs2> for $t {
                #[inline(always)]
                fn mul_add(self, a: Rhs, b: Rhs2) -> Self {
                    self * a.into_value()+ b.into_value()
                }
            }
        )*
    };
}

impl_mul_add_plain!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);
