/// Calculate what e^x will result in the given value
pub const trait NaturalLog {
    /// Calculate what e^x will result in the given value
    #[must_use]
    fn natural_log(self) -> Self;
}

// TODO: Implement NaturalLog for all U and I types

#[cfg(feature = "f16")]
impl NaturalLog for f16 {
    fn natural_log(self) -> Self {
        core::intrinsics::log(self)
    }
}
impl NaturalLog for f32 {
    fn natural_log(self) -> Self {
        core::intrinsics::log(self)
    }
}

impl NaturalLog for f64 {
    fn natural_log(self) -> Self {
        core::intrinsics::log(self)
    }
}
#[cfg(feature = "f128")]
impl NaturalLog for f128 {
    fn natural_log(self) -> Self {
        core::intrinsics::log(self)
    }
}
