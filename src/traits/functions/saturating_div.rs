/// Saturating division
pub const trait SaturatingDiv<Rhs = Self> {
    /// The resulting type after applying saturating division
    type Output;

    #[must_use]
    /// Divtract two values, saturating at the numeric bounds instead of overflowing
    fn saturating_div(self, rhs: Rhs) -> Self::Output;
}
const impl SaturatingDiv for i8 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for i16 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for i32 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for i64 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for i128 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for isize {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for u8 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for u16 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for u32 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for u64 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for u128 {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}

const impl SaturatingDiv for usize {
    type Output = Self;
    fn saturating_div(self, rhs: Self) -> Self::Output {
        self.saturating_div(rhs)
    }
}
