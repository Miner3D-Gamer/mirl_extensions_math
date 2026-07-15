/// Find the exponent x where 2^x is the next power of two
pub const trait NextPowerOfTwoExponent {
    /// Find the exponent x where 2^x is the next power of two
    #[must_use]
    fn next_power_of_two_exponent(self) -> Self;
}

const impl NextPowerOfTwoExponent for u8 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for u16 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for u32 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros()
    }
}

const impl NextPowerOfTwoExponent for u64 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for u128 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for usize {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for i8 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for i16 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for i32 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for i64 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for i128 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

const impl NextPowerOfTwoExponent for isize {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl NextPowerOfTwoExponent for f16 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f32 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f64 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f128 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}
