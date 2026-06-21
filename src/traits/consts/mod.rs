#![allow(clippy::approx_constant)]

use mirl_core::impl_trait;

/// The value π for self
pub const trait ConstPi {
    /// The closest approximation of π for self
    const PI: Self;
}

const impl ConstPi for f128 {
    const PI: Self = 3.141_592_653_589_793_238_462_643_383_279_502_884_197_169_399_375_105_820_974_944_592_307_816_406_286_208_998_628_034_825_342_117_067_982_148_086_513_282_306_647_093_844_609_550_582_231_725_359_408_128_481;
}

impl_trait!(ConstPi {const PI: Self = f128::PI as Self;} for  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,f16,f32,f64 );

/// The full-circle constant τ = 2π for `Self`.
pub const trait ConstTau {
    /// The full-circle constant τ = 2π for `Self`.
    const TAU: Self;
}

const impl<T: ConstPi + const core::ops::Add<Output = T>> ConstTau for T {
    const TAU: Self = Self::PI + Self::PI;
}

/// π/2 for `Self`.
pub const trait ConstFracPi2 {
    /// π/2 for `Self`.
    const FRAC_PI_2: Self;
}

const impl<T: ConstPi + const core::ops::Div<Output = T> + ConstNumbers128> ConstFracPi2 for T {
    const FRAC_PI_2: Self = Self::PI / T::CONST_2;
}

/// π/3 for `Self`.
pub const trait ConstFracPi3 {
    /// π/3 for `Self`.
    const FRAC_PI_3: Self;
}

const impl<T: ConstPi + const core::ops::Div<Output = T> + ConstNumbers128> ConstFracPi3 for T {
    const FRAC_PI_3: Self = Self::PI / T::CONST_3;
}

/// π/4 for `Self`.
pub const trait ConstFracPi4 {
    /// π/4 for `Self`.
    const FRAC_PI_4: Self;
}

const impl<T: ConstPi + const core::ops::Div<Output = T> + ConstNumbers128> ConstFracPi4 for T {
    const FRAC_PI_4: Self = Self::PI / T::CONST_4;
}

/// π/6 for `Self`.
pub const trait ConstFracPi6 {
    /// π/6 for `Self`.
    const FRAC_PI_6: Self;
}

const impl<T: ConstPi + const core::ops::Div<Output = T> + ConstNumbers128> ConstFracPi6 for T {
    const FRAC_PI_6: Self = Self::PI / T::CONST_6;
}

/// π/8 for `Self`.
pub const trait ConstFracPi8 {
    /// π/8 for `Self`.
    const FRAC_PI_8: Self;
}

const impl<T: ConstPi + const core::ops::Div<Output = T> + ConstNumbers128> ConstFracPi8 for T {
    const FRAC_PI_8: Self = Self::PI / Self::CONST_8;
}

/// 1/π for `Self`.
pub const trait ConstFrac1Pi {
    /// 1/π for `Self`.
    const FRAC_1_PI: Self;
}

const impl ConstFrac1Pi for f128 {
    const FRAC_1_PI: Self = 0.318_309_886_183_790_671_537_767_526_745_028_724_068_919_291_480_912_897_495_334_688_117_793_595_268_453_070_180_227_605_532_506_171_912_145_685_453_515_916_316_849_467_786_892_184_724_882_613_572_966_735;
}

impl_trait!(ConstFrac1Pi { const FRAC_1_PI: Self = f128::FRAC_1_PI as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 2/π for `Self`.
pub const trait ConstFrac2Pi {
    /// 2/π for `Self`.
    const FRAC_2_PI: Self;
}

const impl<T: ConstFrac1Pi + const core::ops::Add<Output = T> + Copy> ConstFrac2Pi for T {
    const FRAC_2_PI: Self = Self::FRAC_1_PI + Self::FRAC_1_PI;
}

/// 1/√π for `Self`.
pub const trait ConstFrac1SqrtPi {
    /// 1/√π for `Self`.
    const FRAC_1_SQRT_PI: Self;
}

const impl ConstFrac1SqrtPi for f128 {
    const FRAC_1_SQRT_PI: Self = 0.564_189_583_547_756_286_948_079_451_560_772_585_844_050_629_328_998_856_844_085_721_710_642_468_441_493_414_486_743_660_202_107_363_443_028_347_906_997_685_557_610_799_704_053_132_160_490_796_173_323_183;
}

impl_trait!(ConstFrac1SqrtPi { const FRAC_1_SQRT_PI: Self = f128::FRAC_1_SQRT_PI as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 1/√(2π) for `Self`.
pub const trait ConstFrac1Sqrt2Pi {
    /// 1/√(2π) for `Self`.
    const FRAC_1_SQRT_2PI: Self;
}

const impl ConstFrac1Sqrt2Pi for f128 {
    const FRAC_1_SQRT_2PI: Self = 0.398_942_280_401_432_677_939_946_059_934_381_868_475_858_631_164_934_657_665_925_829_670_657_925_899_301_838_501_252_333_907_306_936_430_302_558_863_635_916_791_231_979_536_054_296_508_827_720_380_217_803;
}

impl_trait!(ConstFrac1Sqrt2Pi { const FRAC_1_SQRT_2PI: Self = f128::FRAC_1_SQRT_2PI as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 2/√π for `Self`.
pub const trait ConstFrac2SqrtPi {
    /// 2/√π for `Self`.
    const FRAC_2_SQRT_PI: Self;
}

const impl<T: ConstFrac1SqrtPi + const core::ops::Add<Output = T> + Copy> ConstFrac2SqrtPi for T {
    const FRAC_2_SQRT_PI: Self = Self::FRAC_1_SQRT_PI + Self::FRAC_1_SQRT_PI;
}

/// √2 for `Self`.
pub const trait ConstSqrt2 {
    /// √2 for `Self`.
    const SQRT_2: Self;
}

const impl ConstSqrt2 for f128 {
    const SQRT_2: Self = 1.414_213_562_373_095_048_801_688_724_209_698_078_569_671_875_376_948_073_176_679_737_990_732_478_462_107_038_850_387_534_327_641_572_735_013_846_230_912_297_024_924_836_055_850_737_212_644_121_497_099_936;
}

impl_trait!(ConstSqrt2 { const SQRT_2: Self = f128::SQRT_2 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 1/√2 for `Self`.
pub const trait ConstFrac1Sqrt2 {
    /// 1/√2 for `Self`.
    const FRAC_1_SQRT_2: Self;
}

const impl ConstFrac1Sqrt2 for f128 {
    // derived: 1/√2 = √2/2
    const FRAC_1_SQRT_2: Self = Self::SQRT_2 / 2.0;
}

impl_trait!(ConstFrac1Sqrt2 { const FRAC_1_SQRT_2: Self = f128::FRAC_1_SQRT_2 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// √3 for `Self`.
pub const trait ConstSqrt3 {
    /// √3 for `Self`.
    const SQRT_3: Self;
}

const impl ConstSqrt3 for f128 {
    const SQRT_3: Self = 1.732_050_807_568_877_293_527_446_341_505_872_366_942_805_253_810_380_628_055_006_811_842_539_275_453_904_092_885_952_734_459_534_823_697_552_271_428_031_459_564_690_450_929_735_892_046_697_906_929_736_761;
}

impl_trait!(ConstSqrt3 { const SQRT_3: Self = f128::SQRT_3 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 1/√3 for `Self`.
pub const trait ConstFrac1Sqrt3 {
    /// 1/√3 for `Self`.
    const FRAC_1_SQRT_3: Self;
}

const impl ConstFrac1Sqrt3 for f128 {
    // derived: 1/√3 = √3/3
    const FRAC_1_SQRT_3: Self = Self::SQRT_3 / 3.0;
}

impl_trait!(ConstFrac1Sqrt3 { const FRAC_1_SQRT_3: Self = f128::FRAC_1_SQRT_3 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// √5 for `Self`.
pub const trait ConstSqrt5 {
    /// √5 for `Self`.
    const SQRT_5: Self;
}

const impl ConstSqrt5 for f128 {
    const SQRT_5: Self = 2.236_067_977_499_789_696_409_173_668_731_276_235_440_618_359_611_525_724_270_897_245_410_520_925_637_804_899_414_414_408_378_782_274_969_508_176_022_283_768_454_473_382_952_960_985_865_757_825_634_441_397;
}

impl_trait!(ConstSqrt5 { const SQRT_5: Self = f128::SQRT_5 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// 1/√5 for `Self`.
pub const trait ConstFrac1Sqrt5 {
    /// 1/√5 for `Self`.
    const FRAC_1_SQRT_5: Self;
}

const impl ConstFrac1Sqrt5 for f128 {
    // derived: 1/√5 = √5/5
    const FRAC_1_SQRT_5: Self = Self::SQRT_5 / 5.0;
}

impl_trait!(ConstFrac1Sqrt5 { const FRAC_1_SQRT_5: Self = f128::FRAC_1_SQRT_5 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// The golden ratio φ = (1 + √5) / 2 for `Self`.
pub const trait ConstGoldenRatio {
    /// The golden ratio φ = (1 + √5) / 2 for `Self`.
    const GOLDEN_RATIO: Self;
}

const impl ConstGoldenRatio for f128 {
    // derived from √5
    const GOLDEN_RATIO: Self = Self::midpoint(1.0, Self::SQRT_5);
}

impl_trait!(ConstGoldenRatio { const GOLDEN_RATIO: Self = f128::GOLDEN_RATIO as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// The Euler–Mascheroni constant γ for `Self`.
pub const trait ConstEulerGamma {
    /// The Euler–Mascheroni constant γ for `Self`.
    const EULER_GAMMA: Self;
}

const impl ConstEulerGamma for f128 {
    const EULER_GAMMA: Self = 0.577_215_664_901_532_860_606_512_090_082_402_431_042_159_335_939_923_598_805_767_234_884_867_726_777_664_670_936_947_063_291_746_749_514_631_447_249_807_082_480_960_504_014_486_542_836_224_173_997_644_923;
}

impl_trait!(ConstEulerGamma { const EULER_GAMMA: Self = f128::EULER_GAMMA as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// Euler's number e for `Self`.
pub const trait ConstE {
    /// Euler's number e for `Self`.
    const E: Self;
}

const impl ConstE for f128 {
    const E: Self = 2.718_281_828_459_045_235_360_287_471_352_662_497_757_247_093_699_959_574_966_967_627_724_076_630_353_547_594_571_382_178_525_166_427_427_466_391_932_003_059_921_817_413_596_629_043_572_900_334_295_260_595;
}

impl_trait!(ConstE { const E: Self = f128::E as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// log₂(e) for `Self`.
pub const trait ConstLog2E {
    /// log₂(e) for `Self`.
    const LOG2_E: Self;
}

const impl ConstLog2E for f128 {
    const LOG2_E: Self = 1.442_695_040_888_963_407_359_924_681_001_892_137_426_645_954_152_985_934_135_449_406_931_109_219_181_185_079_885_526_622_893_506_344_496_997_887_060_953_884_853_517_204_976_940_734_965_879_566_948_604_569;
}

impl_trait!(ConstLog2E { const LOG2_E: Self = f128::LOG2_E as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// log₂(10) for `Self`.
pub const trait ConstLog210 {
    /// log₂(10) for `Self`.
    const LOG2_10: Self;
}

const impl ConstLog210 for f128 {
    const LOG2_10: Self =
        3.321_928_094_887_362_347_870_319_429_489_390_175_864_831_393_024_580_612_054;
}

impl_trait!(ConstLog210 { const LOG2_10: Self = f128::LOG2_10 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// log₁₀(e) for `Self`.
pub const trait ConstLog10E {
    /// log₁₀(e) for `Self`.
    const LOG10_E: Self;
}

const impl ConstLog10E for f128 {
    const LOG10_E: Self = 0.434_294_481_903_251_827_651_128_918_916_605_082_294_397_005_803_666_566_114_453_783_165_864_649_208_870_774_729_224_949_338_431_748_318_706_106_744_766_303_733_641_679_252_815_521_765_691_916_292_720_703;
}

impl_trait!(ConstLog10E { const LOG10_E: Self = f128::LOG10_E as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// log₁₀(2) for `Self`.
///
/// Derived as 1 / log₂(10).
pub const trait ConstLog102 {
    /// log₁₀(2) for `Self`.
    ///
    /// Derived as 1 / log₂(10).
    const LOG10_2: Self;
}

const impl ConstLog102 for f128 {
    // derived: log₁₀(2) = 1 / log₂(10)
    const LOG10_2: Self = 1.0 / Self::LOG2_10;
}

impl_trait!(ConstLog102 { const LOG10_2: Self = f128::LOG10_2 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// ln(2) for `Self`.
pub const trait ConstLn2 {
    /// ln(2) for `Self`.
    const LN_2: Self;
}

const impl ConstLn2 for f128 {
    const LN_2: Self = 0.693_147_180_559_945_309_417_232_121_458_176_568_075_500_134_360_255_254_120_680_009_493_393_621_969_694_715_605_863_326_996_418_687_542_001_481_020_570_685_733_685_520_235_758_130_557_032_670_751_635_075;
}

impl_trait!(ConstLn2 { const LN_2: Self = f128::LN_2 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// ln(10) for `Self`.
///
/// Derived as ln(2) × log₂(10).
pub const trait ConstLn10 {
    /// ln(10) for `Self`.
    ///
    /// Derived as ln(2) × log₂(10).
    const LN_10: Self;
}

const impl ConstLn10 for f128 {
    // derived: ln(10) = ln(2) * log₂(10)
    const LN_10: Self = Self::LN_2 * Self::LOG2_10;
}

impl_trait!(ConstLn10 { const LN_10: Self = f128::LN_10 as Self; }
    for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f16, f32, f64);

/// A single super trait that bundles every mathematical constant trait
pub const trait MathConsts:
    ConstPi
    + ConstTau
    + ConstFracPi2
    + ConstFracPi3
    + ConstFracPi4
    + ConstFracPi6
    + ConstFracPi8
    + ConstFrac1Pi
    + ConstFrac2Pi
    + ConstFrac1SqrtPi
    + ConstFrac1Sqrt2Pi
    + ConstFrac2SqrtPi
    + ConstSqrt2
    + ConstFrac1Sqrt2
    + ConstSqrt3
    + ConstFrac1Sqrt3
    + ConstSqrt5
    + ConstFrac1Sqrt5
    + ConstGoldenRatio
    + ConstEulerGamma
    + ConstE
    + ConstLog2E
    + ConstLog210
    + ConstLog10E
    + ConstLog102
    + ConstLn2
    + ConstLn10
{
}

// Blanket implementation: anything that satisfies all sub-traits gets MathConsts for free.
const impl<
    T: const ConstPi
        + const ConstTau
        + const ConstFracPi2
        + const ConstFracPi3
        + const ConstFracPi4
        + const ConstFracPi6
        + const ConstFracPi8
        + const ConstFrac1Pi
        + const ConstFrac2Pi
        + const ConstFrac1SqrtPi
        + const ConstFrac1Sqrt2Pi
        + const ConstFrac2SqrtPi
        + const ConstSqrt2
        + const ConstFrac1Sqrt2
        + const ConstSqrt3
        + const ConstFrac1Sqrt3
        + const ConstSqrt5
        + const ConstFrac1Sqrt5
        + const ConstGoldenRatio
        + const ConstEulerGamma
        + const ConstE
        + const ConstLog2E
        + const ConstLog210
        + const ConstLog10E
        + const ConstLog102
        + const ConstLn2
        + const ConstLn10,
> MathConsts for T
{
}
/// Convert between degrees and radians
///
/// [`DEGREES_TO_RADIAN_RATIO`](Self::DEGREES_TO_RADIAN_RATIO) π/180 - Multiply with this number to convert from degrees to radians
///
/// [`RADIANS_TO_DEGREES_RATIO`](Self::RADIANS_TO_DEGREES_RATIO) 180/π - Multiply with this number to convert from radians to degrees
pub const trait ConstRotationRatio {
    /// π/180 - Multiply with this number to convert from degrees to radians
    const DEGREES_TO_RADIAN_RATIO: Self;
    /// 180/π - Multiply with this number to convert from radians to degrees
    const RADIANS_TO_DEGREES_RATIO: Self;
}

const impl<T: ConstPi + ConstNumbers256 + const std::ops::Div<Output = T>> ConstRotationRatio
    for T
{
    const DEGREES_TO_RADIAN_RATIO: Self = T::PI / T::CONST_180;
    const RADIANS_TO_DEGREES_RATIO: Self = T::CONST_180 / T::PI;
}

#[allow(missing_docs)]
/// An extended version of the [`ConstOne`](crate::math::ConstOne)/[`ConstZero`](crate::math::ConstZero) traits covering all numbers from 0 to 127
pub const trait ConstNumbers128: SupportsRange128 {
    const CONST_0: Self;
    const CONST_1: Self;
    const CONST_2: Self;
    const CONST_3: Self;
    const CONST_4: Self;
    const CONST_5: Self;
    const CONST_6: Self;
    const CONST_7: Self;
    const CONST_8: Self;
    const CONST_9: Self;
    const CONST_10: Self;
    const CONST_11: Self;
    const CONST_12: Self;
    const CONST_13: Self;
    const CONST_14: Self;
    const CONST_15: Self;
    const CONST_16: Self;
    const CONST_17: Self;
    const CONST_18: Self;
    const CONST_19: Self;
    const CONST_20: Self;
    const CONST_21: Self;
    const CONST_22: Self;
    const CONST_23: Self;
    const CONST_24: Self;
    const CONST_25: Self;
    const CONST_26: Self;
    const CONST_27: Self;
    const CONST_28: Self;
    const CONST_29: Self;
    const CONST_30: Self;
    const CONST_31: Self;
    const CONST_32: Self;
    const CONST_33: Self;
    const CONST_34: Self;
    const CONST_35: Self;
    const CONST_36: Self;
    const CONST_37: Self;
    const CONST_38: Self;
    const CONST_39: Self;
    const CONST_40: Self;
    const CONST_41: Self;
    const CONST_42: Self;
    const CONST_43: Self;
    const CONST_44: Self;
    const CONST_45: Self;
    const CONST_46: Self;
    const CONST_47: Self;
    const CONST_48: Self;
    const CONST_49: Self;
    const CONST_50: Self;
    const CONST_51: Self;
    const CONST_52: Self;
    const CONST_53: Self;
    const CONST_54: Self;
    const CONST_55: Self;
    const CONST_56: Self;
    const CONST_57: Self;
    const CONST_58: Self;
    const CONST_59: Self;
    const CONST_60: Self;
    const CONST_61: Self;
    const CONST_62: Self;
    const CONST_63: Self;
    const CONST_64: Self;
    const CONST_65: Self;
    const CONST_66: Self;
    const CONST_67: Self;
    const CONST_68: Self;
    const CONST_69: Self;
    const CONST_70: Self;
    const CONST_71: Self;
    const CONST_72: Self;
    const CONST_73: Self;
    const CONST_74: Self;
    const CONST_75: Self;
    const CONST_76: Self;
    const CONST_77: Self;
    const CONST_78: Self;
    const CONST_79: Self;
    const CONST_80: Self;
    const CONST_81: Self;
    const CONST_82: Self;
    const CONST_83: Self;
    const CONST_84: Self;
    const CONST_85: Self;
    const CONST_86: Self;
    const CONST_87: Self;
    const CONST_88: Self;
    const CONST_89: Self;
    const CONST_90: Self;
    const CONST_91: Self;
    const CONST_92: Self;
    const CONST_93: Self;
    const CONST_94: Self;
    const CONST_95: Self;
    const CONST_96: Self;
    const CONST_97: Self;
    const CONST_98: Self;
    const CONST_99: Self;
    const CONST_100: Self;
    const CONST_101: Self;
    const CONST_102: Self;
    const CONST_103: Self;
    const CONST_104: Self;
    const CONST_105: Self;
    const CONST_106: Self;
    const CONST_107: Self;
    const CONST_108: Self;
    const CONST_109: Self;
    const CONST_110: Self;
    const CONST_111: Self;
    const CONST_112: Self;
    const CONST_113: Self;
    const CONST_114: Self;
    const CONST_115: Self;
    const CONST_116: Self;
    const CONST_117: Self;
    const CONST_118: Self;
    const CONST_119: Self;
    const CONST_120: Self;
    const CONST_121: Self;
    const CONST_122: Self;
    const CONST_123: Self;
    const CONST_124: Self;
    const CONST_125: Self;
    const CONST_126: Self;
    const CONST_127: Self;
}

const impl<T: ConstOne + ConstZero + SupportsRange128 + const core::ops::Add<Output = T>>
    ConstNumbers128 for T
{
    const CONST_0: Self = T::ZERO;
    const CONST_1: Self = T::ONE;
    const CONST_2: Self = T::CONST_1 + T::CONST_1;
    const CONST_3: Self = T::CONST_2 + T::CONST_1;
    const CONST_4: Self = T::CONST_3 + T::CONST_1;
    const CONST_5: Self = T::CONST_4 + T::CONST_1;
    const CONST_6: Self = T::CONST_5 + T::CONST_1;
    const CONST_7: Self = T::CONST_6 + T::CONST_1;
    const CONST_8: Self = T::CONST_7 + T::CONST_1;
    const CONST_9: Self = T::CONST_8 + T::CONST_1;
    const CONST_10: Self = T::CONST_9 + T::CONST_1;
    const CONST_11: Self = T::CONST_10 + T::CONST_1;
    const CONST_12: Self = T::CONST_11 + T::CONST_1;
    const CONST_13: Self = T::CONST_12 + T::CONST_1;
    const CONST_14: Self = T::CONST_13 + T::CONST_1;
    const CONST_15: Self = T::CONST_14 + T::CONST_1;
    const CONST_16: Self = T::CONST_15 + T::CONST_1;
    const CONST_17: Self = T::CONST_16 + T::CONST_1;
    const CONST_18: Self = T::CONST_17 + T::CONST_1;
    const CONST_19: Self = T::CONST_18 + T::CONST_1;
    const CONST_20: Self = T::CONST_19 + T::CONST_1;
    const CONST_21: Self = T::CONST_20 + T::CONST_1;
    const CONST_22: Self = T::CONST_21 + T::CONST_1;
    const CONST_23: Self = T::CONST_22 + T::CONST_1;
    const CONST_24: Self = T::CONST_23 + T::CONST_1;
    const CONST_25: Self = T::CONST_24 + T::CONST_1;
    const CONST_26: Self = T::CONST_25 + T::CONST_1;
    const CONST_27: Self = T::CONST_26 + T::CONST_1;
    const CONST_28: Self = T::CONST_27 + T::CONST_1;
    const CONST_29: Self = T::CONST_28 + T::CONST_1;
    const CONST_30: Self = T::CONST_29 + T::CONST_1;
    const CONST_31: Self = T::CONST_30 + T::CONST_1;
    const CONST_32: Self = T::CONST_31 + T::CONST_1;
    const CONST_33: Self = T::CONST_32 + T::CONST_1;
    const CONST_34: Self = T::CONST_33 + T::CONST_1;
    const CONST_35: Self = T::CONST_34 + T::CONST_1;
    const CONST_36: Self = T::CONST_35 + T::CONST_1;
    const CONST_37: Self = T::CONST_36 + T::CONST_1;
    const CONST_38: Self = T::CONST_37 + T::CONST_1;
    const CONST_39: Self = T::CONST_38 + T::CONST_1;
    const CONST_40: Self = T::CONST_39 + T::CONST_1;
    const CONST_41: Self = T::CONST_40 + T::CONST_1;
    const CONST_42: Self = T::CONST_41 + T::CONST_1;
    const CONST_43: Self = T::CONST_42 + T::CONST_1;
    const CONST_44: Self = T::CONST_43 + T::CONST_1;
    const CONST_45: Self = T::CONST_44 + T::CONST_1;
    const CONST_46: Self = T::CONST_45 + T::CONST_1;
    const CONST_47: Self = T::CONST_46 + T::CONST_1;
    const CONST_48: Self = T::CONST_47 + T::CONST_1;
    const CONST_49: Self = T::CONST_48 + T::CONST_1;
    const CONST_50: Self = T::CONST_49 + T::CONST_1;
    const CONST_51: Self = T::CONST_50 + T::CONST_1;
    const CONST_52: Self = T::CONST_51 + T::CONST_1;
    const CONST_53: Self = T::CONST_52 + T::CONST_1;
    const CONST_54: Self = T::CONST_53 + T::CONST_1;
    const CONST_55: Self = T::CONST_54 + T::CONST_1;
    const CONST_56: Self = T::CONST_55 + T::CONST_1;
    const CONST_57: Self = T::CONST_56 + T::CONST_1;
    const CONST_58: Self = T::CONST_57 + T::CONST_1;
    const CONST_59: Self = T::CONST_58 + T::CONST_1;
    const CONST_60: Self = T::CONST_59 + T::CONST_1;
    const CONST_61: Self = T::CONST_60 + T::CONST_1;
    const CONST_62: Self = T::CONST_61 + T::CONST_1;
    const CONST_63: Self = T::CONST_62 + T::CONST_1;
    const CONST_64: Self = T::CONST_63 + T::CONST_1;
    const CONST_65: Self = T::CONST_64 + T::CONST_1;
    const CONST_66: Self = T::CONST_65 + T::CONST_1;
    const CONST_67: Self = T::CONST_66 + T::CONST_1;
    const CONST_68: Self = T::CONST_67 + T::CONST_1;
    const CONST_69: Self = T::CONST_68 + T::CONST_1;
    const CONST_70: Self = T::CONST_69 + T::CONST_1;
    const CONST_71: Self = T::CONST_70 + T::CONST_1;
    const CONST_72: Self = T::CONST_71 + T::CONST_1;
    const CONST_73: Self = T::CONST_72 + T::CONST_1;
    const CONST_74: Self = T::CONST_73 + T::CONST_1;
    const CONST_75: Self = T::CONST_74 + T::CONST_1;
    const CONST_76: Self = T::CONST_75 + T::CONST_1;
    const CONST_77: Self = T::CONST_76 + T::CONST_1;
    const CONST_78: Self = T::CONST_77 + T::CONST_1;
    const CONST_79: Self = T::CONST_78 + T::CONST_1;
    const CONST_80: Self = T::CONST_79 + T::CONST_1;
    const CONST_81: Self = T::CONST_80 + T::CONST_1;
    const CONST_82: Self = T::CONST_81 + T::CONST_1;
    const CONST_83: Self = T::CONST_82 + T::CONST_1;
    const CONST_84: Self = T::CONST_83 + T::CONST_1;
    const CONST_85: Self = T::CONST_84 + T::CONST_1;
    const CONST_86: Self = T::CONST_85 + T::CONST_1;
    const CONST_87: Self = T::CONST_86 + T::CONST_1;
    const CONST_88: Self = T::CONST_87 + T::CONST_1;
    const CONST_89: Self = T::CONST_88 + T::CONST_1;
    const CONST_90: Self = T::CONST_89 + T::CONST_1;
    const CONST_91: Self = T::CONST_90 + T::CONST_1;
    const CONST_92: Self = T::CONST_91 + T::CONST_1;
    const CONST_93: Self = T::CONST_92 + T::CONST_1;
    const CONST_94: Self = T::CONST_93 + T::CONST_1;
    const CONST_95: Self = T::CONST_94 + T::CONST_1;
    const CONST_96: Self = T::CONST_95 + T::CONST_1;
    const CONST_97: Self = T::CONST_96 + T::CONST_1;
    const CONST_98: Self = T::CONST_97 + T::CONST_1;
    const CONST_99: Self = T::CONST_98 + T::CONST_1;
    const CONST_100: Self = T::CONST_99 + T::CONST_1;
    const CONST_101: Self = T::CONST_100 + T::CONST_1;
    const CONST_102: Self = T::CONST_101 + T::CONST_1;
    const CONST_103: Self = T::CONST_102 + T::CONST_1;
    const CONST_104: Self = T::CONST_103 + T::CONST_1;
    const CONST_105: Self = T::CONST_104 + T::CONST_1;
    const CONST_106: Self = T::CONST_105 + T::CONST_1;
    const CONST_107: Self = T::CONST_106 + T::CONST_1;
    const CONST_108: Self = T::CONST_107 + T::CONST_1;
    const CONST_109: Self = T::CONST_108 + T::CONST_1;
    const CONST_110: Self = T::CONST_109 + T::CONST_1;
    const CONST_111: Self = T::CONST_110 + T::CONST_1;
    const CONST_112: Self = T::CONST_111 + T::CONST_1;
    const CONST_113: Self = T::CONST_112 + T::CONST_1;
    const CONST_114: Self = T::CONST_113 + T::CONST_1;
    const CONST_115: Self = T::CONST_114 + T::CONST_1;
    const CONST_116: Self = T::CONST_115 + T::CONST_1;
    const CONST_117: Self = T::CONST_116 + T::CONST_1;
    const CONST_118: Self = T::CONST_117 + T::CONST_1;
    const CONST_119: Self = T::CONST_118 + T::CONST_1;
    const CONST_120: Self = T::CONST_119 + T::CONST_1;
    const CONST_121: Self = T::CONST_120 + T::CONST_1;
    const CONST_122: Self = T::CONST_121 + T::CONST_1;
    const CONST_123: Self = T::CONST_122 + T::CONST_1;
    const CONST_124: Self = T::CONST_123 + T::CONST_1;
    const CONST_125: Self = T::CONST_124 + T::CONST_1;
    const CONST_126: Self = T::CONST_125 + T::CONST_1;
    const CONST_127: Self = T::CONST_126 + T::CONST_1;
}
#[allow(missing_docs)]
/// An extended version of [`ConstNumbers128`] covering all numbers from 0 to 255
pub const trait ConstNumbers256: ConstNumbers128 + SupportsRange256 {
    const CONST_128: Self;
    const CONST_129: Self;
    const CONST_130: Self;
    const CONST_131: Self;
    const CONST_132: Self;
    const CONST_133: Self;
    const CONST_134: Self;
    const CONST_135: Self;
    const CONST_136: Self;
    const CONST_137: Self;
    const CONST_138: Self;
    const CONST_139: Self;
    const CONST_140: Self;
    const CONST_141: Self;
    const CONST_142: Self;
    const CONST_143: Self;
    const CONST_144: Self;
    const CONST_145: Self;
    const CONST_146: Self;
    const CONST_147: Self;
    const CONST_148: Self;
    const CONST_149: Self;
    const CONST_150: Self;
    const CONST_151: Self;
    const CONST_152: Self;
    const CONST_153: Self;
    const CONST_154: Self;
    const CONST_155: Self;
    const CONST_156: Self;
    const CONST_157: Self;
    const CONST_158: Self;
    const CONST_159: Self;
    const CONST_160: Self;
    const CONST_161: Self;
    const CONST_162: Self;
    const CONST_163: Self;
    const CONST_164: Self;
    const CONST_165: Self;
    const CONST_166: Self;
    const CONST_167: Self;
    const CONST_168: Self;
    const CONST_169: Self;
    const CONST_170: Self;
    const CONST_171: Self;
    const CONST_172: Self;
    const CONST_173: Self;
    const CONST_174: Self;
    const CONST_175: Self;
    const CONST_176: Self;
    const CONST_177: Self;
    const CONST_178: Self;
    const CONST_179: Self;
    const CONST_180: Self;
    const CONST_181: Self;
    const CONST_182: Self;
    const CONST_183: Self;
    const CONST_184: Self;
    const CONST_185: Self;
    const CONST_186: Self;
    const CONST_187: Self;
    const CONST_188: Self;
    const CONST_189: Self;
    const CONST_190: Self;
    const CONST_191: Self;
    const CONST_192: Self;
    const CONST_193: Self;
    const CONST_194: Self;
    const CONST_195: Self;
    const CONST_196: Self;
    const CONST_197: Self;
    const CONST_198: Self;
    const CONST_199: Self;
    const CONST_200: Self;
    const CONST_201: Self;
    const CONST_202: Self;
    const CONST_203: Self;
    const CONST_204: Self;
    const CONST_205: Self;
    const CONST_206: Self;
    const CONST_207: Self;
    const CONST_208: Self;
    const CONST_209: Self;
    const CONST_210: Self;
    const CONST_211: Self;
    const CONST_212: Self;
    const CONST_213: Self;
    const CONST_214: Self;
    const CONST_215: Self;
    const CONST_216: Self;
    const CONST_217: Self;
    const CONST_218: Self;
    const CONST_219: Self;
    const CONST_220: Self;
    const CONST_221: Self;
    const CONST_222: Self;
    const CONST_223: Self;
    const CONST_224: Self;
    const CONST_225: Self;
    const CONST_226: Self;
    const CONST_227: Self;
    const CONST_228: Self;
    const CONST_229: Self;
    const CONST_230: Self;
    const CONST_231: Self;
    const CONST_232: Self;
    const CONST_233: Self;
    const CONST_234: Self;
    const CONST_235: Self;
    const CONST_236: Self;
    const CONST_237: Self;
    const CONST_238: Self;
    const CONST_239: Self;
    const CONST_240: Self;
    const CONST_241: Self;
    const CONST_242: Self;
    const CONST_243: Self;
    const CONST_244: Self;
    const CONST_245: Self;
    const CONST_246: Self;
    const CONST_247: Self;
    const CONST_248: Self;
    const CONST_249: Self;
    const CONST_250: Self;
    const CONST_251: Self;
    const CONST_252: Self;
    const CONST_253: Self;
    const CONST_254: Self;
    const CONST_255: Self;
}

use mirl_extensions_core::*;
const impl<
    T: ConstOne
        + ConstZero
        + SupportsRange256
        + const ConstNumbers128
        + const core::ops::Add<Output = T>,
> ConstNumbers256 for T
{
    const CONST_128: Self = T::CONST_127 + T::CONST_1;
    const CONST_129: Self = T::CONST_128 + T::CONST_1;
    const CONST_130: Self = T::CONST_129 + T::CONST_1;
    const CONST_131: Self = T::CONST_130 + T::CONST_1;
    const CONST_132: Self = T::CONST_131 + T::CONST_1;
    const CONST_133: Self = T::CONST_132 + T::CONST_1;
    const CONST_134: Self = T::CONST_133 + T::CONST_1;
    const CONST_135: Self = T::CONST_134 + T::CONST_1;
    const CONST_136: Self = T::CONST_135 + T::CONST_1;
    const CONST_137: Self = T::CONST_136 + T::CONST_1;
    const CONST_138: Self = T::CONST_137 + T::CONST_1;
    const CONST_139: Self = T::CONST_138 + T::CONST_1;
    const CONST_140: Self = T::CONST_139 + T::CONST_1;
    const CONST_141: Self = T::CONST_140 + T::CONST_1;
    const CONST_142: Self = T::CONST_141 + T::CONST_1;
    const CONST_143: Self = T::CONST_142 + T::CONST_1;
    const CONST_144: Self = T::CONST_143 + T::CONST_1;
    const CONST_145: Self = T::CONST_144 + T::CONST_1;
    const CONST_146: Self = T::CONST_145 + T::CONST_1;
    const CONST_147: Self = T::CONST_146 + T::CONST_1;
    const CONST_148: Self = T::CONST_147 + T::CONST_1;
    const CONST_149: Self = T::CONST_148 + T::CONST_1;
    const CONST_150: Self = T::CONST_149 + T::CONST_1;
    const CONST_151: Self = T::CONST_150 + T::CONST_1;
    const CONST_152: Self = T::CONST_151 + T::CONST_1;
    const CONST_153: Self = T::CONST_152 + T::CONST_1;
    const CONST_154: Self = T::CONST_153 + T::CONST_1;
    const CONST_155: Self = T::CONST_154 + T::CONST_1;
    const CONST_156: Self = T::CONST_155 + T::CONST_1;
    const CONST_157: Self = T::CONST_156 + T::CONST_1;
    const CONST_158: Self = T::CONST_157 + T::CONST_1;
    const CONST_159: Self = T::CONST_158 + T::CONST_1;
    const CONST_160: Self = T::CONST_159 + T::CONST_1;
    const CONST_161: Self = T::CONST_160 + T::CONST_1;
    const CONST_162: Self = T::CONST_161 + T::CONST_1;
    const CONST_163: Self = T::CONST_162 + T::CONST_1;
    const CONST_164: Self = T::CONST_163 + T::CONST_1;
    const CONST_165: Self = T::CONST_164 + T::CONST_1;
    const CONST_166: Self = T::CONST_165 + T::CONST_1;
    const CONST_167: Self = T::CONST_166 + T::CONST_1;
    const CONST_168: Self = T::CONST_167 + T::CONST_1;
    const CONST_169: Self = T::CONST_168 + T::CONST_1;
    const CONST_170: Self = T::CONST_169 + T::CONST_1;
    const CONST_171: Self = T::CONST_170 + T::CONST_1;
    const CONST_172: Self = T::CONST_171 + T::CONST_1;
    const CONST_173: Self = T::CONST_172 + T::CONST_1;
    const CONST_174: Self = T::CONST_173 + T::CONST_1;
    const CONST_175: Self = T::CONST_174 + T::CONST_1;
    const CONST_176: Self = T::CONST_175 + T::CONST_1;
    const CONST_177: Self = T::CONST_176 + T::CONST_1;
    const CONST_178: Self = T::CONST_177 + T::CONST_1;
    const CONST_179: Self = T::CONST_178 + T::CONST_1;
    const CONST_180: Self = T::CONST_179 + T::CONST_1;
    const CONST_181: Self = T::CONST_180 + T::CONST_1;
    const CONST_182: Self = T::CONST_181 + T::CONST_1;
    const CONST_183: Self = T::CONST_182 + T::CONST_1;
    const CONST_184: Self = T::CONST_183 + T::CONST_1;
    const CONST_185: Self = T::CONST_184 + T::CONST_1;
    const CONST_186: Self = T::CONST_185 + T::CONST_1;
    const CONST_187: Self = T::CONST_186 + T::CONST_1;
    const CONST_188: Self = T::CONST_187 + T::CONST_1;
    const CONST_189: Self = T::CONST_188 + T::CONST_1;
    const CONST_190: Self = T::CONST_189 + T::CONST_1;
    const CONST_191: Self = T::CONST_190 + T::CONST_1;
    const CONST_192: Self = T::CONST_191 + T::CONST_1;
    const CONST_193: Self = T::CONST_192 + T::CONST_1;
    const CONST_194: Self = T::CONST_193 + T::CONST_1;
    const CONST_195: Self = T::CONST_194 + T::CONST_1;
    const CONST_196: Self = T::CONST_195 + T::CONST_1;
    const CONST_197: Self = T::CONST_196 + T::CONST_1;
    const CONST_198: Self = T::CONST_197 + T::CONST_1;
    const CONST_199: Self = T::CONST_198 + T::CONST_1;
    const CONST_200: Self = T::CONST_199 + T::CONST_1;
    const CONST_201: Self = T::CONST_200 + T::CONST_1;
    const CONST_202: Self = T::CONST_201 + T::CONST_1;
    const CONST_203: Self = T::CONST_202 + T::CONST_1;
    const CONST_204: Self = T::CONST_203 + T::CONST_1;
    const CONST_205: Self = T::CONST_204 + T::CONST_1;
    const CONST_206: Self = T::CONST_205 + T::CONST_1;
    const CONST_207: Self = T::CONST_206 + T::CONST_1;
    const CONST_208: Self = T::CONST_207 + T::CONST_1;
    const CONST_209: Self = T::CONST_208 + T::CONST_1;
    const CONST_210: Self = T::CONST_209 + T::CONST_1;
    const CONST_211: Self = T::CONST_210 + T::CONST_1;
    const CONST_212: Self = T::CONST_211 + T::CONST_1;
    const CONST_213: Self = T::CONST_212 + T::CONST_1;
    const CONST_214: Self = T::CONST_213 + T::CONST_1;
    const CONST_215: Self = T::CONST_214 + T::CONST_1;
    const CONST_216: Self = T::CONST_215 + T::CONST_1;
    const CONST_217: Self = T::CONST_216 + T::CONST_1;
    const CONST_218: Self = T::CONST_217 + T::CONST_1;
    const CONST_219: Self = T::CONST_218 + T::CONST_1;
    const CONST_220: Self = T::CONST_219 + T::CONST_1;
    const CONST_221: Self = T::CONST_220 + T::CONST_1;
    const CONST_222: Self = T::CONST_221 + T::CONST_1;
    const CONST_223: Self = T::CONST_222 + T::CONST_1;
    const CONST_224: Self = T::CONST_223 + T::CONST_1;
    const CONST_225: Self = T::CONST_224 + T::CONST_1;
    const CONST_226: Self = T::CONST_225 + T::CONST_1;
    const CONST_227: Self = T::CONST_226 + T::CONST_1;
    const CONST_228: Self = T::CONST_227 + T::CONST_1;
    const CONST_229: Self = T::CONST_228 + T::CONST_1;
    const CONST_230: Self = T::CONST_229 + T::CONST_1;
    const CONST_231: Self = T::CONST_230 + T::CONST_1;
    const CONST_232: Self = T::CONST_231 + T::CONST_1;
    const CONST_233: Self = T::CONST_232 + T::CONST_1;
    const CONST_234: Self = T::CONST_233 + T::CONST_1;
    const CONST_235: Self = T::CONST_234 + T::CONST_1;
    const CONST_236: Self = T::CONST_235 + T::CONST_1;
    const CONST_237: Self = T::CONST_236 + T::CONST_1;
    const CONST_238: Self = T::CONST_237 + T::CONST_1;
    const CONST_239: Self = T::CONST_238 + T::CONST_1;
    const CONST_240: Self = T::CONST_239 + T::CONST_1;
    const CONST_241: Self = T::CONST_240 + T::CONST_1;
    const CONST_242: Self = T::CONST_241 + T::CONST_1;
    const CONST_243: Self = T::CONST_242 + T::CONST_1;
    const CONST_244: Self = T::CONST_243 + T::CONST_1;
    const CONST_245: Self = T::CONST_244 + T::CONST_1;
    const CONST_246: Self = T::CONST_245 + T::CONST_1;
    const CONST_247: Self = T::CONST_246 + T::CONST_1;
    const CONST_248: Self = T::CONST_247 + T::CONST_1;
    const CONST_249: Self = T::CONST_248 + T::CONST_1;
    const CONST_250: Self = T::CONST_249 + T::CONST_1;
    const CONST_251: Self = T::CONST_250 + T::CONST_1;
    const CONST_252: Self = T::CONST_251 + T::CONST_1;
    const CONST_253: Self = T::CONST_252 + T::CONST_1;
    const CONST_254: Self = T::CONST_253 + T::CONST_1;
    const CONST_255: Self = T::CONST_254 + T::CONST_1;
}
