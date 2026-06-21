/// Find the next number that is divisible by two
pub const trait NextPowerOfTwo {
    /// Find the next number that is divisible by two
    #[must_use]
    fn next_power_of_two(self) -> Self;
}

const impl NextPowerOfTwo for u8 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

const impl NextPowerOfTwo for u16 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

const impl NextPowerOfTwo for u32 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

const impl NextPowerOfTwo for u64 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

const impl NextPowerOfTwo for u128 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

const impl NextPowerOfTwo for usize {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

const impl NextPowerOfTwo for i8 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

const impl NextPowerOfTwo for i16 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

const impl NextPowerOfTwo for i32 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

const impl NextPowerOfTwo for i64 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

const impl NextPowerOfTwo for i128 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

const impl NextPowerOfTwo for isize {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl NextPowerOfTwo for f16 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}
impl NextPowerOfTwo for f32 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}

impl NextPowerOfTwo for f64 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}
impl NextPowerOfTwo for f128 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}
