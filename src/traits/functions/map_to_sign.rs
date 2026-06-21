use crate::WrapOps;
use mirl_extensions_core::ConstOne;

/// Trait for mapping between signed and unsigned integer types
pub const trait MapToSign {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;

    /// Map from patch unsigned back to signed by flipping the sign bit
    fn map_non_sign_to_sign(self) -> Self::Signed;
}

/// Trait for mapping between signed and unsigned integer types
pub const trait MapToUnSign {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;

    /// Map from patch signed to unsigned by flipping the sign bit
    fn map_sign_to_non_sign(self) -> Self::Unsigned;
}

/// Macro to implement `SignMapping` for integer type pairs
macro_rules! impl_sign_mapping {
    ($signed:ty, $unsigned:ty) => {
        const impl MapToUnSign for $signed {
            type Signed = $signed;
            type Unsigned = $unsigned;

            #[allow(clippy::cast_sign_loss)]
            #[allow(trivial_numeric_casts)]
            fn map_sign_to_non_sign(self) -> Self::Unsigned {
                (self as Self::Unsigned).wrapping_add(
                    <$unsigned>::MAX / (<$unsigned>::ONE + <$unsigned>::ONE) + <$unsigned>::ONE,
                )
            }
        }
        const impl MapToSign for $unsigned {
            type Signed = $signed;
            type Unsigned = $unsigned;

            #[allow(clippy::cast_possible_wrap)]
            #[allow(trivial_numeric_casts)]
            fn map_non_sign_to_sign(self) -> Self::Signed {
                self.wrapping_sub(
                    <$unsigned>::MAX / (<$unsigned>::ONE + <$unsigned>::ONE) + <$unsigned>::ONE,
                ) as Self::Signed
            }
        }
    };
}

// Implement for all standard integer pairs
impl_sign_mapping!(i8, u8);
impl_sign_mapping!(i16, u16);
impl_sign_mapping!(i32, u32);
impl_sign_mapping!(i64, u64);
impl_sign_mapping!(i128, u128);
impl_sign_mapping!(isize, usize);
impl_sign_mapping!(f16, f16);
impl_sign_mapping!(f32, f32);
impl_sign_mapping!(f64, f64);
impl_sign_mapping!(f128, f128);
