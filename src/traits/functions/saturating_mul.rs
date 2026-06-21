/// Saturating multiplication
pub const trait SaturatingMul<Rhs = Self> {
    /// The resulting type after applying saturating multiplication
    type Output;

    #[must_use]
    /// Multiply two values, saturating at the numeric bounds instead of overflowing
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}

const impl SaturatingMul for i8 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for i16 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for i32 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for i64 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for i128 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for isize {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for u8 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for u16 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for u32 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for u64 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for u128 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

const impl SaturatingMul for usize {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}
