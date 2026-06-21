/// Saturating addition
pub const trait SaturatingAdd<Rhs = Self> {
    /// The resulting type after applying saturating addition
    type Output;

    #[must_use]
    /// Add two values, saturating at the numeric bounds instead of overflowing
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

const impl SaturatingAdd for i8 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for i16 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for i32 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for i64 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for i128 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for isize {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for u8 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for u16 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for u32 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for u64 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for u128 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

const impl SaturatingAdd for usize {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}
