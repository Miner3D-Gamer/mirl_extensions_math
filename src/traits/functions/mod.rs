mod wrapping_ops;
pub use wrapping_ops::*;

mod round;
pub use round::*;
mod sqrt;
pub use sqrt::*;
mod abs;
pub use abs::*;
mod next_power_of_two;
pub use next_power_of_two::*;
mod next_power_of_two_as_exponent;
pub use next_power_of_two_as_exponent::*;

mod i_give_up_with_the_naming;
pub use i_give_up_with_the_naming::*;
mod log2;
pub use log2::*;
mod log10;
pub use log10::*;
mod exp2;
pub use exp2::*;
mod fract;
pub use fract::*;
mod saturating_abs;
pub use saturating_abs::*;
mod saturating_add;
pub use saturating_add::*;
mod saturating_div;
pub use saturating_div::*;
mod saturating_mul;
pub use saturating_mul::*;
mod saturating_negative;
pub use saturating_negative::*;
mod saturating_sub;
pub use saturating_sub::*;

mod previous_power_of_two;
pub use previous_power_of_two::*;

mod is_power_of_two;
pub use is_power_of_two::*;

mod math_rotation;
pub use math_rotation::*;
mod mul_add;
pub use mul_add::*;
mod ceil;
pub use ceil::*;
mod floor;
pub use floor::*;
mod clamp;
pub use clamp::*;
mod natural_exp;
pub use natural_exp::*;
mod trunc;
pub use trunc::*;
mod natural_log;
pub use natural_log::*;
mod hypot;
pub use hypot::*;
mod add_sign;
pub use add_sign::*;

mod map_to_sign;
pub use map_to_sign::*;
mod uniform_previous_next;
pub use uniform_previous_next::*;
mod next_up_down;
pub use next_up_down::*;
mod interpolate_colors;
pub use interpolate_colors::*;
mod interpolate_values;
pub use interpolate_values::*;
mod angular_conversion;
pub use angular_conversion::*;
mod normalize_vector;
pub use normalize_vector::*;

// TODO: Put this into its own file
/// Interpolate between 0 and 1 by using self as the interpolator
pub const trait Interpolate0To1AsInterpolator {
    /// Interpolate between 0 and 1 by using self as the interpolator
    ///
    /// Steepness is how quickly the result goes from 0 to 1
    ///
    /// Offset is when the result goes from 0 to 1
    ///
    /// At x 1, the y will be 0.5 if the offset is 0
    /// If the offset is 2 the y will be 2 at x 0.5
    #[must_use]
    fn interpolate_smooth_0_to_1(self, steepness: Self, offset: Self) -> Self;
}

impl<
    T: InterpolateAsInterpolator<T>
        + mirl_extensions_core::One
        + std::ops::Add<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Neg<Output = T>
        + NaturalExp
        + Clone,
> Interpolate0To1AsInterpolator for T
{
    fn interpolate_smooth_0_to_1(self, steepness: Self, offset: Self) -> Self {
        Self::one()
            / (Self::one() + { ((self / offset) * steepness.clone()).natural_exp() } + (-steepness))
    }
}

// use crate::{U1, U2, U4};

// pub const trait Modular<Rhs = Self> {
//     type Output;
//     fn modular(&self, modulus: Rhs) -> Self::Output;
// }

// macro_rules! impl_modular_unsigned {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0 {
//                         panic!("Division by zero: modulus cannot be zero");
//                     }
//                     self % modulus
//                 }
//             }
//         )*
//     };
// }

// macro_rules! impl_modular_signed {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0 {
//                         panic!("Division by zero: modulus cannot be zero");
//                     }
//                     // Use rem_euclid to ensure positive remainder for negative numbers
//                     self.rem_euclid(modulus.abs())
//                 }
//             }
//         )*
//     };
// }

// macro_rules! impl_modular_float {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0.0 || modulus.is_nan() || self.is_nan() {
//                         panic!("Invalid modular operation: NaN or zero modulus");
//                     }
//                     // Use rem_euclid for consistent positive results
//                     self.rem_euclid(modulus.abs())
//                 }
//             }
//         )*
//     };
// }

// impl_modular_unsigned!(u8, u16, u32, u64, u128, usize, U4);
// impl_modular_signed!(i8, i16, i32, i64, i128, isize);
// impl_modular_float!(f32, f64);
