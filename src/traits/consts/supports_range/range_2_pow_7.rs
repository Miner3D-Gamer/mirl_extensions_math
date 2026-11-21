use mirl_extensions_core::SupportsRange2Pow7;

use crate::Numbers2Pow6;
#[allow(missing_docs)] // TODO: Docs
/// Get any number between 0 and 127
///
/// An extended version of the [`One`](crate::math::One)/[`Zero`](crate::math::Zero) traits covering all numbers from 0 to 127
pub const trait Numbers2Pow7: SupportsRange2Pow7 + [const] Numbers2Pow6 {
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
const impl<T: SupportsRange2Pow7 + const Numbers2Pow6 + const core::ops::Add<Output = T>>
    Numbers2Pow7 for T
{
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
