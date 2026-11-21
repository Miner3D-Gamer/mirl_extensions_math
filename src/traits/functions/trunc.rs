/// Removes the decimal part of a number.
///
/// In the positives it act's as `.floor()` while in the negatives as `.top()`. It's a switch.
pub const trait Trunc {
    /// Removes the decimal part of a number.
    #[must_use]
    fn trunc(self) -> Self;
}

// TODO: Implement Fract for all U and I types

#[cfg(feature = "f16")]
impl Trunc for f16 {
    fn trunc(self) -> Self {
        core::intrinsics::trunc(self)
    }
}
impl Trunc for f32 {
    fn trunc(self) -> Self {
        core::f32::math::trunc(self)
    }
}

impl Trunc for f64 {
    fn trunc(self) -> Self {
        core::f64::math::trunc(self)
    }
}
#[cfg(feature = "f128")]
impl Trunc for f128 {
    fn trunc(self) -> Self {
        core::intrinsics::trunc(self)
    }
}
