use mirl_extensions_core::{One, SupportsRange256, Zero};

use crate::Numbers128;

#[allow(missing_docs)] // TODO: Docs
/// Get any number between 128 and 255
///
/// An extended version of the [`Numbers128`] trait covering all numbers from 128 to 255
pub const trait Numbers256: SupportsRange256 + Numbers128 {
    fn num_128() -> Self;
    fn num_129() -> Self;
    fn num_130() -> Self;
    fn num_131() -> Self;
    fn num_132() -> Self;
    fn num_133() -> Self;
    fn num_134() -> Self;
    fn num_135() -> Self;
    fn num_136() -> Self;
    fn num_137() -> Self;
    fn num_138() -> Self;
    fn num_139() -> Self;
    fn num_140() -> Self;
    fn num_141() -> Self;
    fn num_142() -> Self;
    fn num_143() -> Self;
    fn num_144() -> Self;
    fn num_145() -> Self;
    fn num_146() -> Self;
    fn num_147() -> Self;
    fn num_148() -> Self;
    fn num_149() -> Self;
    fn num_150() -> Self;
    fn num_151() -> Self;
    fn num_152() -> Self;
    fn num_153() -> Self;
    fn num_154() -> Self;
    fn num_155() -> Self;
    fn num_156() -> Self;
    fn num_157() -> Self;
    fn num_158() -> Self;
    fn num_159() -> Self;
    fn num_160() -> Self;
    fn num_161() -> Self;
    fn num_162() -> Self;
    fn num_163() -> Self;
    fn num_164() -> Self;
    fn num_165() -> Self;
    fn num_166() -> Self;
    fn num_167() -> Self;
    fn num_168() -> Self;
    fn num_169() -> Self;
    fn num_170() -> Self;
    fn num_171() -> Self;
    fn num_172() -> Self;
    fn num_173() -> Self;
    fn num_174() -> Self;
    fn num_175() -> Self;
    fn num_176() -> Self;
    fn num_177() -> Self;
    fn num_178() -> Self;
    fn num_179() -> Self;
    fn num_180() -> Self;
    fn num_181() -> Self;
    fn num_182() -> Self;
    fn num_183() -> Self;
    fn num_184() -> Self;
    fn num_185() -> Self;
    fn num_186() -> Self;
    fn num_187() -> Self;
    fn num_188() -> Self;
    fn num_189() -> Self;
    fn num_190() -> Self;
    fn num_191() -> Self;
    fn num_192() -> Self;
    fn num_193() -> Self;
    fn num_194() -> Self;
    fn num_195() -> Self;
    fn num_196() -> Self;
    fn num_197() -> Self;
    fn num_198() -> Self;
    fn num_199() -> Self;
    fn num_200() -> Self;
    fn num_201() -> Self;
    fn num_202() -> Self;
    fn num_203() -> Self;
    fn num_204() -> Self;
    fn num_205() -> Self;
    fn num_206() -> Self;
    fn num_207() -> Self;
    fn num_208() -> Self;
    fn num_209() -> Self;
    fn num_210() -> Self;
    fn num_211() -> Self;
    fn num_212() -> Self;
    fn num_213() -> Self;
    fn num_214() -> Self;
    fn num_215() -> Self;
    fn num_216() -> Self;
    fn num_217() -> Self;
    fn num_218() -> Self;
    fn num_219() -> Self;
    fn num_220() -> Self;
    fn num_221() -> Self;
    fn num_222() -> Self;
    fn num_223() -> Self;
    fn num_224() -> Self;
    fn num_225() -> Self;
    fn num_226() -> Self;
    fn num_227() -> Self;
    fn num_228() -> Self;
    fn num_229() -> Self;
    fn num_230() -> Self;
    fn num_231() -> Self;
    fn num_232() -> Self;
    fn num_233() -> Self;
    fn num_234() -> Self;
    fn num_235() -> Self;
    fn num_236() -> Self;
    fn num_237() -> Self;
    fn num_238() -> Self;
    fn num_239() -> Self;
    fn num_240() -> Self;
    fn num_241() -> Self;
    fn num_242() -> Self;
    fn num_243() -> Self;
    fn num_244() -> Self;
    fn num_245() -> Self;
    fn num_246() -> Self;
    fn num_247() -> Self;
    fn num_248() -> Self;
    fn num_249() -> Self;
    fn num_250() -> Self;
    fn num_251() -> Self;
    fn num_252() -> Self;
    fn num_253() -> Self;
    fn num_254() -> Self;
    fn num_255() -> Self;
}
const impl<
    T: SupportsRange256 + const One + const Zero + const core::ops::Add<Output = T> + const Numbers128,
> Numbers256 for T
{
    fn num_128() -> Self {
        Self::num_127() + Self::num_1()
    }
    fn num_129() -> Self {
        Self::num_128() + Self::num_1()
    }
    fn num_130() -> Self {
        Self::num_129() + Self::num_1()
    }
    fn num_131() -> Self {
        Self::num_130() + Self::num_1()
    }
    fn num_132() -> Self {
        Self::num_131() + Self::num_1()
    }
    fn num_133() -> Self {
        Self::num_132() + Self::num_1()
    }
    fn num_134() -> Self {
        Self::num_133() + Self::num_1()
    }
    fn num_135() -> Self {
        Self::num_134() + Self::num_1()
    }
    fn num_136() -> Self {
        Self::num_135() + Self::num_1()
    }
    fn num_137() -> Self {
        Self::num_136() + Self::num_1()
    }
    fn num_138() -> Self {
        Self::num_137() + Self::num_1()
    }
    fn num_139() -> Self {
        Self::num_138() + Self::num_1()
    }
    fn num_140() -> Self {
        Self::num_139() + Self::num_1()
    }
    fn num_141() -> Self {
        Self::num_140() + Self::num_1()
    }
    fn num_142() -> Self {
        Self::num_141() + Self::num_1()
    }
    fn num_143() -> Self {
        Self::num_142() + Self::num_1()
    }
    fn num_144() -> Self {
        Self::num_143() + Self::num_1()
    }
    fn num_145() -> Self {
        Self::num_144() + Self::num_1()
    }
    fn num_146() -> Self {
        Self::num_145() + Self::num_1()
    }
    fn num_147() -> Self {
        Self::num_146() + Self::num_1()
    }
    fn num_148() -> Self {
        Self::num_147() + Self::num_1()
    }
    fn num_149() -> Self {
        Self::num_148() + Self::num_1()
    }
    fn num_150() -> Self {
        Self::num_149() + Self::num_1()
    }
    fn num_151() -> Self {
        Self::num_150() + Self::num_1()
    }
    fn num_152() -> Self {
        Self::num_151() + Self::num_1()
    }
    fn num_153() -> Self {
        Self::num_152() + Self::num_1()
    }
    fn num_154() -> Self {
        Self::num_153() + Self::num_1()
    }
    fn num_155() -> Self {
        Self::num_154() + Self::num_1()
    }
    fn num_156() -> Self {
        Self::num_155() + Self::num_1()
    }
    fn num_157() -> Self {
        Self::num_156() + Self::num_1()
    }
    fn num_158() -> Self {
        Self::num_157() + Self::num_1()
    }
    fn num_159() -> Self {
        Self::num_158() + Self::num_1()
    }
    fn num_160() -> Self {
        Self::num_159() + Self::num_1()
    }
    fn num_161() -> Self {
        Self::num_160() + Self::num_1()
    }
    fn num_162() -> Self {
        Self::num_161() + Self::num_1()
    }
    fn num_163() -> Self {
        Self::num_162() + Self::num_1()
    }
    fn num_164() -> Self {
        Self::num_163() + Self::num_1()
    }
    fn num_165() -> Self {
        Self::num_164() + Self::num_1()
    }
    fn num_166() -> Self {
        Self::num_165() + Self::num_1()
    }
    fn num_167() -> Self {
        Self::num_166() + Self::num_1()
    }
    fn num_168() -> Self {
        Self::num_167() + Self::num_1()
    }
    fn num_169() -> Self {
        Self::num_168() + Self::num_1()
    }
    fn num_170() -> Self {
        Self::num_169() + Self::num_1()
    }
    fn num_171() -> Self {
        Self::num_170() + Self::num_1()
    }
    fn num_172() -> Self {
        Self::num_171() + Self::num_1()
    }
    fn num_173() -> Self {
        Self::num_172() + Self::num_1()
    }
    fn num_174() -> Self {
        Self::num_173() + Self::num_1()
    }
    fn num_175() -> Self {
        Self::num_174() + Self::num_1()
    }
    fn num_176() -> Self {
        Self::num_175() + Self::num_1()
    }
    fn num_177() -> Self {
        Self::num_176() + Self::num_1()
    }
    fn num_178() -> Self {
        Self::num_177() + Self::num_1()
    }
    fn num_179() -> Self {
        Self::num_178() + Self::num_1()
    }
    fn num_180() -> Self {
        Self::num_179() + Self::num_1()
    }
    fn num_181() -> Self {
        Self::num_180() + Self::num_1()
    }
    fn num_182() -> Self {
        Self::num_181() + Self::num_1()
    }
    fn num_183() -> Self {
        Self::num_182() + Self::num_1()
    }
    fn num_184() -> Self {
        Self::num_183() + Self::num_1()
    }
    fn num_185() -> Self {
        Self::num_184() + Self::num_1()
    }
    fn num_186() -> Self {
        Self::num_185() + Self::num_1()
    }
    fn num_187() -> Self {
        Self::num_186() + Self::num_1()
    }
    fn num_188() -> Self {
        Self::num_187() + Self::num_1()
    }
    fn num_189() -> Self {
        Self::num_188() + Self::num_1()
    }
    fn num_190() -> Self {
        Self::num_189() + Self::num_1()
    }
    fn num_191() -> Self {
        Self::num_190() + Self::num_1()
    }
    fn num_192() -> Self {
        Self::num_191() + Self::num_1()
    }
    fn num_193() -> Self {
        Self::num_192() + Self::num_1()
    }
    fn num_194() -> Self {
        Self::num_193() + Self::num_1()
    }
    fn num_195() -> Self {
        Self::num_194() + Self::num_1()
    }
    fn num_196() -> Self {
        Self::num_195() + Self::num_1()
    }
    fn num_197() -> Self {
        Self::num_196() + Self::num_1()
    }
    fn num_198() -> Self {
        Self::num_197() + Self::num_1()
    }
    fn num_199() -> Self {
        Self::num_198() + Self::num_1()
    }
    fn num_200() -> Self {
        Self::num_199() + Self::num_1()
    }
    fn num_201() -> Self {
        Self::num_200() + Self::num_1()
    }
    fn num_202() -> Self {
        Self::num_201() + Self::num_1()
    }
    fn num_203() -> Self {
        Self::num_202() + Self::num_1()
    }
    fn num_204() -> Self {
        Self::num_203() + Self::num_1()
    }
    fn num_205() -> Self {
        Self::num_204() + Self::num_1()
    }
    fn num_206() -> Self {
        Self::num_205() + Self::num_1()
    }
    fn num_207() -> Self {
        Self::num_206() + Self::num_1()
    }
    fn num_208() -> Self {
        Self::num_207() + Self::num_1()
    }
    fn num_209() -> Self {
        Self::num_208() + Self::num_1()
    }
    fn num_210() -> Self {
        Self::num_209() + Self::num_1()
    }
    fn num_211() -> Self {
        Self::num_210() + Self::num_1()
    }
    fn num_212() -> Self {
        Self::num_211() + Self::num_1()
    }
    fn num_213() -> Self {
        Self::num_212() + Self::num_1()
    }
    fn num_214() -> Self {
        Self::num_213() + Self::num_1()
    }
    fn num_215() -> Self {
        Self::num_214() + Self::num_1()
    }
    fn num_216() -> Self {
        Self::num_215() + Self::num_1()
    }
    fn num_217() -> Self {
        Self::num_216() + Self::num_1()
    }
    fn num_218() -> Self {
        Self::num_217() + Self::num_1()
    }
    fn num_219() -> Self {
        Self::num_218() + Self::num_1()
    }
    fn num_220() -> Self {
        Self::num_219() + Self::num_1()
    }
    fn num_221() -> Self {
        Self::num_220() + Self::num_1()
    }
    fn num_222() -> Self {
        Self::num_221() + Self::num_1()
    }
    fn num_223() -> Self {
        Self::num_222() + Self::num_1()
    }
    fn num_224() -> Self {
        Self::num_223() + Self::num_1()
    }
    fn num_225() -> Self {
        Self::num_224() + Self::num_1()
    }
    fn num_226() -> Self {
        Self::num_225() + Self::num_1()
    }
    fn num_227() -> Self {
        Self::num_226() + Self::num_1()
    }
    fn num_228() -> Self {
        Self::num_227() + Self::num_1()
    }
    fn num_229() -> Self {
        Self::num_228() + Self::num_1()
    }
    fn num_230() -> Self {
        Self::num_229() + Self::num_1()
    }
    fn num_231() -> Self {
        Self::num_230() + Self::num_1()
    }
    fn num_232() -> Self {
        Self::num_231() + Self::num_1()
    }
    fn num_233() -> Self {
        Self::num_232() + Self::num_1()
    }
    fn num_234() -> Self {
        Self::num_233() + Self::num_1()
    }
    fn num_235() -> Self {
        Self::num_234() + Self::num_1()
    }
    fn num_236() -> Self {
        Self::num_235() + Self::num_1()
    }
    fn num_237() -> Self {
        Self::num_236() + Self::num_1()
    }
    fn num_238() -> Self {
        Self::num_237() + Self::num_1()
    }
    fn num_239() -> Self {
        Self::num_238() + Self::num_1()
    }
    fn num_240() -> Self {
        Self::num_239() + Self::num_1()
    }
    fn num_241() -> Self {
        Self::num_240() + Self::num_1()
    }
    fn num_242() -> Self {
        Self::num_241() + Self::num_1()
    }
    fn num_243() -> Self {
        Self::num_242() + Self::num_1()
    }
    fn num_244() -> Self {
        Self::num_243() + Self::num_1()
    }
    fn num_245() -> Self {
        Self::num_244() + Self::num_1()
    }
    fn num_246() -> Self {
        Self::num_245() + Self::num_1()
    }
    fn num_247() -> Self {
        Self::num_246() + Self::num_1()
    }
    fn num_248() -> Self {
        Self::num_247() + Self::num_1()
    }
    fn num_249() -> Self {
        Self::num_248() + Self::num_1()
    }
    fn num_250() -> Self {
        Self::num_249() + Self::num_1()
    }
    fn num_251() -> Self {
        Self::num_250() + Self::num_1()
    }
    fn num_252() -> Self {
        Self::num_251() + Self::num_1()
    }
    fn num_253() -> Self {
        Self::num_252() + Self::num_1()
    }
    fn num_254() -> Self {
        Self::num_253() + Self::num_1()
    }
    fn num_255() -> Self {
        Self::num_254() + Self::num_1()
    }
}
