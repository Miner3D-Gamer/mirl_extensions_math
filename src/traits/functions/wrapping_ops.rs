/// A unified trait providing wrapping arithmetic operations for all numeric types
pub const trait WrapOps {
    /// Wrapping addition. Computes `self + other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_add(self, other: Self) -> Self;

    /// Wrapping subtraction. Computes `self - other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_sub(self, other: Self) -> Self;

    /// Wrapping multiplication. Computes `self * other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_mul(self, other: Self) -> Self;

    /// Wrapping multiplication. Computes `self * other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_div(self, other: Self) -> Self;
}

macro_rules! impl_wrap_ops_int {
    ($($t:ty)*) => ($(
        const impl WrapOps for $t {
            #[inline]
            fn wrapping_add(self, other: Self) -> Self {
                <$t>::wrapping_add(self, other)
            }

            #[inline]
            fn wrapping_sub(self, other: Self) -> Self {
                <$t>::wrapping_sub(self, other)
            }

            #[inline]
            fn wrapping_mul(self, other: Self) -> Self {
                <$t>::wrapping_mul(self, other)
            }
            #[inline]
            fn wrapping_div(self, other: Self) -> Self {
                <$t>::wrapping_div(self, other)
            }
        }
    )*)
}

macro_rules! impl_wrap_ops_float {
    ($($t:ty)*) => ($(
        const impl WrapOps for $t {
            #[inline]
            fn wrapping_add(self, other: Self) -> Self {
                self + other
            }

            #[inline]
            fn wrapping_sub(self, other: Self) -> Self {
                self - other
            }

            #[inline]
            fn wrapping_mul(self, other: Self) -> Self {
                self * other
            }
            #[inline]
            fn wrapping_div(self, other: Self) -> Self {
                self / other
            }
        }
    )*)
}

impl_wrap_ops_int! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
impl_wrap_ops_float! {f16 f32 f64 f128}
