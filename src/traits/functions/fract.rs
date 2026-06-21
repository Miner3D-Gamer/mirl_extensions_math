/// Get the decimals behind the dot
pub const trait Fract {
    /// Get the decimals behind the dot
    #[must_use]
    fn fract(self) -> Self;
}
impl Fract for f16 {
    fn fract(self) -> Self {
        self - self.trunc()
    }
}
impl Fract for f32 {
    fn fract(self) -> Self {
        core::f32::math::fract(self)
    }
}

impl Fract for f64 {
    fn fract(self) -> Self {
        core::f64::math::fract(self)
    }
}
impl Fract for f128 {
    fn fract(self) -> Self {
        self - self.trunc()
    }
}
