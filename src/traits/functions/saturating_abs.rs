


/// Saturating absolute value
pub const trait SaturatingAbs {
    /// The resulting type after applying saturating absolute value
    type Output;

    #[must_use]
    /// Take the absolute value, saturating at the numeric bounds instead of overflowing
    fn saturating_abs(self) -> Self::Output;
}

const impl SaturatingAbs for i8 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

const impl SaturatingAbs for i16 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

const impl SaturatingAbs for i32 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

const impl SaturatingAbs for i64 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

const impl SaturatingAbs for i128 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

const impl SaturatingAbs for isize {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}