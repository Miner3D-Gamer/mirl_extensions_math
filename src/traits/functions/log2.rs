/// Calculate what 2^x will result in the given value
pub const trait Log2 {
    /// Calculate what 2^x will result in the given value
    #[must_use]
    fn log2(self) -> Self;
}

// TODO: Implement Log2 for all U and I types

impl Log2 for f16 {
    fn log2(self) -> Self {
        core::intrinsics::log2f16(self)
    }
}
impl Log2 for f32 {
    fn log2(self) -> Self {
        core::intrinsics::log2f32(self)
    }
}

impl Log2 for f64 {
    fn log2(self) -> Self {
        core::intrinsics::log2f64(self)
    }
}
impl Log2 for f128 {
    fn log2(self) -> Self {
        core::intrinsics::log2f128(self)
    }
}
