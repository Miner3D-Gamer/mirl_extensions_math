#[cfg(feature = "test")]
#[test]
fn generate() {
    //TODO:
    // - Add more traits to be implemented
    // - Take inspiration of the amount of functions implemented by values under fixed::types::* and create traits for all of them

    // `cargo test --package mirl_extensions_math --features fixed,test -- --no-capture > log.txt`

    use mirl_extensions_core::_mirl_generate_supported_crate_fixed;

    let impls: [mirl_extensions_core::MirlGenerateTraitImplFunction; _] = [
        |_name: &str, _signed: bool, _int_bits, _frac_bits| {
            Some((
                "Abs".to_string(),
                format!(
                    "fn abs(self)->Self{{{}}}",
                    if _signed { "self.abs()" } else { "self" }
                ),
            ))
        },
        |_name: &str, _signed: bool, _int_bits, frac_bits| {
            Some((
                "Ceil".to_string(),
                format!(
                    "fn ceil(self)->Self{{{}}}",
                    if frac_bits > 0 { "self.ceil()" } else { "self" }
                ),
            ))
        },
        |_name: &str, _signed: bool, _int_bits, frac_bits| {
            Some((
                "Floor".to_string(),
                format!(
                    "fn floor(self)->Self{{{}}}",
                    if frac_bits > 0 {
                        "self.floor()"
                    } else {
                        "self"
                    }
                ),
            ))
        },
        |_name: &str, _signed: bool, _int_bits, frac_bits| {
            Some((
                "Round".to_string(),
                format!(
                    "fn round(self)->Self{{{}}}",
                    if frac_bits > 0 {
                        "self.round()"
                    } else {
                        "self"
                    }
                ),
            ))
        },
        |_name: &str, _signed: bool, _int_bits, _frac_bits| {
            Some((
                "Sqrt".to_string(),
                "fn sqrt(self)->Self{self.sqrt()}".to_string(),
            ))
        },
        |_name: &str, _signed: bool, _int_bits, _frac_bits| {
            Some((
                "SaturatingAdd".to_string(),
                "type Output=Self;fn saturating_add(self,other:Self)->Self{self.saturating_add(other)}"
                    .to_string(),
            ))
        },
        |_name: &str, _signed: bool, _int_bits, _frac_bits| {
            Some((
                "SaturatingSub".to_string(),
                "type Output=Self;fn saturating_sub(self,other:Self)->Self{self.saturating_sub(other)}"
                    .to_string(),
            ))
        },
        |_name: &str, _signed: bool, _int_bits, _frac_bits| {
            Some((
                "SaturatingMul".to_string(),
                "type Output=Self;fn saturating_mul(self,other:Self)->Self{self.saturating_mul(other)}"
                    .to_string(),
            ))
        },
        |_name: &str, _signed: bool, _int_bits, _frac_bits| {
            Some((
                "SaturatingDiv".to_string(),
                "type Output=Self;fn saturating_div(self,other:Self)->Self{self.saturating_div(other)}"
                    .to_string(),
            ))
        },
    ];

    _mirl_generate_supported_crate_fixed(&impls);
}

use crate::*;
impl Abs for fixed::types::I8F0 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I8F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::I8F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::I8F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::I8F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I8F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I8F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I8F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I8F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I7F1 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I7F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I7F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I7F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I7F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I7F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I7F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I7F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I7F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I6F2 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I6F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I6F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I6F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I6F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I6F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I6F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I6F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I6F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I5F3 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I5F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I5F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I5F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I5F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I5F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I5F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I5F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I5F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I4F4 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I4F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I4F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I4F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I4F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I4F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I4F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I4F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I4F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I3F5 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I3F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I3F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I3F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I3F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I3F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I3F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I3F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I3F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I2F6 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I2F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I2F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I2F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I2F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I2F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I2F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I2F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I2F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I1F7 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I1F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I1F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I1F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I1F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I1F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I1F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I1F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I1F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I0F8 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I0F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I0F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I0F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I0F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I0F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I0F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I0F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I0F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I16F0 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I16F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::I16F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::I16F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::I16F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I16F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I16F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I16F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I16F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I15F1 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I15F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I15F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I15F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I15F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I15F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I15F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I15F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I15F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I14F2 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I14F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I14F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I14F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I14F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I14F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I14F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I14F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I14F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I13F3 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I13F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I13F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I13F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I13F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I13F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I13F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I13F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I13F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I12F4 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I12F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I12F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I12F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I12F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I12F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I12F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I12F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I12F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I11F5 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I11F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I11F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I11F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I11F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I11F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I11F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I11F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I11F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I10F6 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I10F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I10F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I10F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I10F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I10F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I10F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I10F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I10F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I9F7 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I9F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I9F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I9F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I9F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I9F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I9F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I9F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I9F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I8F8 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I8F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I8F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I8F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I8F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I8F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I8F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I8F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I8F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I7F9 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I7F9 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I7F9 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I7F9 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I7F9 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I7F9 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I7F9 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I7F9 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I7F9 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I6F10 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I6F10 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I6F10 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I6F10 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I6F10 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I6F10 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I6F10 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I6F10 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I6F10 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I5F11 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I5F11 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I5F11 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I5F11 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I5F11 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I5F11 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I5F11 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I5F11 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I5F11 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I4F12 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I4F12 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I4F12 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I4F12 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I4F12 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I4F12 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I4F12 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I4F12 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I4F12 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I3F13 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I3F13 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I3F13 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I3F13 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I3F13 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I3F13 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I3F13 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I3F13 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I3F13 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I2F14 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I2F14 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I2F14 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I2F14 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I2F14 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I2F14 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I2F14 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I2F14 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I2F14 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I1F15 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I1F15 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I1F15 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I1F15 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I1F15 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I1F15 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I1F15 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I1F15 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I1F15 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I0F16 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I0F16 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I0F16 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I0F16 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I0F16 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I0F16 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I0F16 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I0F16 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I0F16 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I32F0 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I32F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::I32F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::I32F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::I32F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I32F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I32F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I32F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I32F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I31F1 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I31F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I31F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I31F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I31F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I31F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I31F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I31F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I31F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I30F2 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I30F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I30F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I30F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I30F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I30F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I30F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I30F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I30F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I29F3 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I29F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I29F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I29F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I29F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I29F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I29F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I29F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I29F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I28F4 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I28F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I28F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I28F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I28F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I28F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I28F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I28F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I28F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I27F5 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I27F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I27F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I27F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I27F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I27F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I27F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I27F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I27F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I26F6 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I26F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I26F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I26F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I26F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I26F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I26F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I26F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I26F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I25F7 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I25F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I25F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I25F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I25F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I25F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I25F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I25F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I25F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I24F8 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I24F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I24F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I24F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I24F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I24F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I24F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I24F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I24F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I23F9 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I23F9 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I23F9 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I23F9 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I23F9 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I23F9 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I23F9 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I23F9 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I23F9 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I22F10 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I22F10 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I22F10 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I22F10 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I22F10 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I22F10 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I22F10 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I22F10 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I22F10 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I21F11 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I21F11 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I21F11 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I21F11 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I21F11 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I21F11 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I21F11 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I21F11 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I21F11 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I20F12 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I20F12 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I20F12 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I20F12 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I20F12 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I20F12 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I20F12 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I20F12 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I20F12 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I19F13 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I19F13 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I19F13 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I19F13 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I19F13 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I19F13 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I19F13 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I19F13 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I19F13 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I18F14 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I18F14 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I18F14 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I18F14 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I18F14 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I18F14 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I18F14 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I18F14 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I18F14 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I17F15 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I17F15 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I17F15 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I17F15 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I17F15 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I17F15 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I17F15 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I17F15 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I17F15 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I16F16 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I16F16 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I16F16 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I16F16 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I16F16 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I16F16 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I16F16 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I16F16 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I16F16 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I15F17 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I15F17 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I15F17 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I15F17 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I15F17 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I15F17 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I15F17 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I15F17 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I15F17 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I14F18 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I14F18 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I14F18 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I14F18 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I14F18 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I14F18 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I14F18 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I14F18 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I14F18 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I13F19 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I13F19 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I13F19 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I13F19 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I13F19 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I13F19 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I13F19 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I13F19 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I13F19 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I12F20 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I12F20 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I12F20 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I12F20 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I12F20 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I12F20 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I12F20 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I12F20 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I12F20 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I11F21 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I11F21 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I11F21 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I11F21 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I11F21 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I11F21 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I11F21 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I11F21 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I11F21 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I10F22 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I10F22 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I10F22 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I10F22 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I10F22 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I10F22 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I10F22 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I10F22 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I10F22 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I9F23 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I9F23 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I9F23 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I9F23 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I9F23 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I9F23 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I9F23 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I9F23 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I9F23 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I8F24 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I8F24 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I8F24 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I8F24 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I8F24 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I8F24 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I8F24 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I8F24 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I8F24 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I7F25 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I7F25 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I7F25 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I7F25 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I7F25 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I7F25 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I7F25 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I7F25 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I7F25 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I6F26 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I6F26 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I6F26 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I6F26 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I6F26 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I6F26 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I6F26 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I6F26 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I6F26 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I5F27 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I5F27 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I5F27 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I5F27 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I5F27 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I5F27 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I5F27 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I5F27 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I5F27 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I4F28 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I4F28 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I4F28 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I4F28 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I4F28 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I4F28 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I4F28 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I4F28 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I4F28 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I3F29 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I3F29 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I3F29 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I3F29 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I3F29 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I3F29 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I3F29 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I3F29 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I3F29 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I2F30 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I2F30 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I2F30 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I2F30 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I2F30 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I2F30 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I2F30 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I2F30 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I2F30 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I1F31 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I1F31 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I1F31 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I1F31 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I1F31 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I1F31 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I1F31 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I1F31 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I1F31 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I0F32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I0F32 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I0F32 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I0F32 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I0F32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I0F32 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I0F32 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I0F32 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I0F32 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I64F0 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I64F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::I64F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::I64F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::I64F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I64F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I64F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I64F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I64F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I63F1 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I63F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I63F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I63F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I63F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I63F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I63F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I63F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I63F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I62F2 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I62F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I62F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I62F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I62F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I62F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I62F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I62F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I62F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I61F3 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I61F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I61F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I61F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I61F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I61F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I61F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I61F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I61F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I60F4 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I60F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I60F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I60F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I60F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I60F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I60F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I60F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I60F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I59F5 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I59F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I59F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I59F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I59F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I59F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I59F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I59F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I59F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I58F6 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I58F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I58F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I58F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I58F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I58F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I58F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I58F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I58F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I57F7 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I57F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I57F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I57F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I57F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I57F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I57F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I57F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I57F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I56F8 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I56F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I56F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I56F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I56F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I56F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I56F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I56F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I56F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I55F9 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I55F9 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I55F9 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I55F9 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I55F9 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I55F9 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I55F9 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I55F9 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I55F9 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I54F10 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I54F10 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I54F10 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I54F10 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I54F10 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I54F10 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I54F10 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I54F10 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I54F10 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I53F11 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I53F11 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I53F11 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I53F11 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I53F11 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I53F11 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I53F11 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I53F11 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I53F11 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I52F12 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I52F12 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I52F12 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I52F12 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I52F12 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I52F12 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I52F12 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I52F12 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I52F12 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I51F13 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I51F13 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I51F13 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I51F13 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I51F13 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I51F13 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I51F13 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I51F13 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I51F13 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I50F14 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I50F14 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I50F14 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I50F14 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I50F14 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I50F14 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I50F14 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I50F14 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I50F14 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I49F15 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I49F15 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I49F15 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I49F15 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I49F15 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I49F15 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I49F15 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I49F15 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I49F15 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I48F16 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I48F16 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I48F16 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I48F16 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I48F16 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I48F16 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I48F16 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I48F16 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I48F16 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I47F17 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I47F17 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I47F17 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I47F17 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I47F17 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I47F17 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I47F17 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I47F17 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I47F17 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I46F18 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I46F18 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I46F18 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I46F18 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I46F18 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I46F18 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I46F18 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I46F18 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I46F18 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I45F19 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I45F19 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I45F19 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I45F19 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I45F19 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I45F19 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I45F19 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I45F19 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I45F19 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I44F20 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I44F20 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I44F20 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I44F20 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I44F20 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I44F20 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I44F20 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I44F20 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I44F20 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I43F21 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I43F21 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I43F21 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I43F21 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I43F21 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I43F21 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I43F21 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I43F21 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I43F21 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I42F22 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I42F22 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I42F22 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I42F22 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I42F22 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I42F22 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I42F22 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I42F22 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I42F22 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I41F23 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I41F23 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I41F23 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I41F23 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I41F23 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I41F23 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I41F23 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I41F23 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I41F23 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I40F24 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I40F24 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I40F24 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I40F24 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I40F24 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I40F24 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I40F24 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I40F24 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I40F24 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I39F25 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I39F25 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I39F25 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I39F25 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I39F25 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I39F25 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I39F25 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I39F25 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I39F25 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I38F26 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I38F26 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I38F26 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I38F26 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I38F26 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I38F26 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I38F26 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I38F26 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I38F26 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I37F27 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I37F27 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I37F27 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I37F27 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I37F27 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I37F27 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I37F27 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I37F27 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I37F27 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I36F28 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I36F28 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I36F28 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I36F28 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I36F28 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I36F28 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I36F28 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I36F28 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I36F28 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I35F29 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I35F29 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I35F29 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I35F29 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I35F29 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I35F29 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I35F29 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I35F29 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I35F29 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I34F30 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I34F30 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I34F30 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I34F30 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I34F30 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I34F30 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I34F30 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I34F30 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I34F30 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I33F31 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I33F31 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I33F31 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I33F31 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I33F31 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I33F31 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I33F31 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I33F31 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I33F31 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I32F32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I32F32 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I32F32 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I32F32 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I32F32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I32F32 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I32F32 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I32F32 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I32F32 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I31F33 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I31F33 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I31F33 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I31F33 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I31F33 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I31F33 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I31F33 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I31F33 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I31F33 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I30F34 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I30F34 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I30F34 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I30F34 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I30F34 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I30F34 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I30F34 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I30F34 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I30F34 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I29F35 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I29F35 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I29F35 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I29F35 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I29F35 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I29F35 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I29F35 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I29F35 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I29F35 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I28F36 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I28F36 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I28F36 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I28F36 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I28F36 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I28F36 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I28F36 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I28F36 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I28F36 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I27F37 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I27F37 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I27F37 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I27F37 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I27F37 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I27F37 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I27F37 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I27F37 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I27F37 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I26F38 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I26F38 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I26F38 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I26F38 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I26F38 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I26F38 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I26F38 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I26F38 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I26F38 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I25F39 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I25F39 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I25F39 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I25F39 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I25F39 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I25F39 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I25F39 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I25F39 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I25F39 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I24F40 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I24F40 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I24F40 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I24F40 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I24F40 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I24F40 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I24F40 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I24F40 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I24F40 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I23F41 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I23F41 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I23F41 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I23F41 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I23F41 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I23F41 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I23F41 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I23F41 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I23F41 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I22F42 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I22F42 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I22F42 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I22F42 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I22F42 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I22F42 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I22F42 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I22F42 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I22F42 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I21F43 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I21F43 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I21F43 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I21F43 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I21F43 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I21F43 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I21F43 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I21F43 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I21F43 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I20F44 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I20F44 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I20F44 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I20F44 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I20F44 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I20F44 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I20F44 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I20F44 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I20F44 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I19F45 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I19F45 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I19F45 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I19F45 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I19F45 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I19F45 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I19F45 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I19F45 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I19F45 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I18F46 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I18F46 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I18F46 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I18F46 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I18F46 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I18F46 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I18F46 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I18F46 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I18F46 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I17F47 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I17F47 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I17F47 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I17F47 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I17F47 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I17F47 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I17F47 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I17F47 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I17F47 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I16F48 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I16F48 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I16F48 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I16F48 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I16F48 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I16F48 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I16F48 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I16F48 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I16F48 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I15F49 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I15F49 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I15F49 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I15F49 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I15F49 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I15F49 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I15F49 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I15F49 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I15F49 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I14F50 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I14F50 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I14F50 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I14F50 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I14F50 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I14F50 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I14F50 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I14F50 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I14F50 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I13F51 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I13F51 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I13F51 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I13F51 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I13F51 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I13F51 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I13F51 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I13F51 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I13F51 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I12F52 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I12F52 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I12F52 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I12F52 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I12F52 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I12F52 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I12F52 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I12F52 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I12F52 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I11F53 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I11F53 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I11F53 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I11F53 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I11F53 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I11F53 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I11F53 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I11F53 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I11F53 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I10F54 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I10F54 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I10F54 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I10F54 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I10F54 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I10F54 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I10F54 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I10F54 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I10F54 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I9F55 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I9F55 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I9F55 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I9F55 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I9F55 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I9F55 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I9F55 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I9F55 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I9F55 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I8F56 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I8F56 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I8F56 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I8F56 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I8F56 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I8F56 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I8F56 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I8F56 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I8F56 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I7F57 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I7F57 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I7F57 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I7F57 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I7F57 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I7F57 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I7F57 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I7F57 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I7F57 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I6F58 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I6F58 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I6F58 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I6F58 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I6F58 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I6F58 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I6F58 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I6F58 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I6F58 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I5F59 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I5F59 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I5F59 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I5F59 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I5F59 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I5F59 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I5F59 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I5F59 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I5F59 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I4F60 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I4F60 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I4F60 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I4F60 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I4F60 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I4F60 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I4F60 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I4F60 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I4F60 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I3F61 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I3F61 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I3F61 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I3F61 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I3F61 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I3F61 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I3F61 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I3F61 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I3F61 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I2F62 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I2F62 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I2F62 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I2F62 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I2F62 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I2F62 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I2F62 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I2F62 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I2F62 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I1F63 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I1F63 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I1F63 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I1F63 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I1F63 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I1F63 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I1F63 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I1F63 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I1F63 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I0F64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I0F64 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I0F64 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I0F64 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I0F64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I0F64 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I0F64 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I0F64 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I0F64 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I128F0 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I128F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::I128F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::I128F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::I128F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I128F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I128F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I128F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I128F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I127F1 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I127F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I127F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I127F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I127F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I127F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I127F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I127F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I127F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I126F2 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I126F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I126F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I126F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I126F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I126F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I126F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I126F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I126F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I125F3 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I125F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I125F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I125F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I125F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I125F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I125F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I125F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I125F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I124F4 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I124F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I124F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I124F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I124F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I124F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I124F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I124F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I124F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I123F5 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I123F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I123F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I123F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I123F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I123F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I123F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I123F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I123F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I122F6 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I122F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I122F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I122F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I122F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I122F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I122F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I122F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I122F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I121F7 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I121F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I121F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I121F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I121F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I121F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I121F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I121F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I121F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I120F8 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I120F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I120F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I120F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I120F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I120F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I120F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I120F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I120F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I119F9 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I119F9 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I119F9 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I119F9 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I119F9 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I119F9 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I119F9 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I119F9 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I119F9 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I118F10 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I118F10 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I118F10 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I118F10 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I118F10 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I118F10 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I118F10 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I118F10 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I118F10 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I117F11 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I117F11 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I117F11 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I117F11 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I117F11 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I117F11 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I117F11 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I117F11 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I117F11 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I116F12 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I116F12 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I116F12 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I116F12 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I116F12 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I116F12 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I116F12 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I116F12 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I116F12 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I115F13 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I115F13 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I115F13 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I115F13 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I115F13 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I115F13 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I115F13 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I115F13 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I115F13 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I114F14 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I114F14 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I114F14 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I114F14 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I114F14 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I114F14 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I114F14 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I114F14 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I114F14 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I113F15 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I113F15 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I113F15 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I113F15 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I113F15 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I113F15 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I113F15 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I113F15 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I113F15 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I112F16 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I112F16 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I112F16 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I112F16 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I112F16 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I112F16 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I112F16 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I112F16 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I112F16 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I111F17 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I111F17 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I111F17 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I111F17 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I111F17 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I111F17 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I111F17 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I111F17 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I111F17 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I110F18 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I110F18 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I110F18 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I110F18 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I110F18 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I110F18 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I110F18 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I110F18 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I110F18 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I109F19 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I109F19 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I109F19 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I109F19 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I109F19 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I109F19 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I109F19 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I109F19 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I109F19 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I108F20 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I108F20 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I108F20 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I108F20 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I108F20 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I108F20 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I108F20 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I108F20 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I108F20 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I107F21 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I107F21 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I107F21 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I107F21 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I107F21 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I107F21 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I107F21 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I107F21 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I107F21 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I106F22 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I106F22 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I106F22 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I106F22 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I106F22 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I106F22 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I106F22 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I106F22 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I106F22 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I105F23 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I105F23 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I105F23 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I105F23 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I105F23 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I105F23 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I105F23 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I105F23 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I105F23 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I104F24 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I104F24 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I104F24 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I104F24 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I104F24 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I104F24 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I104F24 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I104F24 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I104F24 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I103F25 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I103F25 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I103F25 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I103F25 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I103F25 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I103F25 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I103F25 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I103F25 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I103F25 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I102F26 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I102F26 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I102F26 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I102F26 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I102F26 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I102F26 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I102F26 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I102F26 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I102F26 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I101F27 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I101F27 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I101F27 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I101F27 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I101F27 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I101F27 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I101F27 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I101F27 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I101F27 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I100F28 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I100F28 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I100F28 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I100F28 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I100F28 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I100F28 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I100F28 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I100F28 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I100F28 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I99F29 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I99F29 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I99F29 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I99F29 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I99F29 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I99F29 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I99F29 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I99F29 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I99F29 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I98F30 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I98F30 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I98F30 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I98F30 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I98F30 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I98F30 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I98F30 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I98F30 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I98F30 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I97F31 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I97F31 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I97F31 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I97F31 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I97F31 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I97F31 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I97F31 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I97F31 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I97F31 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I96F32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I96F32 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I96F32 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I96F32 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I96F32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I96F32 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I96F32 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I96F32 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I96F32 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I95F33 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I95F33 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I95F33 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I95F33 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I95F33 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I95F33 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I95F33 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I95F33 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I95F33 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I94F34 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I94F34 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I94F34 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I94F34 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I94F34 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I94F34 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I94F34 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I94F34 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I94F34 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I93F35 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I93F35 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I93F35 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I93F35 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I93F35 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I93F35 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I93F35 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I93F35 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I93F35 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I92F36 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I92F36 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I92F36 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I92F36 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I92F36 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I92F36 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I92F36 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I92F36 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I92F36 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I91F37 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I91F37 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I91F37 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I91F37 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I91F37 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I91F37 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I91F37 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I91F37 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I91F37 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I90F38 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I90F38 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I90F38 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I90F38 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I90F38 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I90F38 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I90F38 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I90F38 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I90F38 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I89F39 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I89F39 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I89F39 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I89F39 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I89F39 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I89F39 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I89F39 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I89F39 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I89F39 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I88F40 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I88F40 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I88F40 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I88F40 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I88F40 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I88F40 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I88F40 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I88F40 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I88F40 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I87F41 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I87F41 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I87F41 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I87F41 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I87F41 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I87F41 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I87F41 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I87F41 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I87F41 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I86F42 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I86F42 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I86F42 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I86F42 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I86F42 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I86F42 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I86F42 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I86F42 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I86F42 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I85F43 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I85F43 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I85F43 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I85F43 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I85F43 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I85F43 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I85F43 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I85F43 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I85F43 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I84F44 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I84F44 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I84F44 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I84F44 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I84F44 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I84F44 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I84F44 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I84F44 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I84F44 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I83F45 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I83F45 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I83F45 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I83F45 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I83F45 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I83F45 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I83F45 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I83F45 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I83F45 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I82F46 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I82F46 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I82F46 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I82F46 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I82F46 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I82F46 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I82F46 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I82F46 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I82F46 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I81F47 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I81F47 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I81F47 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I81F47 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I81F47 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I81F47 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I81F47 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I81F47 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I81F47 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I80F48 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I80F48 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I80F48 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I80F48 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I80F48 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I80F48 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I80F48 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I80F48 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I80F48 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I79F49 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I79F49 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I79F49 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I79F49 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I79F49 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I79F49 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I79F49 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I79F49 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I79F49 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I78F50 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I78F50 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I78F50 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I78F50 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I78F50 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I78F50 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I78F50 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I78F50 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I78F50 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I77F51 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I77F51 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I77F51 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I77F51 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I77F51 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I77F51 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I77F51 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I77F51 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I77F51 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I76F52 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I76F52 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I76F52 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I76F52 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I76F52 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I76F52 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I76F52 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I76F52 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I76F52 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I75F53 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I75F53 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I75F53 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I75F53 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I75F53 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I75F53 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I75F53 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I75F53 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I75F53 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I74F54 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I74F54 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I74F54 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I74F54 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I74F54 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I74F54 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I74F54 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I74F54 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I74F54 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I73F55 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I73F55 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I73F55 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I73F55 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I73F55 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I73F55 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I73F55 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I73F55 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I73F55 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I72F56 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I72F56 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I72F56 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I72F56 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I72F56 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I72F56 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I72F56 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I72F56 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I72F56 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I71F57 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I71F57 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I71F57 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I71F57 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I71F57 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I71F57 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I71F57 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I71F57 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I71F57 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I70F58 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I70F58 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I70F58 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I70F58 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I70F58 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I70F58 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I70F58 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I70F58 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I70F58 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I69F59 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I69F59 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I69F59 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I69F59 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I69F59 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I69F59 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I69F59 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I69F59 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I69F59 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I68F60 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I68F60 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I68F60 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I68F60 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I68F60 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I68F60 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I68F60 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I68F60 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I68F60 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I67F61 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I67F61 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I67F61 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I67F61 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I67F61 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I67F61 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I67F61 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I67F61 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I67F61 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I66F62 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I66F62 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I66F62 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I66F62 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I66F62 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I66F62 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I66F62 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I66F62 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I66F62 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I65F63 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I65F63 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I65F63 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I65F63 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I65F63 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I65F63 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I65F63 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I65F63 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I65F63 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I64F64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I64F64 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I64F64 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I64F64 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I64F64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I64F64 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I64F64 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I64F64 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I64F64 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I63F65 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I63F65 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I63F65 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I63F65 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I63F65 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I63F65 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I63F65 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I63F65 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I63F65 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I62F66 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I62F66 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I62F66 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I62F66 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I62F66 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I62F66 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I62F66 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I62F66 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I62F66 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I61F67 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I61F67 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I61F67 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I61F67 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I61F67 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I61F67 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I61F67 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I61F67 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I61F67 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I60F68 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I60F68 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I60F68 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I60F68 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I60F68 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I60F68 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I60F68 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I60F68 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I60F68 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I59F69 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I59F69 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I59F69 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I59F69 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I59F69 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I59F69 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I59F69 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I59F69 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I59F69 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I58F70 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I58F70 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I58F70 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I58F70 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I58F70 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I58F70 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I58F70 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I58F70 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I58F70 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I57F71 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I57F71 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I57F71 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I57F71 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I57F71 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I57F71 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I57F71 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I57F71 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I57F71 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I56F72 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I56F72 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I56F72 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I56F72 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I56F72 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I56F72 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I56F72 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I56F72 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I56F72 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I55F73 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I55F73 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I55F73 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I55F73 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I55F73 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I55F73 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I55F73 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I55F73 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I55F73 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I54F74 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I54F74 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I54F74 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I54F74 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I54F74 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I54F74 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I54F74 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I54F74 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I54F74 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I53F75 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I53F75 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I53F75 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I53F75 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I53F75 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I53F75 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I53F75 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I53F75 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I53F75 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I52F76 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I52F76 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I52F76 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I52F76 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I52F76 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I52F76 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I52F76 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I52F76 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I52F76 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I51F77 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I51F77 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I51F77 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I51F77 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I51F77 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I51F77 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I51F77 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I51F77 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I51F77 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I50F78 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I50F78 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I50F78 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I50F78 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I50F78 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I50F78 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I50F78 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I50F78 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I50F78 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I49F79 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I49F79 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I49F79 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I49F79 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I49F79 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I49F79 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I49F79 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I49F79 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I49F79 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I48F80 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I48F80 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I48F80 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I48F80 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I48F80 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I48F80 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I48F80 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I48F80 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I48F80 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I47F81 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I47F81 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I47F81 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I47F81 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I47F81 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I47F81 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I47F81 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I47F81 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I47F81 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I46F82 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I46F82 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I46F82 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I46F82 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I46F82 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I46F82 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I46F82 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I46F82 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I46F82 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I45F83 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I45F83 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I45F83 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I45F83 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I45F83 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I45F83 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I45F83 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I45F83 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I45F83 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I44F84 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I44F84 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I44F84 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I44F84 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I44F84 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I44F84 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I44F84 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I44F84 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I44F84 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I43F85 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I43F85 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I43F85 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I43F85 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I43F85 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I43F85 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I43F85 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I43F85 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I43F85 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I42F86 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I42F86 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I42F86 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I42F86 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I42F86 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I42F86 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I42F86 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I42F86 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I42F86 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I41F87 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I41F87 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I41F87 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I41F87 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I41F87 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I41F87 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I41F87 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I41F87 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I41F87 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I40F88 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I40F88 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I40F88 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I40F88 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I40F88 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I40F88 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I40F88 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I40F88 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I40F88 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I39F89 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I39F89 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I39F89 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I39F89 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I39F89 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I39F89 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I39F89 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I39F89 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I39F89 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I38F90 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I38F90 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I38F90 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I38F90 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I38F90 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I38F90 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I38F90 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I38F90 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I38F90 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I37F91 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I37F91 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I37F91 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I37F91 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I37F91 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I37F91 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I37F91 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I37F91 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I37F91 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I36F92 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I36F92 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I36F92 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I36F92 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I36F92 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I36F92 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I36F92 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I36F92 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I36F92 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I35F93 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I35F93 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I35F93 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I35F93 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I35F93 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I35F93 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I35F93 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I35F93 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I35F93 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I34F94 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I34F94 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I34F94 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I34F94 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I34F94 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I34F94 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I34F94 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I34F94 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I34F94 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I33F95 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I33F95 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I33F95 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I33F95 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I33F95 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I33F95 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I33F95 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I33F95 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I33F95 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I32F96 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I32F96 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I32F96 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I32F96 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I32F96 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I32F96 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I32F96 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I32F96 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I32F96 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I31F97 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I31F97 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I31F97 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I31F97 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I31F97 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I31F97 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I31F97 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I31F97 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I31F97 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I30F98 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I30F98 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I30F98 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I30F98 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I30F98 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I30F98 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I30F98 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I30F98 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I30F98 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I29F99 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I29F99 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I29F99 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I29F99 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I29F99 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I29F99 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I29F99 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I29F99 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I29F99 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I28F100 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I28F100 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I28F100 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I28F100 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I28F100 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I28F100 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I28F100 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I28F100 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I28F100 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I27F101 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I27F101 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I27F101 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I27F101 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I27F101 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I27F101 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I27F101 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I27F101 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I27F101 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I26F102 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I26F102 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I26F102 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I26F102 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I26F102 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I26F102 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I26F102 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I26F102 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I26F102 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I25F103 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I25F103 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I25F103 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I25F103 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I25F103 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I25F103 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I25F103 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I25F103 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I25F103 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I24F104 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I24F104 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I24F104 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I24F104 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I24F104 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I24F104 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I24F104 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I24F104 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I24F104 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I23F105 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I23F105 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I23F105 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I23F105 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I23F105 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I23F105 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I23F105 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I23F105 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I23F105 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I22F106 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I22F106 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I22F106 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I22F106 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I22F106 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I22F106 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I22F106 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I22F106 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I22F106 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I21F107 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I21F107 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I21F107 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I21F107 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I21F107 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I21F107 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I21F107 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I21F107 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I21F107 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I20F108 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I20F108 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I20F108 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I20F108 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I20F108 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I20F108 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I20F108 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I20F108 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I20F108 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I19F109 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I19F109 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I19F109 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I19F109 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I19F109 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I19F109 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I19F109 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I19F109 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I19F109 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I18F110 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I18F110 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I18F110 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I18F110 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I18F110 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I18F110 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I18F110 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I18F110 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I18F110 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I17F111 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I17F111 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I17F111 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I17F111 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I17F111 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I17F111 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I17F111 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I17F111 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I17F111 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I16F112 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I16F112 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I16F112 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I16F112 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I16F112 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I16F112 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I16F112 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I16F112 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I16F112 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I15F113 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I15F113 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I15F113 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I15F113 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I15F113 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I15F113 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I15F113 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I15F113 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I15F113 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I14F114 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I14F114 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I14F114 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I14F114 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I14F114 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I14F114 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I14F114 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I14F114 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I14F114 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I13F115 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I13F115 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I13F115 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I13F115 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I13F115 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I13F115 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I13F115 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I13F115 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I13F115 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I12F116 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I12F116 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I12F116 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I12F116 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I12F116 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I12F116 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I12F116 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I12F116 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I12F116 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I11F117 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I11F117 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I11F117 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I11F117 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I11F117 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I11F117 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I11F117 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I11F117 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I11F117 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I10F118 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I10F118 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I10F118 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I10F118 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I10F118 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I10F118 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I10F118 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I10F118 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I10F118 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I9F119 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I9F119 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I9F119 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I9F119 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I9F119 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I9F119 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I9F119 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I9F119 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I9F119 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I8F120 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I8F120 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I8F120 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I8F120 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I8F120 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I8F120 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I8F120 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I8F120 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I8F120 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I7F121 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I7F121 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I7F121 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I7F121 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I7F121 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I7F121 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I7F121 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I7F121 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I7F121 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I6F122 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I6F122 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I6F122 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I6F122 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I6F122 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I6F122 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I6F122 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I6F122 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I6F122 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I5F123 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I5F123 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I5F123 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I5F123 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I5F123 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I5F123 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I5F123 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I5F123 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I5F123 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I4F124 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I4F124 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I4F124 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I4F124 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I4F124 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I4F124 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I4F124 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I4F124 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I4F124 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I3F125 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I3F125 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I3F125 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I3F125 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I3F125 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I3F125 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I3F125 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I3F125 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I3F125 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I2F126 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I2F126 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I2F126 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I2F126 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I2F126 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I2F126 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I2F126 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I2F126 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I2F126 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I1F127 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I1F127 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I1F127 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I1F127 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I1F127 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I1F127 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I1F127 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I1F127 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I1F127 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::I0F128 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Ceil for fixed::types::I0F128 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::I0F128 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::I0F128 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::I0F128 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::I0F128 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::I0F128 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::I0F128 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::I0F128 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U8F0 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U8F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::U8F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::U8F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::U8F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U8F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U8F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U8F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U8F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U7F1 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U7F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U7F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U7F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U7F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U7F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U7F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U7F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U7F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U6F2 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U6F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U6F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U6F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U6F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U6F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U6F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U6F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U6F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U5F3 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U5F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U5F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U5F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U5F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U5F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U5F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U5F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U5F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U4F4 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U4F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U4F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U4F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U4F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U4F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U4F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U4F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U4F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U3F5 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U3F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U3F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U3F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U3F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U3F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U3F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U3F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U3F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U2F6 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U2F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U2F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U2F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U2F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U2F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U2F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U2F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U2F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U1F7 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U1F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U1F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U1F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U1F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U1F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U1F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U1F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U1F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U0F8 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U0F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U0F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U0F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U0F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U0F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U0F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U0F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U0F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U16F0 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U16F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::U16F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::U16F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::U16F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U16F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U16F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U16F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U16F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U15F1 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U15F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U15F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U15F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U15F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U15F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U15F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U15F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U15F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U14F2 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U14F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U14F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U14F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U14F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U14F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U14F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U14F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U14F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U13F3 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U13F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U13F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U13F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U13F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U13F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U13F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U13F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U13F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U12F4 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U12F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U12F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U12F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U12F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U12F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U12F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U12F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U12F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U11F5 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U11F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U11F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U11F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U11F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U11F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U11F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U11F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U11F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U10F6 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U10F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U10F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U10F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U10F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U10F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U10F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U10F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U10F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U9F7 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U9F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U9F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U9F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U9F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U9F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U9F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U9F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U9F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U8F8 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U8F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U8F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U8F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U8F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U8F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U8F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U8F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U8F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U7F9 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U7F9 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U7F9 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U7F9 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U7F9 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U7F9 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U7F9 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U7F9 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U7F9 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U6F10 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U6F10 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U6F10 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U6F10 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U6F10 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U6F10 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U6F10 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U6F10 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U6F10 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U5F11 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U5F11 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U5F11 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U5F11 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U5F11 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U5F11 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U5F11 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U5F11 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U5F11 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U4F12 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U4F12 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U4F12 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U4F12 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U4F12 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U4F12 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U4F12 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U4F12 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U4F12 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U3F13 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U3F13 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U3F13 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U3F13 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U3F13 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U3F13 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U3F13 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U3F13 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U3F13 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U2F14 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U2F14 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U2F14 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U2F14 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U2F14 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U2F14 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U2F14 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U2F14 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U2F14 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U1F15 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U1F15 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U1F15 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U1F15 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U1F15 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U1F15 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U1F15 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U1F15 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U1F15 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U0F16 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U0F16 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U0F16 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U0F16 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U0F16 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U0F16 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U0F16 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U0F16 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U0F16 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U32F0 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U32F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::U32F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::U32F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::U32F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U32F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U32F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U32F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U32F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U31F1 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U31F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U31F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U31F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U31F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U31F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U31F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U31F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U31F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U30F2 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U30F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U30F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U30F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U30F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U30F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U30F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U30F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U30F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U29F3 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U29F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U29F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U29F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U29F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U29F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U29F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U29F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U29F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U28F4 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U28F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U28F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U28F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U28F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U28F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U28F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U28F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U28F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U27F5 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U27F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U27F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U27F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U27F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U27F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U27F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U27F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U27F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U26F6 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U26F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U26F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U26F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U26F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U26F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U26F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U26F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U26F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U25F7 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U25F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U25F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U25F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U25F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U25F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U25F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U25F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U25F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U24F8 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U24F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U24F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U24F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U24F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U24F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U24F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U24F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U24F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U23F9 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U23F9 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U23F9 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U23F9 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U23F9 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U23F9 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U23F9 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U23F9 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U23F9 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U22F10 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U22F10 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U22F10 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U22F10 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U22F10 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U22F10 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U22F10 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U22F10 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U22F10 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U21F11 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U21F11 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U21F11 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U21F11 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U21F11 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U21F11 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U21F11 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U21F11 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U21F11 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U20F12 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U20F12 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U20F12 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U20F12 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U20F12 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U20F12 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U20F12 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U20F12 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U20F12 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U19F13 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U19F13 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U19F13 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U19F13 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U19F13 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U19F13 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U19F13 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U19F13 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U19F13 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U18F14 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U18F14 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U18F14 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U18F14 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U18F14 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U18F14 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U18F14 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U18F14 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U18F14 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U17F15 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U17F15 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U17F15 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U17F15 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U17F15 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U17F15 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U17F15 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U17F15 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U17F15 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U16F16 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U16F16 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U16F16 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U16F16 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U16F16 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U16F16 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U16F16 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U16F16 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U16F16 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U15F17 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U15F17 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U15F17 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U15F17 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U15F17 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U15F17 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U15F17 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U15F17 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U15F17 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U14F18 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U14F18 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U14F18 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U14F18 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U14F18 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U14F18 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U14F18 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U14F18 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U14F18 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U13F19 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U13F19 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U13F19 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U13F19 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U13F19 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U13F19 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U13F19 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U13F19 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U13F19 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U12F20 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U12F20 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U12F20 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U12F20 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U12F20 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U12F20 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U12F20 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U12F20 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U12F20 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U11F21 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U11F21 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U11F21 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U11F21 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U11F21 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U11F21 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U11F21 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U11F21 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U11F21 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U10F22 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U10F22 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U10F22 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U10F22 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U10F22 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U10F22 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U10F22 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U10F22 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U10F22 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U9F23 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U9F23 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U9F23 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U9F23 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U9F23 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U9F23 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U9F23 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U9F23 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U9F23 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U8F24 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U8F24 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U8F24 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U8F24 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U8F24 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U8F24 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U8F24 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U8F24 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U8F24 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U7F25 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U7F25 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U7F25 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U7F25 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U7F25 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U7F25 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U7F25 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U7F25 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U7F25 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U6F26 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U6F26 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U6F26 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U6F26 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U6F26 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U6F26 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U6F26 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U6F26 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U6F26 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U5F27 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U5F27 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U5F27 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U5F27 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U5F27 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U5F27 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U5F27 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U5F27 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U5F27 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U4F28 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U4F28 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U4F28 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U4F28 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U4F28 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U4F28 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U4F28 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U4F28 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U4F28 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U3F29 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U3F29 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U3F29 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U3F29 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U3F29 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U3F29 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U3F29 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U3F29 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U3F29 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U2F30 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U2F30 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U2F30 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U2F30 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U2F30 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U2F30 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U2F30 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U2F30 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U2F30 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U1F31 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U1F31 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U1F31 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U1F31 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U1F31 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U1F31 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U1F31 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U1F31 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U1F31 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U0F32 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U0F32 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U0F32 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U0F32 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U0F32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U0F32 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U0F32 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U0F32 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U0F32 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U64F0 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U64F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::U64F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::U64F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::U64F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U64F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U64F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U64F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U64F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U63F1 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U63F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U63F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U63F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U63F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U63F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U63F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U63F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U63F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U62F2 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U62F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U62F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U62F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U62F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U62F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U62F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U62F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U62F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U61F3 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U61F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U61F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U61F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U61F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U61F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U61F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U61F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U61F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U60F4 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U60F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U60F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U60F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U60F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U60F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U60F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U60F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U60F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U59F5 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U59F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U59F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U59F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U59F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U59F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U59F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U59F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U59F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U58F6 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U58F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U58F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U58F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U58F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U58F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U58F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U58F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U58F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U57F7 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U57F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U57F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U57F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U57F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U57F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U57F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U57F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U57F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U56F8 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U56F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U56F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U56F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U56F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U56F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U56F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U56F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U56F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U55F9 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U55F9 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U55F9 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U55F9 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U55F9 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U55F9 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U55F9 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U55F9 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U55F9 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U54F10 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U54F10 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U54F10 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U54F10 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U54F10 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U54F10 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U54F10 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U54F10 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U54F10 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U53F11 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U53F11 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U53F11 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U53F11 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U53F11 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U53F11 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U53F11 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U53F11 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U53F11 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U52F12 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U52F12 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U52F12 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U52F12 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U52F12 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U52F12 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U52F12 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U52F12 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U52F12 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U51F13 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U51F13 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U51F13 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U51F13 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U51F13 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U51F13 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U51F13 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U51F13 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U51F13 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U50F14 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U50F14 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U50F14 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U50F14 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U50F14 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U50F14 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U50F14 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U50F14 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U50F14 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U49F15 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U49F15 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U49F15 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U49F15 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U49F15 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U49F15 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U49F15 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U49F15 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U49F15 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U48F16 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U48F16 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U48F16 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U48F16 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U48F16 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U48F16 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U48F16 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U48F16 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U48F16 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U47F17 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U47F17 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U47F17 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U47F17 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U47F17 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U47F17 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U47F17 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U47F17 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U47F17 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U46F18 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U46F18 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U46F18 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U46F18 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U46F18 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U46F18 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U46F18 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U46F18 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U46F18 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U45F19 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U45F19 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U45F19 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U45F19 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U45F19 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U45F19 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U45F19 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U45F19 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U45F19 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U44F20 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U44F20 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U44F20 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U44F20 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U44F20 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U44F20 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U44F20 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U44F20 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U44F20 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U43F21 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U43F21 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U43F21 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U43F21 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U43F21 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U43F21 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U43F21 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U43F21 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U43F21 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U42F22 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U42F22 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U42F22 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U42F22 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U42F22 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U42F22 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U42F22 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U42F22 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U42F22 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U41F23 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U41F23 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U41F23 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U41F23 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U41F23 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U41F23 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U41F23 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U41F23 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U41F23 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U40F24 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U40F24 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U40F24 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U40F24 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U40F24 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U40F24 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U40F24 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U40F24 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U40F24 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U39F25 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U39F25 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U39F25 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U39F25 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U39F25 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U39F25 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U39F25 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U39F25 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U39F25 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U38F26 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U38F26 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U38F26 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U38F26 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U38F26 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U38F26 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U38F26 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U38F26 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U38F26 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U37F27 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U37F27 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U37F27 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U37F27 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U37F27 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U37F27 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U37F27 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U37F27 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U37F27 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U36F28 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U36F28 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U36F28 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U36F28 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U36F28 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U36F28 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U36F28 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U36F28 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U36F28 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U35F29 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U35F29 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U35F29 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U35F29 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U35F29 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U35F29 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U35F29 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U35F29 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U35F29 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U34F30 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U34F30 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U34F30 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U34F30 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U34F30 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U34F30 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U34F30 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U34F30 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U34F30 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U33F31 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U33F31 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U33F31 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U33F31 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U33F31 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U33F31 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U33F31 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U33F31 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U33F31 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U32F32 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U32F32 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U32F32 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U32F32 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U32F32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U32F32 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U32F32 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U32F32 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U32F32 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U31F33 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U31F33 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U31F33 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U31F33 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U31F33 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U31F33 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U31F33 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U31F33 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U31F33 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U30F34 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U30F34 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U30F34 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U30F34 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U30F34 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U30F34 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U30F34 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U30F34 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U30F34 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U29F35 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U29F35 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U29F35 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U29F35 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U29F35 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U29F35 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U29F35 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U29F35 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U29F35 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U28F36 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U28F36 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U28F36 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U28F36 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U28F36 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U28F36 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U28F36 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U28F36 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U28F36 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U27F37 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U27F37 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U27F37 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U27F37 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U27F37 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U27F37 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U27F37 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U27F37 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U27F37 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U26F38 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U26F38 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U26F38 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U26F38 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U26F38 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U26F38 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U26F38 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U26F38 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U26F38 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U25F39 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U25F39 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U25F39 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U25F39 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U25F39 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U25F39 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U25F39 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U25F39 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U25F39 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U24F40 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U24F40 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U24F40 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U24F40 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U24F40 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U24F40 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U24F40 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U24F40 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U24F40 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U23F41 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U23F41 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U23F41 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U23F41 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U23F41 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U23F41 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U23F41 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U23F41 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U23F41 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U22F42 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U22F42 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U22F42 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U22F42 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U22F42 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U22F42 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U22F42 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U22F42 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U22F42 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U21F43 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U21F43 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U21F43 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U21F43 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U21F43 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U21F43 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U21F43 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U21F43 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U21F43 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U20F44 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U20F44 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U20F44 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U20F44 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U20F44 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U20F44 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U20F44 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U20F44 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U20F44 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U19F45 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U19F45 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U19F45 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U19F45 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U19F45 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U19F45 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U19F45 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U19F45 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U19F45 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U18F46 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U18F46 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U18F46 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U18F46 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U18F46 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U18F46 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U18F46 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U18F46 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U18F46 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U17F47 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U17F47 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U17F47 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U17F47 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U17F47 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U17F47 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U17F47 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U17F47 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U17F47 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U16F48 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U16F48 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U16F48 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U16F48 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U16F48 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U16F48 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U16F48 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U16F48 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U16F48 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U15F49 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U15F49 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U15F49 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U15F49 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U15F49 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U15F49 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U15F49 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U15F49 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U15F49 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U14F50 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U14F50 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U14F50 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U14F50 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U14F50 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U14F50 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U14F50 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U14F50 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U14F50 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U13F51 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U13F51 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U13F51 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U13F51 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U13F51 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U13F51 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U13F51 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U13F51 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U13F51 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U12F52 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U12F52 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U12F52 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U12F52 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U12F52 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U12F52 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U12F52 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U12F52 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U12F52 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U11F53 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U11F53 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U11F53 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U11F53 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U11F53 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U11F53 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U11F53 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U11F53 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U11F53 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U10F54 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U10F54 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U10F54 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U10F54 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U10F54 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U10F54 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U10F54 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U10F54 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U10F54 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U9F55 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U9F55 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U9F55 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U9F55 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U9F55 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U9F55 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U9F55 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U9F55 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U9F55 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U8F56 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U8F56 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U8F56 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U8F56 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U8F56 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U8F56 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U8F56 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U8F56 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U8F56 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U7F57 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U7F57 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U7F57 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U7F57 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U7F57 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U7F57 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U7F57 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U7F57 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U7F57 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U6F58 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U6F58 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U6F58 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U6F58 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U6F58 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U6F58 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U6F58 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U6F58 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U6F58 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U5F59 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U5F59 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U5F59 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U5F59 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U5F59 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U5F59 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U5F59 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U5F59 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U5F59 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U4F60 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U4F60 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U4F60 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U4F60 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U4F60 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U4F60 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U4F60 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U4F60 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U4F60 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U3F61 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U3F61 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U3F61 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U3F61 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U3F61 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U3F61 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U3F61 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U3F61 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U3F61 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U2F62 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U2F62 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U2F62 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U2F62 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U2F62 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U2F62 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U2F62 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U2F62 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U2F62 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U1F63 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U1F63 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U1F63 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U1F63 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U1F63 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U1F63 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U1F63 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U1F63 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U1F63 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U0F64 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U0F64 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U0F64 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U0F64 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U0F64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U0F64 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U0F64 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U0F64 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U0F64 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U128F0 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U128F0 {
    fn ceil(self) -> Self {
        self
    }
}
impl Floor for fixed::types::U128F0 {
    fn floor(self) -> Self {
        self
    }
}
impl Round for fixed::types::U128F0 {
    fn round(self) -> Self {
        self
    }
}
impl Sqrt for fixed::types::U128F0 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U128F0 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U128F0 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U128F0 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U128F0 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U127F1 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U127F1 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U127F1 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U127F1 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U127F1 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U127F1 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U127F1 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U127F1 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U127F1 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U126F2 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U126F2 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U126F2 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U126F2 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U126F2 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U126F2 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U126F2 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U126F2 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U126F2 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U125F3 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U125F3 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U125F3 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U125F3 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U125F3 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U125F3 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U125F3 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U125F3 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U125F3 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U124F4 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U124F4 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U124F4 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U124F4 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U124F4 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U124F4 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U124F4 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U124F4 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U124F4 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U123F5 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U123F5 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U123F5 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U123F5 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U123F5 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U123F5 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U123F5 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U123F5 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U123F5 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U122F6 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U122F6 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U122F6 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U122F6 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U122F6 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U122F6 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U122F6 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U122F6 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U122F6 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U121F7 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U121F7 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U121F7 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U121F7 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U121F7 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U121F7 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U121F7 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U121F7 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U121F7 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U120F8 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U120F8 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U120F8 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U120F8 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U120F8 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U120F8 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U120F8 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U120F8 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U120F8 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U119F9 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U119F9 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U119F9 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U119F9 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U119F9 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U119F9 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U119F9 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U119F9 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U119F9 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U118F10 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U118F10 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U118F10 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U118F10 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U118F10 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U118F10 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U118F10 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U118F10 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U118F10 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U117F11 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U117F11 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U117F11 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U117F11 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U117F11 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U117F11 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U117F11 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U117F11 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U117F11 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U116F12 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U116F12 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U116F12 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U116F12 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U116F12 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U116F12 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U116F12 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U116F12 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U116F12 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U115F13 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U115F13 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U115F13 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U115F13 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U115F13 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U115F13 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U115F13 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U115F13 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U115F13 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U114F14 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U114F14 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U114F14 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U114F14 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U114F14 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U114F14 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U114F14 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U114F14 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U114F14 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U113F15 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U113F15 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U113F15 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U113F15 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U113F15 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U113F15 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U113F15 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U113F15 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U113F15 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U112F16 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U112F16 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U112F16 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U112F16 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U112F16 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U112F16 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U112F16 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U112F16 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U112F16 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U111F17 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U111F17 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U111F17 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U111F17 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U111F17 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U111F17 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U111F17 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U111F17 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U111F17 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U110F18 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U110F18 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U110F18 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U110F18 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U110F18 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U110F18 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U110F18 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U110F18 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U110F18 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U109F19 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U109F19 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U109F19 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U109F19 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U109F19 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U109F19 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U109F19 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U109F19 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U109F19 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U108F20 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U108F20 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U108F20 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U108F20 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U108F20 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U108F20 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U108F20 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U108F20 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U108F20 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U107F21 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U107F21 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U107F21 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U107F21 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U107F21 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U107F21 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U107F21 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U107F21 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U107F21 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U106F22 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U106F22 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U106F22 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U106F22 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U106F22 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U106F22 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U106F22 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U106F22 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U106F22 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U105F23 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U105F23 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U105F23 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U105F23 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U105F23 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U105F23 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U105F23 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U105F23 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U105F23 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U104F24 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U104F24 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U104F24 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U104F24 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U104F24 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U104F24 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U104F24 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U104F24 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U104F24 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U103F25 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U103F25 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U103F25 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U103F25 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U103F25 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U103F25 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U103F25 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U103F25 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U103F25 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U102F26 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U102F26 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U102F26 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U102F26 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U102F26 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U102F26 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U102F26 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U102F26 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U102F26 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U101F27 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U101F27 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U101F27 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U101F27 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U101F27 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U101F27 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U101F27 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U101F27 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U101F27 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U100F28 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U100F28 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U100F28 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U100F28 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U100F28 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U100F28 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U100F28 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U100F28 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U100F28 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U99F29 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U99F29 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U99F29 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U99F29 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U99F29 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U99F29 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U99F29 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U99F29 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U99F29 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U98F30 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U98F30 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U98F30 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U98F30 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U98F30 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U98F30 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U98F30 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U98F30 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U98F30 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U97F31 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U97F31 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U97F31 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U97F31 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U97F31 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U97F31 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U97F31 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U97F31 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U97F31 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U96F32 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U96F32 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U96F32 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U96F32 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U96F32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U96F32 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U96F32 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U96F32 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U96F32 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U95F33 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U95F33 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U95F33 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U95F33 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U95F33 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U95F33 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U95F33 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U95F33 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U95F33 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U94F34 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U94F34 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U94F34 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U94F34 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U94F34 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U94F34 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U94F34 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U94F34 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U94F34 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U93F35 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U93F35 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U93F35 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U93F35 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U93F35 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U93F35 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U93F35 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U93F35 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U93F35 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U92F36 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U92F36 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U92F36 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U92F36 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U92F36 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U92F36 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U92F36 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U92F36 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U92F36 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U91F37 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U91F37 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U91F37 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U91F37 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U91F37 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U91F37 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U91F37 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U91F37 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U91F37 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U90F38 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U90F38 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U90F38 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U90F38 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U90F38 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U90F38 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U90F38 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U90F38 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U90F38 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U89F39 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U89F39 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U89F39 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U89F39 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U89F39 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U89F39 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U89F39 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U89F39 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U89F39 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U88F40 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U88F40 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U88F40 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U88F40 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U88F40 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U88F40 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U88F40 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U88F40 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U88F40 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U87F41 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U87F41 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U87F41 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U87F41 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U87F41 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U87F41 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U87F41 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U87F41 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U87F41 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U86F42 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U86F42 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U86F42 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U86F42 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U86F42 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U86F42 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U86F42 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U86F42 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U86F42 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U85F43 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U85F43 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U85F43 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U85F43 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U85F43 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U85F43 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U85F43 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U85F43 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U85F43 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U84F44 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U84F44 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U84F44 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U84F44 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U84F44 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U84F44 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U84F44 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U84F44 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U84F44 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U83F45 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U83F45 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U83F45 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U83F45 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U83F45 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U83F45 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U83F45 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U83F45 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U83F45 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U82F46 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U82F46 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U82F46 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U82F46 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U82F46 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U82F46 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U82F46 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U82F46 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U82F46 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U81F47 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U81F47 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U81F47 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U81F47 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U81F47 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U81F47 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U81F47 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U81F47 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U81F47 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U80F48 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U80F48 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U80F48 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U80F48 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U80F48 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U80F48 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U80F48 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U80F48 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U80F48 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U79F49 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U79F49 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U79F49 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U79F49 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U79F49 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U79F49 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U79F49 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U79F49 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U79F49 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U78F50 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U78F50 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U78F50 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U78F50 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U78F50 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U78F50 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U78F50 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U78F50 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U78F50 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U77F51 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U77F51 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U77F51 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U77F51 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U77F51 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U77F51 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U77F51 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U77F51 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U77F51 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U76F52 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U76F52 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U76F52 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U76F52 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U76F52 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U76F52 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U76F52 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U76F52 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U76F52 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U75F53 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U75F53 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U75F53 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U75F53 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U75F53 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U75F53 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U75F53 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U75F53 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U75F53 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U74F54 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U74F54 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U74F54 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U74F54 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U74F54 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U74F54 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U74F54 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U74F54 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U74F54 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U73F55 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U73F55 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U73F55 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U73F55 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U73F55 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U73F55 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U73F55 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U73F55 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U73F55 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U72F56 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U72F56 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U72F56 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U72F56 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U72F56 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U72F56 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U72F56 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U72F56 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U72F56 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U71F57 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U71F57 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U71F57 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U71F57 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U71F57 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U71F57 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U71F57 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U71F57 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U71F57 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U70F58 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U70F58 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U70F58 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U70F58 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U70F58 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U70F58 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U70F58 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U70F58 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U70F58 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U69F59 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U69F59 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U69F59 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U69F59 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U69F59 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U69F59 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U69F59 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U69F59 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U69F59 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U68F60 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U68F60 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U68F60 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U68F60 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U68F60 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U68F60 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U68F60 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U68F60 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U68F60 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U67F61 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U67F61 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U67F61 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U67F61 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U67F61 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U67F61 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U67F61 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U67F61 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U67F61 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U66F62 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U66F62 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U66F62 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U66F62 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U66F62 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U66F62 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U66F62 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U66F62 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U66F62 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U65F63 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U65F63 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U65F63 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U65F63 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U65F63 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U65F63 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U65F63 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U65F63 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U65F63 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U64F64 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U64F64 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U64F64 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U64F64 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U64F64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U64F64 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U64F64 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U64F64 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U64F64 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U63F65 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U63F65 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U63F65 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U63F65 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U63F65 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U63F65 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U63F65 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U63F65 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U63F65 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U62F66 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U62F66 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U62F66 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U62F66 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U62F66 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U62F66 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U62F66 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U62F66 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U62F66 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U61F67 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U61F67 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U61F67 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U61F67 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U61F67 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U61F67 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U61F67 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U61F67 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U61F67 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U60F68 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U60F68 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U60F68 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U60F68 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U60F68 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U60F68 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U60F68 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U60F68 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U60F68 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U59F69 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U59F69 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U59F69 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U59F69 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U59F69 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U59F69 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U59F69 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U59F69 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U59F69 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U58F70 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U58F70 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U58F70 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U58F70 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U58F70 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U58F70 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U58F70 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U58F70 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U58F70 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U57F71 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U57F71 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U57F71 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U57F71 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U57F71 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U57F71 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U57F71 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U57F71 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U57F71 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U56F72 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U56F72 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U56F72 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U56F72 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U56F72 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U56F72 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U56F72 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U56F72 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U56F72 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U55F73 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U55F73 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U55F73 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U55F73 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U55F73 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U55F73 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U55F73 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U55F73 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U55F73 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U54F74 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U54F74 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U54F74 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U54F74 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U54F74 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U54F74 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U54F74 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U54F74 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U54F74 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U53F75 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U53F75 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U53F75 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U53F75 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U53F75 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U53F75 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U53F75 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U53F75 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U53F75 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U52F76 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U52F76 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U52F76 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U52F76 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U52F76 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U52F76 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U52F76 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U52F76 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U52F76 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U51F77 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U51F77 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U51F77 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U51F77 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U51F77 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U51F77 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U51F77 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U51F77 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U51F77 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U50F78 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U50F78 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U50F78 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U50F78 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U50F78 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U50F78 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U50F78 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U50F78 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U50F78 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U49F79 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U49F79 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U49F79 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U49F79 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U49F79 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U49F79 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U49F79 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U49F79 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U49F79 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U48F80 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U48F80 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U48F80 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U48F80 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U48F80 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U48F80 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U48F80 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U48F80 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U48F80 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U47F81 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U47F81 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U47F81 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U47F81 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U47F81 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U47F81 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U47F81 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U47F81 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U47F81 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U46F82 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U46F82 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U46F82 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U46F82 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U46F82 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U46F82 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U46F82 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U46F82 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U46F82 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U45F83 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U45F83 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U45F83 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U45F83 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U45F83 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U45F83 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U45F83 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U45F83 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U45F83 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U44F84 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U44F84 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U44F84 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U44F84 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U44F84 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U44F84 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U44F84 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U44F84 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U44F84 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U43F85 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U43F85 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U43F85 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U43F85 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U43F85 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U43F85 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U43F85 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U43F85 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U43F85 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U42F86 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U42F86 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U42F86 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U42F86 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U42F86 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U42F86 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U42F86 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U42F86 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U42F86 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U41F87 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U41F87 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U41F87 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U41F87 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U41F87 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U41F87 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U41F87 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U41F87 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U41F87 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U40F88 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U40F88 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U40F88 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U40F88 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U40F88 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U40F88 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U40F88 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U40F88 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U40F88 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U39F89 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U39F89 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U39F89 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U39F89 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U39F89 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U39F89 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U39F89 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U39F89 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U39F89 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U38F90 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U38F90 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U38F90 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U38F90 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U38F90 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U38F90 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U38F90 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U38F90 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U38F90 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U37F91 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U37F91 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U37F91 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U37F91 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U37F91 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U37F91 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U37F91 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U37F91 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U37F91 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U36F92 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U36F92 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U36F92 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U36F92 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U36F92 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U36F92 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U36F92 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U36F92 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U36F92 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U35F93 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U35F93 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U35F93 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U35F93 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U35F93 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U35F93 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U35F93 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U35F93 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U35F93 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U34F94 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U34F94 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U34F94 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U34F94 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U34F94 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U34F94 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U34F94 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U34F94 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U34F94 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U33F95 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U33F95 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U33F95 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U33F95 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U33F95 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U33F95 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U33F95 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U33F95 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U33F95 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U32F96 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U32F96 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U32F96 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U32F96 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U32F96 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U32F96 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U32F96 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U32F96 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U32F96 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U31F97 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U31F97 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U31F97 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U31F97 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U31F97 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U31F97 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U31F97 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U31F97 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U31F97 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U30F98 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U30F98 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U30F98 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U30F98 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U30F98 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U30F98 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U30F98 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U30F98 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U30F98 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U29F99 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U29F99 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U29F99 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U29F99 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U29F99 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U29F99 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U29F99 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U29F99 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U29F99 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U28F100 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U28F100 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U28F100 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U28F100 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U28F100 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U28F100 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U28F100 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U28F100 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U28F100 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U27F101 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U27F101 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U27F101 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U27F101 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U27F101 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U27F101 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U27F101 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U27F101 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U27F101 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U26F102 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U26F102 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U26F102 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U26F102 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U26F102 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U26F102 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U26F102 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U26F102 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U26F102 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U25F103 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U25F103 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U25F103 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U25F103 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U25F103 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U25F103 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U25F103 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U25F103 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U25F103 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U24F104 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U24F104 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U24F104 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U24F104 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U24F104 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U24F104 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U24F104 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U24F104 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U24F104 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U23F105 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U23F105 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U23F105 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U23F105 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U23F105 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U23F105 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U23F105 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U23F105 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U23F105 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U22F106 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U22F106 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U22F106 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U22F106 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U22F106 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U22F106 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U22F106 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U22F106 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U22F106 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U21F107 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U21F107 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U21F107 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U21F107 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U21F107 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U21F107 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U21F107 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U21F107 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U21F107 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U20F108 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U20F108 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U20F108 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U20F108 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U20F108 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U20F108 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U20F108 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U20F108 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U20F108 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U19F109 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U19F109 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U19F109 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U19F109 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U19F109 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U19F109 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U19F109 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U19F109 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U19F109 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U18F110 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U18F110 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U18F110 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U18F110 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U18F110 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U18F110 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U18F110 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U18F110 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U18F110 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U17F111 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U17F111 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U17F111 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U17F111 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U17F111 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U17F111 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U17F111 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U17F111 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U17F111 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U16F112 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U16F112 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U16F112 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U16F112 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U16F112 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U16F112 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U16F112 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U16F112 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U16F112 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U15F113 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U15F113 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U15F113 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U15F113 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U15F113 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U15F113 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U15F113 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U15F113 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U15F113 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U14F114 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U14F114 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U14F114 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U14F114 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U14F114 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U14F114 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U14F114 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U14F114 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U14F114 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U13F115 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U13F115 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U13F115 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U13F115 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U13F115 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U13F115 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U13F115 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U13F115 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U13F115 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U12F116 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U12F116 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U12F116 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U12F116 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U12F116 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U12F116 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U12F116 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U12F116 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U12F116 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U11F117 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U11F117 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U11F117 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U11F117 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U11F117 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U11F117 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U11F117 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U11F117 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U11F117 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U10F118 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U10F118 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U10F118 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U10F118 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U10F118 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U10F118 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U10F118 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U10F118 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U10F118 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U9F119 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U9F119 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U9F119 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U9F119 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U9F119 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U9F119 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U9F119 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U9F119 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U9F119 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U8F120 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U8F120 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U8F120 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U8F120 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U8F120 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U8F120 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U8F120 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U8F120 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U8F120 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U7F121 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U7F121 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U7F121 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U7F121 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U7F121 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U7F121 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U7F121 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U7F121 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U7F121 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U6F122 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U6F122 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U6F122 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U6F122 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U6F122 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U6F122 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U6F122 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U6F122 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U6F122 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U5F123 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U5F123 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U5F123 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U5F123 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U5F123 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U5F123 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U5F123 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U5F123 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U5F123 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U4F124 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U4F124 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U4F124 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U4F124 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U4F124 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U4F124 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U4F124 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U4F124 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U4F124 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U3F125 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U3F125 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U3F125 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U3F125 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U3F125 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U3F125 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U3F125 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U3F125 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U3F125 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U2F126 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U2F126 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U2F126 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U2F126 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U2F126 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U2F126 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U2F126 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U2F126 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U2F126 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U1F127 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U1F127 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U1F127 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U1F127 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U1F127 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U1F127 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U1F127 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U1F127 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U1F127 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
impl Abs for fixed::types::U0F128 {
    fn abs(self) -> Self {
        self
    }
}
impl Ceil for fixed::types::U0F128 {
    fn ceil(self) -> Self {
        self.ceil()
    }
}
impl Floor for fixed::types::U0F128 {
    fn floor(self) -> Self {
        self.floor()
    }
}
impl Round for fixed::types::U0F128 {
    fn round(self) -> Self {
        self.round()
    }
}
impl Sqrt for fixed::types::U0F128 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SaturatingAdd for fixed::types::U0F128 {
    type Output = Self;
    fn saturating_add(self, other: Self) -> Self {
        self.saturating_add(other)
    }
}
impl SaturatingSub for fixed::types::U0F128 {
    type Output = Self;
    fn saturating_sub(self, other: Self) -> Self {
        self.saturating_sub(other)
    }
}
impl SaturatingMul for fixed::types::U0F128 {
    type Output = Self;
    fn saturating_mul(self, other: Self) -> Self {
        self.saturating_mul(other)
    }
}
impl SaturatingDiv for fixed::types::U0F128 {
    type Output = Self;
    fn saturating_div(self, other: Self) -> Self {
        self.saturating_div(other)
    }
}
