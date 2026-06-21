/// Saturating negation
pub const trait SaturatingNeg {
    /// The resulting type after applying saturating negation
    type Output;

    #[must_use]
    /// Negate the value, saturating at the numeric bounds instead of overflowing
    fn saturating_neg(self) -> Self::Output;
}

const impl SaturatingNeg for i8 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

const impl SaturatingNeg for i16 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

const impl SaturatingNeg for i32 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

const impl SaturatingNeg for i64 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

const impl SaturatingNeg for i128 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

const impl SaturatingNeg for isize {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}
