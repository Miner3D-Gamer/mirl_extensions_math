/// Calculate what 10^x will result in the given value
pub const trait Log10 {
    /// Calculate what 10^x will result in the given value
    #[must_use]
    fn log10(self) -> Self;
}

// TODO: Implement Log10 for all U and I types

impl Log10 for f16 {
    fn log10(self) -> Self {
        core::intrinsics::log10f16(self)
    }
}
impl Log10 for f32 {
    fn log10(self) -> Self {
        core::intrinsics::log10f32(self)
    }
}

impl Log10 for f64 {
    fn log10(self) -> Self {
        core::intrinsics::log10f64(self)
    }
}
impl Log10 for f128 {
    fn log10(self) -> Self {
        core::intrinsics::log10f128(self)
    }
}
