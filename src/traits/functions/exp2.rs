/// Calculate 2^x
pub const trait Exp2 {
    ///  Calculate 2^x
    #[must_use]
    fn exp2(self) -> Self;
}

// TODO: Implement Exp for all U and I types

#[cfg(feature = "f16")]
impl Exp2 for f16 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2(self)
    }
}
impl Exp2 for f32 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2(self)
    }
}

impl Exp2 for f64 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2(self)
    }
}
#[cfg(feature = "f128")]
impl Exp2 for f128 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2(self)
    }
}
