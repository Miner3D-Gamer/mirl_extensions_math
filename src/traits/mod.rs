#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]

#[cfg(feature = "bigfloat")]
mod bigfloat;
#[cfg(feature = "bigint")]
mod bigint;

mod consts;
pub use consts::*;

mod functions;
// #[cfg(test)]
// /// A list of tests
// pub mod tests;

pub use functions::*;
// macro_rules! impl_sign {
//     ($($t:ty),*) => {
//         // See those two spaces between the `const` and `Sign`?
//         $(const impl  Sign for $t {
//             fn sign(self) -> Self {
//                 if self > 0 { 1  }
//                 else if self < 0  { -1 }
//                 else { 0  }
//             }
//         })*
//     };
// }
// macro_rules! impl_sign_float {
//     ($($t:ty),*) => {
//         // See those two spaces between the `const` and `Sign`?
//         $(const impl  Sign for $t {
//             fn sign(self) -> Self {
//                 if self > 0.0 { 1.0  }
//                 else if self < 0.0  { -1.0 }
//                 else { 0.0  }
//             }
//         })*
//     };
// }

// impl_sign!(i8, i16, i32, i64, i128, isize);
// impl_sign_float!(f16, f32, f64, f128);

// use core::f32;
// use core::convert::TryFrom;

// pub fn from_u8<T: TryFrom<u8>>(value: u8) -> T {
//     T::try_from(value).ok().expect("constant u8 conversion failed")
// }
