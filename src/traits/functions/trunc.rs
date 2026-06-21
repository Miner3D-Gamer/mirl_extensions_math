/// Removes the decimal part of a number.
///
/// In the positives it act's as `.floor()` while in the negatives as `.top()`. It's a switch.
pub const trait Trunc {
    /// Removes the decimal part of a number.
    #[must_use]
    fn trunc(self) -> Self;
}

// TODO: Implement Fract for all U and I types

impl Trunc for f16 {
    fn trunc(self) -> Self {
        core::intrinsics::truncf16(self)
    }
}
impl Trunc for f32 {
    fn trunc(self) -> Self {
        core::intrinsics::truncf32(self)
    }
}

impl Trunc for f64 {
    fn trunc(self) -> Self {
        core::intrinsics::truncf64(self)
    }
}
impl Trunc for f128 {
    fn trunc(self) -> Self {
        core::intrinsics::truncf128(self)
    }
}
