/// Get the previous power of two (rounding down)
pub const trait PrevPowerOfTwo {
    /// Find the previous power of two (the largest power of two less than or equal to self)
    #[must_use]
    fn prev_power_of_two(self) -> Self;
}

const impl PrevPowerOfTwo for u8 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

const impl PrevPowerOfTwo for u16 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

const impl PrevPowerOfTwo for u32 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

const impl PrevPowerOfTwo for u64 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

const impl PrevPowerOfTwo for u128 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

const impl PrevPowerOfTwo for usize {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

const impl PrevPowerOfTwo for i8 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

const impl PrevPowerOfTwo for i16 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

const impl PrevPowerOfTwo for i32 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

const impl PrevPowerOfTwo for i64 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

const impl PrevPowerOfTwo for i128 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

const impl PrevPowerOfTwo for isize {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl PrevPowerOfTwo for f16 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f32 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f64 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f128 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}
