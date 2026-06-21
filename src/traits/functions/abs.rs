/// Take the abs of the given value
pub const trait Abs {
    #[must_use]
    /// Take the sqrt of the given value and return the result
    fn abs(self) -> Self;
}
const impl Abs for f16 {
    #[cfg(not(any(target_env = "gnu", target_env = "msvc")))]
    fn abs(self) -> Self {
        Self::from_bits(self.to_bits() & !(1 << 15))
    }
    #[cfg(any(target_env = "gnu", target_env = "msvc"))]
    fn abs(self) -> Self {
        core::intrinsics::fabs(self)
    }
}
const impl Abs for f32 {
    #[cfg(not(any(target_env = "gnu", target_env = "msvc")))]
    fn abs(self) -> Self {
        Self::from_bits(self.to_bits() & !(1 << 31))
    }
    #[cfg(any(target_env = "gnu", target_env = "msvc"))]
    fn abs(self) -> Self {
        core::intrinsics::fabs(self)
    }
}
const impl Abs for f64 {
    #[cfg(not(any(target_env = "gnu", target_env = "msvc")))]
    fn abs(self) -> Self {
        Self::from_bits(self.to_bits() & !(1 << 63))
    }
    #[cfg(any(target_env = "gnu", target_env = "msvc"))]
    fn abs(self) -> Self {
        core::intrinsics::fabs(self)
    }
}
const impl Abs for f128 {
    #[cfg(not(any(target_env = "gnu", target_env = "msvc")))]
    fn abs(self) -> Self {
        Self::from_bits(self.to_bits() & !(1 << 127))
    }
    #[cfg(any(target_env = "gnu", target_env = "msvc"))]
    fn abs(self) -> Self {
        core::intrinsics::fabs(self)
    }
}
const impl Abs for i8 {
    fn abs(self) -> Self {
        self.abs()
    }
}
const impl Abs for i16 {
    fn abs(self) -> Self {
        self.abs()
    }
}
const impl Abs for i32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
const impl Abs for i64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
const impl Abs for i128 {
    fn abs(self) -> Self {
        self.abs()
    }
}
const impl Abs for isize {
    fn abs(self) -> Self {
        self.abs()
    }
}
