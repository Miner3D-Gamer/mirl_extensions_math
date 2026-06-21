/// Calculate what e^x will result in the given value
pub const trait NaturalLog {
    /// Calculate what e^x will result in the given value
    #[must_use]
    fn natural_log(self) -> Self;
}

// TODO: Implement NaturalLog for all U and I types

impl NaturalLog for f16 {
    fn natural_log(self) -> Self {
        core::intrinsics::logf16(self)
    }
}
impl NaturalLog for f32 {
    fn natural_log(self) -> Self {
        core::intrinsics::logf32(self)
    }
}

impl NaturalLog for f64 {
    fn natural_log(self) -> Self {
        core::intrinsics::logf64(self)
    }
}
impl NaturalLog for f128 {
    fn natural_log(self) -> Self {
        core::intrinsics::logf128(self)
    }
}
