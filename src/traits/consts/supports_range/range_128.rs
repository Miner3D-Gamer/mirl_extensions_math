use mirl_extensions_core::{One, SupportsRange128, Zero};
#[allow(missing_docs)] // TODO: Docs
/// Get any number between 0 and 128
///
/// An extended version of the [`One`](crate::math::One)/[`Zero`](crate::math::Zero) traits covering all numbers from 0 to 127
pub const trait Numbers128: SupportsRange128 + One + Zero {
    fn num_0() -> Self;
    fn num_1() -> Self;
    fn num_2() -> Self;
    fn num_3() -> Self;
    fn num_4() -> Self;
    fn num_5() -> Self;
    fn num_6() -> Self;
    fn num_7() -> Self;
    fn num_8() -> Self;
    fn num_9() -> Self;
    fn num_10() -> Self;
    fn num_11() -> Self;
    fn num_12() -> Self;
    fn num_13() -> Self;
    fn num_14() -> Self;
    fn num_15() -> Self;
    fn num_16() -> Self;
    fn num_17() -> Self;
    fn num_18() -> Self;
    fn num_19() -> Self;
    fn num_20() -> Self;
    fn num_21() -> Self;
    fn num_22() -> Self;
    fn num_23() -> Self;
    fn num_24() -> Self;
    fn num_25() -> Self;
    fn num_26() -> Self;
    fn num_27() -> Self;
    fn num_28() -> Self;
    fn num_29() -> Self;
    fn num_30() -> Self;
    fn num_31() -> Self;
    fn num_32() -> Self;
    fn num_33() -> Self;
    fn num_34() -> Self;
    fn num_35() -> Self;
    fn num_36() -> Self;
    fn num_37() -> Self;
    fn num_38() -> Self;
    fn num_39() -> Self;
    fn num_40() -> Self;
    fn num_41() -> Self;
    fn num_42() -> Self;
    fn num_43() -> Self;
    fn num_44() -> Self;
    fn num_45() -> Self;
    fn num_46() -> Self;
    fn num_47() -> Self;
    fn num_48() -> Self;
    fn num_49() -> Self;
    fn num_50() -> Self;
    fn num_51() -> Self;
    fn num_52() -> Self;
    fn num_53() -> Self;
    fn num_54() -> Self;
    fn num_55() -> Self;
    fn num_56() -> Self;
    fn num_57() -> Self;
    fn num_58() -> Self;
    fn num_59() -> Self;
    fn num_60() -> Self;
    fn num_61() -> Self;
    fn num_62() -> Self;
    fn num_63() -> Self;
    fn num_64() -> Self;
    fn num_65() -> Self;
    fn num_66() -> Self;
    fn num_67() -> Self;
    fn num_68() -> Self;
    fn num_69() -> Self;
    fn num_70() -> Self;
    fn num_71() -> Self;
    fn num_72() -> Self;
    fn num_73() -> Self;
    fn num_74() -> Self;
    fn num_75() -> Self;
    fn num_76() -> Self;
    fn num_77() -> Self;
    fn num_78() -> Self;
    fn num_79() -> Self;
    fn num_80() -> Self;
    fn num_81() -> Self;
    fn num_82() -> Self;
    fn num_83() -> Self;
    fn num_84() -> Self;
    fn num_85() -> Self;
    fn num_86() -> Self;
    fn num_87() -> Self;
    fn num_88() -> Self;
    fn num_89() -> Self;
    fn num_90() -> Self;
    fn num_91() -> Self;
    fn num_92() -> Self;
    fn num_93() -> Self;
    fn num_94() -> Self;
    fn num_95() -> Self;
    fn num_96() -> Self;
    fn num_97() -> Self;
    fn num_98() -> Self;
    fn num_99() -> Self;
    fn num_100() -> Self;
    fn num_101() -> Self;
    fn num_102() -> Self;
    fn num_103() -> Self;
    fn num_104() -> Self;
    fn num_105() -> Self;
    fn num_106() -> Self;
    fn num_107() -> Self;
    fn num_108() -> Self;
    fn num_109() -> Self;
    fn num_110() -> Self;
    fn num_111() -> Self;
    fn num_112() -> Self;
    fn num_113() -> Self;
    fn num_114() -> Self;
    fn num_115() -> Self;
    fn num_116() -> Self;
    fn num_117() -> Self;
    fn num_118() -> Self;
    fn num_119() -> Self;
    fn num_120() -> Self;
    fn num_121() -> Self;
    fn num_122() -> Self;
    fn num_123() -> Self;
    fn num_124() -> Self;
    fn num_125() -> Self;
    fn num_126() -> Self;
    fn num_127() -> Self;
}
const impl<T: SupportsRange128 + const One + const Zero + const core::ops::Add<Output = T>>
    Numbers128 for T
{
    fn num_0() -> Self {
        Self::zero()
    }
    fn num_1() -> Self {
        Self::one()
    }
    fn num_2() -> Self {
        Self::num_1() + Self::num_1()
    }
    fn num_3() -> Self {
        Self::num_2() + Self::num_1()
    }
    fn num_4() -> Self {
        Self::num_3() + Self::num_1()
    }
    fn num_5() -> Self {
        Self::num_4() + Self::num_1()
    }
    fn num_6() -> Self {
        Self::num_5() + Self::num_1()
    }
    fn num_7() -> Self {
        Self::num_6() + Self::num_1()
    }
    fn num_8() -> Self {
        Self::num_7() + Self::num_1()
    }
    fn num_9() -> Self {
        Self::num_8() + Self::num_1()
    }
    fn num_10() -> Self {
        Self::num_9() + Self::num_1()
    }
    fn num_11() -> Self {
        Self::num_10() + Self::num_1()
    }
    fn num_12() -> Self {
        Self::num_11() + Self::num_1()
    }
    fn num_13() -> Self {
        Self::num_12() + Self::num_1()
    }
    fn num_14() -> Self {
        Self::num_13() + Self::num_1()
    }
    fn num_15() -> Self {
        Self::num_14() + Self::num_1()
    }
    fn num_16() -> Self {
        Self::num_15() + Self::num_1()
    }
    fn num_17() -> Self {
        Self::num_16() + Self::num_1()
    }
    fn num_18() -> Self {
        Self::num_17() + Self::num_1()
    }
    fn num_19() -> Self {
        Self::num_18() + Self::num_1()
    }
    fn num_20() -> Self {
        Self::num_19() + Self::num_1()
    }
    fn num_21() -> Self {
        Self::num_20() + Self::num_1()
    }
    fn num_22() -> Self {
        Self::num_21() + Self::num_1()
    }
    fn num_23() -> Self {
        Self::num_22() + Self::num_1()
    }
    fn num_24() -> Self {
        Self::num_23() + Self::num_1()
    }
    fn num_25() -> Self {
        Self::num_24() + Self::num_1()
    }
    fn num_26() -> Self {
        Self::num_25() + Self::num_1()
    }
    fn num_27() -> Self {
        Self::num_26() + Self::num_1()
    }
    fn num_28() -> Self {
        Self::num_27() + Self::num_1()
    }
    fn num_29() -> Self {
        Self::num_28() + Self::num_1()
    }
    fn num_30() -> Self {
        Self::num_29() + Self::num_1()
    }
    fn num_31() -> Self {
        Self::num_30() + Self::num_1()
    }
    fn num_32() -> Self {
        Self::num_31() + Self::num_1()
    }
    fn num_33() -> Self {
        Self::num_32() + Self::num_1()
    }
    fn num_34() -> Self {
        Self::num_33() + Self::num_1()
    }
    fn num_35() -> Self {
        Self::num_34() + Self::num_1()
    }
    fn num_36() -> Self {
        Self::num_35() + Self::num_1()
    }
    fn num_37() -> Self {
        Self::num_36() + Self::num_1()
    }
    fn num_38() -> Self {
        Self::num_37() + Self::num_1()
    }
    fn num_39() -> Self {
        Self::num_38() + Self::num_1()
    }
    fn num_40() -> Self {
        Self::num_39() + Self::num_1()
    }
    fn num_41() -> Self {
        Self::num_40() + Self::num_1()
    }
    fn num_42() -> Self {
        Self::num_41() + Self::num_1()
    }
    fn num_43() -> Self {
        Self::num_42() + Self::num_1()
    }
    fn num_44() -> Self {
        Self::num_43() + Self::num_1()
    }
    fn num_45() -> Self {
        Self::num_44() + Self::num_1()
    }
    fn num_46() -> Self {
        Self::num_45() + Self::num_1()
    }
    fn num_47() -> Self {
        Self::num_46() + Self::num_1()
    }
    fn num_48() -> Self {
        Self::num_47() + Self::num_1()
    }
    fn num_49() -> Self {
        Self::num_48() + Self::num_1()
    }
    fn num_50() -> Self {
        Self::num_49() + Self::num_1()
    }
    fn num_51() -> Self {
        Self::num_50() + Self::num_1()
    }
    fn num_52() -> Self {
        Self::num_51() + Self::num_1()
    }
    fn num_53() -> Self {
        Self::num_52() + Self::num_1()
    }
    fn num_54() -> Self {
        Self::num_53() + Self::num_1()
    }
    fn num_55() -> Self {
        Self::num_54() + Self::num_1()
    }
    fn num_56() -> Self {
        Self::num_55() + Self::num_1()
    }
    fn num_57() -> Self {
        Self::num_56() + Self::num_1()
    }
    fn num_58() -> Self {
        Self::num_57() + Self::num_1()
    }
    fn num_59() -> Self {
        Self::num_58() + Self::num_1()
    }
    fn num_60() -> Self {
        Self::num_59() + Self::num_1()
    }
    fn num_61() -> Self {
        Self::num_60() + Self::num_1()
    }
    fn num_62() -> Self {
        Self::num_61() + Self::num_1()
    }
    fn num_63() -> Self {
        Self::num_62() + Self::num_1()
    }
    fn num_64() -> Self {
        Self::num_63() + Self::num_1()
    }
    fn num_65() -> Self {
        Self::num_64() + Self::num_1()
    }
    fn num_66() -> Self {
        Self::num_65() + Self::num_1()
    }
    fn num_67() -> Self {
        Self::num_66() + Self::num_1()
    }
    fn num_68() -> Self {
        Self::num_67() + Self::num_1()
    }
    fn num_69() -> Self {
        Self::num_68() + Self::num_1()
    }
    fn num_70() -> Self {
        Self::num_69() + Self::num_1()
    }
    fn num_71() -> Self {
        Self::num_70() + Self::num_1()
    }
    fn num_72() -> Self {
        Self::num_71() + Self::num_1()
    }
    fn num_73() -> Self {
        Self::num_72() + Self::num_1()
    }
    fn num_74() -> Self {
        Self::num_73() + Self::num_1()
    }
    fn num_75() -> Self {
        Self::num_74() + Self::num_1()
    }
    fn num_76() -> Self {
        Self::num_75() + Self::num_1()
    }
    fn num_77() -> Self {
        Self::num_76() + Self::num_1()
    }
    fn num_78() -> Self {
        Self::num_77() + Self::num_1()
    }
    fn num_79() -> Self {
        Self::num_78() + Self::num_1()
    }
    fn num_80() -> Self {
        Self::num_79() + Self::num_1()
    }
    fn num_81() -> Self {
        Self::num_80() + Self::num_1()
    }
    fn num_82() -> Self {
        Self::num_81() + Self::num_1()
    }
    fn num_83() -> Self {
        Self::num_82() + Self::num_1()
    }
    fn num_84() -> Self {
        Self::num_83() + Self::num_1()
    }
    fn num_85() -> Self {
        Self::num_84() + Self::num_1()
    }
    fn num_86() -> Self {
        Self::num_85() + Self::num_1()
    }
    fn num_87() -> Self {
        Self::num_86() + Self::num_1()
    }
    fn num_88() -> Self {
        Self::num_87() + Self::num_1()
    }
    fn num_89() -> Self {
        Self::num_88() + Self::num_1()
    }
    fn num_90() -> Self {
        Self::num_89() + Self::num_1()
    }
    fn num_91() -> Self {
        Self::num_90() + Self::num_1()
    }
    fn num_92() -> Self {
        Self::num_91() + Self::num_1()
    }
    fn num_93() -> Self {
        Self::num_92() + Self::num_1()
    }
    fn num_94() -> Self {
        Self::num_93() + Self::num_1()
    }
    fn num_95() -> Self {
        Self::num_94() + Self::num_1()
    }
    fn num_96() -> Self {
        Self::num_95() + Self::num_1()
    }
    fn num_97() -> Self {
        Self::num_96() + Self::num_1()
    }
    fn num_98() -> Self {
        Self::num_97() + Self::num_1()
    }
    fn num_99() -> Self {
        Self::num_98() + Self::num_1()
    }
    fn num_100() -> Self {
        Self::num_99() + Self::num_1()
    }
    fn num_101() -> Self {
        Self::num_100() + Self::num_1()
    }
    fn num_102() -> Self {
        Self::num_101() + Self::num_1()
    }
    fn num_103() -> Self {
        Self::num_102() + Self::num_1()
    }
    fn num_104() -> Self {
        Self::num_103() + Self::num_1()
    }
    fn num_105() -> Self {
        Self::num_104() + Self::num_1()
    }
    fn num_106() -> Self {
        Self::num_105() + Self::num_1()
    }
    fn num_107() -> Self {
        Self::num_106() + Self::num_1()
    }
    fn num_108() -> Self {
        Self::num_107() + Self::num_1()
    }
    fn num_109() -> Self {
        Self::num_108() + Self::num_1()
    }
    fn num_110() -> Self {
        Self::num_109() + Self::num_1()
    }
    fn num_111() -> Self {
        Self::num_110() + Self::num_1()
    }
    fn num_112() -> Self {
        Self::num_111() + Self::num_1()
    }
    fn num_113() -> Self {
        Self::num_112() + Self::num_1()
    }
    fn num_114() -> Self {
        Self::num_113() + Self::num_1()
    }
    fn num_115() -> Self {
        Self::num_114() + Self::num_1()
    }
    fn num_116() -> Self {
        Self::num_115() + Self::num_1()
    }
    fn num_117() -> Self {
        Self::num_116() + Self::num_1()
    }
    fn num_118() -> Self {
        Self::num_117() + Self::num_1()
    }
    fn num_119() -> Self {
        Self::num_118() + Self::num_1()
    }
    fn num_120() -> Self {
        Self::num_119() + Self::num_1()
    }
    fn num_121() -> Self {
        Self::num_120() + Self::num_1()
    }
    fn num_122() -> Self {
        Self::num_121() + Self::num_1()
    }
    fn num_123() -> Self {
        Self::num_122() + Self::num_1()
    }
    fn num_124() -> Self {
        Self::num_123() + Self::num_1()
    }
    fn num_125() -> Self {
        Self::num_124() + Self::num_1()
    }
    fn num_126() -> Self {
        Self::num_125() + Self::num_1()
    }
    fn num_127() -> Self {
        Self::num_126() + Self::num_1()
    }
}
