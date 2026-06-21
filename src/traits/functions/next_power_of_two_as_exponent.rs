/// Find the next number that is a power of two and return both the value and the exponent
pub const trait NextPowerOfTwoAsExponent {
    /// Find the next power of two and return (2^x, x)
    #[must_use]
    fn next_power_of_two_as_exponent(self) -> (Self, Self)
    where
        Self: core::marker::Sized;
}

const impl NextPowerOfTwoAsExponent for u8 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for u16 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for u32 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros();
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for u64 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for u128 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for usize {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for i8 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for i16 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for i32 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for i64 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for i128 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

const impl NextPowerOfTwoAsExponent for isize {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl NextPowerOfTwoAsExponent for f16 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoAsExponent for f32 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoAsExponent for f64 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoAsExponent for f128 {
    fn next_power_of_two_as_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}
