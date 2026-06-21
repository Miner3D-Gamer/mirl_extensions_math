/// Calculate 2^x
pub const trait Exp2 {
    ///  Calculate 2^x
    #[must_use]
    fn exp2(self) -> Self;
}

// TODO: Implement Exp for all U and I types

impl Exp2 for f16 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f16(self)
    }
}
impl Exp2 for f32 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f32(self)
    }
}

impl Exp2 for f64 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f64(self)
    }
}
impl Exp2 for f128 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f128(self)
    }
}
