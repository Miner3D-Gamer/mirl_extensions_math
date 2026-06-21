/// Saturating subtraction
pub const trait SaturatingSub<Rhs = Self> {
    /// The resulting type after applying saturating subtraction
    type Output;

    #[must_use]
    /// Subtract two values, saturating at the numeric bounds instead of overflowing
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}
const impl SaturatingSub for i8 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for i16 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for i32 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for i64 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for i128 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for isize {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for u8 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for u16 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for u32 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for u64 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for u128 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

const impl SaturatingSub for usize {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}
