/// Returns e^self
pub const trait NaturalExp {
    #[must_use]
    /// Returns e^self
    fn natural_exp(self) -> Self;
}
#[cfg(feature = "f16")]
impl NaturalExp for f16 {
    fn natural_exp(self) -> Self {
        core::intrinsics::exp(self)
    }
}

impl NaturalExp for f32 {
    fn natural_exp(self) -> Self {
        core::intrinsics::exp(self)
    }
}

impl NaturalExp for f64 {
    fn natural_exp(self) -> Self {
        core::intrinsics::exp(self)
    }
}

#[cfg(feature = "f128")]
impl NaturalExp for f128 {
    fn natural_exp(self) -> Self {
        core::intrinsics::exp(self)
    }
}
