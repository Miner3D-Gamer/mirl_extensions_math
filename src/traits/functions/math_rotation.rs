/// Trait providing trigonometric functions
pub const trait MathRotation: std::marker::Sized {
    /// The output type of the operation
    type Output = Self;
    /// Computes the sine of self (in radians)
    #[must_use]
    fn sin(self) -> Self;

    /// Computes the cosine of self (in radians)
    #[must_use]
    fn cos(self) -> Self;
}

#[cfg(feature = "f16")]
impl MathRotation for f16 {
    fn sin(self) -> Self {
        core::intrinsics::sin(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cos(self)
    }
}

impl MathRotation for f32 {
    fn sin(self) -> Self {
        core::intrinsics::sin(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cos(self)
    }
}

impl MathRotation for f64 {
    fn sin(self) -> Self {
        core::intrinsics::sin(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cos(self)
    }
}

#[cfg(feature = "f128")]
impl MathRotation for f128 {
    fn sin(self) -> Self {
        core::intrinsics::sin(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cos(self)
    }
}
