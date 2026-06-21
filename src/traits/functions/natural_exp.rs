/// Returns e^self
pub const trait NaturalExp {
    #[must_use]
    /// Returns e^self
    fn natural_exp(self) -> Self;
}
impl NaturalExp for f16 {
    fn natural_exp(self) -> Self {
        core::intrinsics::expf16(self)
    }
}

impl NaturalExp for f32 {
    fn natural_exp(self) -> Self {
        core::intrinsics::expf32(self)
    }
}

impl NaturalExp for f64 {
    fn natural_exp(self) -> Self {
        core::intrinsics::expf64(self)
    }
}

impl NaturalExp for f128 {
    fn natural_exp(self) -> Self {
        core::intrinsics::expf128(self)
    }
}
