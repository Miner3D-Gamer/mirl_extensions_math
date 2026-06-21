/// Clamp a value between a minimum and maximum bound
///
/// The function isn't called `clamp` for compatibility with `core::ops::Ord` which numbers like `core::f64` do not implement
pub const trait Clamp {
    /// Restricts a value to be within a specified range [min, max]
    #[must_use]
    fn clamped(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_clamp_int {
    ($($t:ty),*) => {
        $(
            impl const Clamp for $t {
                fn clamped(self, min: Self, max: Self) -> Self {
                    if self < min {
                        min
                    } else if self > max {
                        max
                    } else {
                        self
                    }
                }
            }
        )*
    };
}

macro_rules! impl_clamp_float {
    ($($t:ty),*) => {
        $(
            impl const Clamp for $t {
                fn clamped(self, min: Self, max: Self) -> Self {
                    if self.is_nan() || min.is_nan() || max.is_nan() {
                        return self;
                    }

                    if self < min {
                        min
                    } else if self > max {
                        max
                    } else {
                        self
                    }
                }
            }
        )*
    };
}

impl_clamp_int!(i8, i16, i32, i64, i128, isize);

impl_clamp_int!(u8, u16, u32, u64, u128, usize);

impl_clamp_float!(f16, f32, f64, f128);
