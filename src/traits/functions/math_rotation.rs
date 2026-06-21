/// Trait providing trigonometric functions
pub const trait MathRotation {
    /// Computes the sine of self (in radians)
    #[must_use]
    fn sin(self) -> Self;

    /// Computes the cosine of self (in radians)
    #[must_use]
    fn cos(self) -> Self;
}

impl MathRotation for f16 {
    fn sin(self) -> Self {
        core::intrinsics::sinf16(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf16(self)
    }
}

impl MathRotation for f32 {
    fn sin(self) -> Self {
        core::intrinsics::sinf32(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf32(self)
    }
}

impl MathRotation for f64 {
    fn sin(self) -> Self {
        core::intrinsics::sinf64(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf64(self)
    }
}

impl MathRotation for f128 {
    fn sin(self) -> Self {
        core::intrinsics::sinf128(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf128(self)
    }
}
