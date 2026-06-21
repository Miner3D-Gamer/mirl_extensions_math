/// Check if a number is a power of two
pub const trait IsPowerOfTwo {
    /// Returns true if the number is a power of two
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    fn is_power_of_two(self) -> bool;
}

const impl IsPowerOfTwo for u8 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

const impl IsPowerOfTwo for u16 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

const impl IsPowerOfTwo for u32 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

const impl IsPowerOfTwo for u64 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

const impl IsPowerOfTwo for u128 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

const impl IsPowerOfTwo for usize {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

const impl IsPowerOfTwo for i8 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

const impl IsPowerOfTwo for i16 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

const impl IsPowerOfTwo for i32 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

const impl IsPowerOfTwo for i64 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

const impl IsPowerOfTwo for i128 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

const impl IsPowerOfTwo for isize {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl IsPowerOfTwo for f16 {
    #[allow(clippy::float_cmp)]
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f32 {
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f64 {
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f128 {
    #[allow(clippy::float_cmp)]
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}
