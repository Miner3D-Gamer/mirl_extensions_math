//! Math, Geometry, and Expressions
// Const
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![allow(unused_features)]
#![feature(const_try)]
#![feature(const_option_ops)]
#![feature(const_cmp)]
// Core
// #![feature(core_intrinsics)]
// Additions
#![cfg_attr(feature = "f128", feature(f128))]
#![cfg_attr(feature = "f16", feature(f16))]
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(const_destruct)]
#![feature(core_float_math)]
#![feature(core_intrinsics)]
#![feature(associated_type_defaults)]

// #[cfg(not(all(feature = "f128", feature = "f16")))]
// compile_error!("To use this lib you need to enable the `f128` and `f16 features`. \nEverything depends on them.\nTODO: FIX THIS MESS");

mod traits;
pub use traits::*;
