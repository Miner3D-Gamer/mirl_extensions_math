/// Round a value to its nearest neighbor
pub const trait Round {
    #[must_use]
    /// Round the current value to its nearest neighbor and return the result
    fn round(self) -> Self;
}
const impl Round for f16 {
    fn round(self) -> Self {
        core::intrinsics::roundf16(self)
    }
}
const impl Round for f32 {
    fn round(self) -> Self {
        core::f32::math::round(self)
    }
}
const impl Round for f64 {
    fn round(self) -> Self {
        core::f64::math::round(self)
    }
}
const impl Round for f128 {
    fn round(self) -> Self {
        core::intrinsics::roundf128(self)
    }
}
const impl Round for u8 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for u16 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for u64 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for u32 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for u128 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for usize {
    fn round(self) -> Self {
        self
    }
}

const impl Round for i8 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for i16 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for i64 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for i32 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for i128 {
    fn round(self) -> Self {
        self
    }
}
const impl Round for isize {
    fn round(self) -> Self {
        self
    }
}
