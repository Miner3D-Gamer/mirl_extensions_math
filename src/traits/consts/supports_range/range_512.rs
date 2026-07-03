use crate::Numbers128;
use crate::{Numbers256};
use mirl_extensions_core::{One, SupportsRange512, Zero};

/// Get any number between 256 and 511
///
/// An extended version of the [`Numbers256`] trait covering all numbers from 256 to 511
#[allow(missing_docs)]
pub const trait Numbers512: SupportsRange512 + Numbers256 {
    fn num_256() -> Self;
    fn num_257() -> Self;
    fn num_258() -> Self;
    fn num_259() -> Self;
    fn num_260() -> Self;
    fn num_261() -> Self;
    fn num_262() -> Self;
    fn num_263() -> Self;
    fn num_264() -> Self;
    fn num_265() -> Self;
    fn num_266() -> Self;
    fn num_267() -> Self;
    fn num_268() -> Self;
    fn num_269() -> Self;
    fn num_270() -> Self;
    fn num_271() -> Self;
    fn num_272() -> Self;
    fn num_273() -> Self;
    fn num_274() -> Self;
    fn num_275() -> Self;
    fn num_276() -> Self;
    fn num_277() -> Self;
    fn num_278() -> Self;
    fn num_279() -> Self;
    fn num_280() -> Self;
    fn num_281() -> Self;
    fn num_282() -> Self;
    fn num_283() -> Self;
    fn num_284() -> Self;
    fn num_285() -> Self;
    fn num_286() -> Self;
    fn num_287() -> Self;
    fn num_288() -> Self;
    fn num_289() -> Self;
    fn num_290() -> Self;
    fn num_291() -> Self;
    fn num_292() -> Self;
    fn num_293() -> Self;
    fn num_294() -> Self;
    fn num_295() -> Self;
    fn num_296() -> Self;
    fn num_297() -> Self;
    fn num_298() -> Self;
    fn num_299() -> Self;
    fn num_300() -> Self;
    fn num_301() -> Self;
    fn num_302() -> Self;
    fn num_303() -> Self;
    fn num_304() -> Self;
    fn num_305() -> Self;
    fn num_306() -> Self;
    fn num_307() -> Self;
    fn num_308() -> Self;
    fn num_309() -> Self;
    fn num_310() -> Self;
    fn num_311() -> Self;
    fn num_312() -> Self;
    fn num_313() -> Self;
    fn num_314() -> Self;
    fn num_315() -> Self;
    fn num_316() -> Self;
    fn num_317() -> Self;
    fn num_318() -> Self;
    fn num_319() -> Self;
    fn num_320() -> Self;
    fn num_321() -> Self;
    fn num_322() -> Self;
    fn num_323() -> Self;
    fn num_324() -> Self;
    fn num_325() -> Self;
    fn num_326() -> Self;
    fn num_327() -> Self;
    fn num_328() -> Self;
    fn num_329() -> Self;
    fn num_330() -> Self;
    fn num_331() -> Self;
    fn num_332() -> Self;
    fn num_333() -> Self;
    fn num_334() -> Self;
    fn num_335() -> Self;
    fn num_336() -> Self;
    fn num_337() -> Self;
    fn num_338() -> Self;
    fn num_339() -> Self;
    fn num_340() -> Self;
    fn num_341() -> Self;
    fn num_342() -> Self;
    fn num_343() -> Self;
    fn num_344() -> Self;
    fn num_345() -> Self;
    fn num_346() -> Self;
    fn num_347() -> Self;
    fn num_348() -> Self;
    fn num_349() -> Self;
    fn num_350() -> Self;
    fn num_351() -> Self;
    fn num_352() -> Self;
    fn num_353() -> Self;
    fn num_354() -> Self;
    fn num_355() -> Self;
    fn num_356() -> Self;
    fn num_357() -> Self;
    fn num_358() -> Self;
    fn num_359() -> Self;
    fn num_360() -> Self;
    fn num_361() -> Self;
    fn num_362() -> Self;
    fn num_363() -> Self;
    fn num_364() -> Self;
    fn num_365() -> Self;
    fn num_366() -> Self;
    fn num_367() -> Self;
    fn num_368() -> Self;
    fn num_369() -> Self;
    fn num_370() -> Self;
    fn num_371() -> Self;
    fn num_372() -> Self;
    fn num_373() -> Self;
    fn num_374() -> Self;
    fn num_375() -> Self;
    fn num_376() -> Self;
    fn num_377() -> Self;
    fn num_378() -> Self;
    fn num_379() -> Self;
    fn num_380() -> Self;
    fn num_381() -> Self;
    fn num_382() -> Self;
    fn num_383() -> Self;
    fn num_384() -> Self;
    fn num_385() -> Self;
    fn num_386() -> Self;
    fn num_387() -> Self;
    fn num_388() -> Self;
    fn num_389() -> Self;
    fn num_390() -> Self;
    fn num_391() -> Self;
    fn num_392() -> Self;
    fn num_393() -> Self;
    fn num_394() -> Self;
    fn num_395() -> Self;
    fn num_396() -> Self;
    fn num_397() -> Self;
    fn num_398() -> Self;
    fn num_399() -> Self;
    fn num_400() -> Self;
    fn num_401() -> Self;
    fn num_402() -> Self;
    fn num_403() -> Self;
    fn num_404() -> Self;
    fn num_405() -> Self;
    fn num_406() -> Self;
    fn num_407() -> Self;
    fn num_408() -> Self;
    fn num_409() -> Self;
    fn num_410() -> Self;
    fn num_411() -> Self;
    fn num_412() -> Self;
    fn num_413() -> Self;
    fn num_414() -> Self;
    fn num_415() -> Self;
    fn num_416() -> Self;
    fn num_417() -> Self;
    fn num_418() -> Self;
    fn num_419() -> Self;
    fn num_420() -> Self;
    fn num_421() -> Self;
    fn num_422() -> Self;
    fn num_423() -> Self;
    fn num_424() -> Self;
    fn num_425() -> Self;
    fn num_426() -> Self;
    fn num_427() -> Self;
    fn num_428() -> Self;
    fn num_429() -> Self;
    fn num_430() -> Self;
    fn num_431() -> Self;
    fn num_432() -> Self;
    fn num_433() -> Self;
    fn num_434() -> Self;
    fn num_435() -> Self;
    fn num_436() -> Self;
    fn num_437() -> Self;
    fn num_438() -> Self;
    fn num_439() -> Self;
    fn num_440() -> Self;
    fn num_441() -> Self;
    fn num_442() -> Self;
    fn num_443() -> Self;
    fn num_444() -> Self;
    fn num_445() -> Self;
    fn num_446() -> Self;
    fn num_447() -> Self;
    fn num_448() -> Self;
    fn num_449() -> Self;
    fn num_450() -> Self;
    fn num_451() -> Self;
    fn num_452() -> Self;
    fn num_453() -> Self;
    fn num_454() -> Self;
    fn num_455() -> Self;
    fn num_456() -> Self;
    fn num_457() -> Self;
    fn num_458() -> Self;
    fn num_459() -> Self;
    fn num_460() -> Self;
    fn num_461() -> Self;
    fn num_462() -> Self;
    fn num_463() -> Self;
    fn num_464() -> Self;
    fn num_465() -> Self;
    fn num_466() -> Self;
    fn num_467() -> Self;
    fn num_468() -> Self;
    fn num_469() -> Self;
    fn num_470() -> Self;
    fn num_471() -> Self;
    fn num_472() -> Self;
    fn num_473() -> Self;
    fn num_474() -> Self;
    fn num_475() -> Self;
    fn num_476() -> Self;
    fn num_477() -> Self;
    fn num_478() -> Self;
    fn num_479() -> Self;
    fn num_480() -> Self;
    fn num_481() -> Self;
    fn num_482() -> Self;
    fn num_483() -> Self;
    fn num_484() -> Self;
    fn num_485() -> Self;
    fn num_486() -> Self;
    fn num_487() -> Self;
    fn num_488() -> Self;
    fn num_489() -> Self;
    fn num_490() -> Self;
    fn num_491() -> Self;
    fn num_492() -> Self;
    fn num_493() -> Self;
    fn num_494() -> Self;
    fn num_495() -> Self;
    fn num_496() -> Self;
    fn num_497() -> Self;
    fn num_498() -> Self;
    fn num_499() -> Self;
    fn num_500() -> Self;
    fn num_501() -> Self;
    fn num_502() -> Self;
    fn num_503() -> Self;
    fn num_504() -> Self;
    fn num_505() -> Self;
    fn num_506() -> Self;
    fn num_507() -> Self;
    fn num_508() -> Self;
    fn num_509() -> Self;
    fn num_510() -> Self;
    fn num_511() -> Self;
}
const impl<T: SupportsRange512 + [const] One + [const] Zero + [const] core::ops::Add<Output = T> + [const] Numbers256 + [const] Numbers128> Numbers512
    for T
{
    fn num_256() -> Self {
        Self::num_255() + Self::num_1()
    }
    fn num_257() -> Self {
        Self::num_256() + Self::num_1()
    }
    fn num_258() -> Self {
        Self::num_257() + Self::num_1()
    }
    fn num_259() -> Self {
        Self::num_258() + Self::num_1()
    }
    fn num_260() -> Self {
        Self::num_259() + Self::num_1()
    }
    fn num_261() -> Self {
        Self::num_260() + Self::num_1()
    }
    fn num_262() -> Self {
        Self::num_261() + Self::num_1()
    }
    fn num_263() -> Self {
        Self::num_262() + Self::num_1()
    }
    fn num_264() -> Self {
        Self::num_263() + Self::num_1()
    }
    fn num_265() -> Self {
        Self::num_264() + Self::num_1()
    }
    fn num_266() -> Self {
        Self::num_265() + Self::num_1()
    }
    fn num_267() -> Self {
        Self::num_266() + Self::num_1()
    }
    fn num_268() -> Self {
        Self::num_267() + Self::num_1()
    }
    fn num_269() -> Self {
        Self::num_268() + Self::num_1()
    }
    fn num_270() -> Self {
        Self::num_269() + Self::num_1()
    }
    fn num_271() -> Self {
        Self::num_270() + Self::num_1()
    }
    fn num_272() -> Self {
        Self::num_271() + Self::num_1()
    }
    fn num_273() -> Self {
        Self::num_272() + Self::num_1()
    }
    fn num_274() -> Self {
        Self::num_273() + Self::num_1()
    }
    fn num_275() -> Self {
        Self::num_274() + Self::num_1()
    }
    fn num_276() -> Self {
        Self::num_275() + Self::num_1()
    }
    fn num_277() -> Self {
        Self::num_276() + Self::num_1()
    }
    fn num_278() -> Self {
        Self::num_277() + Self::num_1()
    }
    fn num_279() -> Self {
        Self::num_278() + Self::num_1()
    }
    fn num_280() -> Self {
        Self::num_279() + Self::num_1()
    }
    fn num_281() -> Self {
        Self::num_280() + Self::num_1()
    }
    fn num_282() -> Self {
        Self::num_281() + Self::num_1()
    }
    fn num_283() -> Self {
        Self::num_282() + Self::num_1()
    }
    fn num_284() -> Self {
        Self::num_283() + Self::num_1()
    }
    fn num_285() -> Self {
        Self::num_284() + Self::num_1()
    }
    fn num_286() -> Self {
        Self::num_285() + Self::num_1()
    }
    fn num_287() -> Self {
        Self::num_286() + Self::num_1()
    }
    fn num_288() -> Self {
        Self::num_287() + Self::num_1()
    }
    fn num_289() -> Self {
        Self::num_288() + Self::num_1()
    }
    fn num_290() -> Self {
        Self::num_289() + Self::num_1()
    }
    fn num_291() -> Self {
        Self::num_290() + Self::num_1()
    }
    fn num_292() -> Self {
        Self::num_291() + Self::num_1()
    }
    fn num_293() -> Self {
        Self::num_292() + Self::num_1()
    }
    fn num_294() -> Self {
        Self::num_293() + Self::num_1()
    }
    fn num_295() -> Self {
        Self::num_294() + Self::num_1()
    }
    fn num_296() -> Self {
        Self::num_295() + Self::num_1()
    }
    fn num_297() -> Self {
        Self::num_296() + Self::num_1()
    }
    fn num_298() -> Self {
        Self::num_297() + Self::num_1()
    }
    fn num_299() -> Self {
        Self::num_298() + Self::num_1()
    }
    fn num_300() -> Self {
        Self::num_299() + Self::num_1()
    }
    fn num_301() -> Self {
        Self::num_300() + Self::num_1()
    }
    fn num_302() -> Self {
        Self::num_301() + Self::num_1()
    }
    fn num_303() -> Self {
        Self::num_302() + Self::num_1()
    }
    fn num_304() -> Self {
        Self::num_303() + Self::num_1()
    }
    fn num_305() -> Self {
        Self::num_304() + Self::num_1()
    }
    fn num_306() -> Self {
        Self::num_305() + Self::num_1()
    }
    fn num_307() -> Self {
        Self::num_306() + Self::num_1()
    }
    fn num_308() -> Self {
        Self::num_307() + Self::num_1()
    }
    fn num_309() -> Self {
        Self::num_308() + Self::num_1()
    }
    fn num_310() -> Self {
        Self::num_309() + Self::num_1()
    }
    fn num_311() -> Self {
        Self::num_310() + Self::num_1()
    }
    fn num_312() -> Self {
        Self::num_311() + Self::num_1()
    }
    fn num_313() -> Self {
        Self::num_312() + Self::num_1()
    }
    fn num_314() -> Self {
        Self::num_313() + Self::num_1()
    }
    fn num_315() -> Self {
        Self::num_314() + Self::num_1()
    }
    fn num_316() -> Self {
        Self::num_315() + Self::num_1()
    }
    fn num_317() -> Self {
        Self::num_316() + Self::num_1()
    }
    fn num_318() -> Self {
        Self::num_317() + Self::num_1()
    }
    fn num_319() -> Self {
        Self::num_318() + Self::num_1()
    }
    fn num_320() -> Self {
        Self::num_319() + Self::num_1()
    }
    fn num_321() -> Self {
        Self::num_320() + Self::num_1()
    }
    fn num_322() -> Self {
        Self::num_321() + Self::num_1()
    }
    fn num_323() -> Self {
        Self::num_322() + Self::num_1()
    }
    fn num_324() -> Self {
        Self::num_323() + Self::num_1()
    }
    fn num_325() -> Self {
        Self::num_324() + Self::num_1()
    }
    fn num_326() -> Self {
        Self::num_325() + Self::num_1()
    }
    fn num_327() -> Self {
        Self::num_326() + Self::num_1()
    }
    fn num_328() -> Self {
        Self::num_327() + Self::num_1()
    }
    fn num_329() -> Self {
        Self::num_328() + Self::num_1()
    }
    fn num_330() -> Self {
        Self::num_329() + Self::num_1()
    }
    fn num_331() -> Self {
        Self::num_330() + Self::num_1()
    }
    fn num_332() -> Self {
        Self::num_331() + Self::num_1()
    }
    fn num_333() -> Self {
        Self::num_332() + Self::num_1()
    }
    fn num_334() -> Self {
        Self::num_333() + Self::num_1()
    }
    fn num_335() -> Self {
        Self::num_334() + Self::num_1()
    }
    fn num_336() -> Self {
        Self::num_335() + Self::num_1()
    }
    fn num_337() -> Self {
        Self::num_336() + Self::num_1()
    }
    fn num_338() -> Self {
        Self::num_337() + Self::num_1()
    }
    fn num_339() -> Self {
        Self::num_338() + Self::num_1()
    }
    fn num_340() -> Self {
        Self::num_339() + Self::num_1()
    }
    fn num_341() -> Self {
        Self::num_340() + Self::num_1()
    }
    fn num_342() -> Self {
        Self::num_341() + Self::num_1()
    }
    fn num_343() -> Self {
        Self::num_342() + Self::num_1()
    }
    fn num_344() -> Self {
        Self::num_343() + Self::num_1()
    }
    fn num_345() -> Self {
        Self::num_344() + Self::num_1()
    }
    fn num_346() -> Self {
        Self::num_345() + Self::num_1()
    }
    fn num_347() -> Self {
        Self::num_346() + Self::num_1()
    }
    fn num_348() -> Self {
        Self::num_347() + Self::num_1()
    }
    fn num_349() -> Self {
        Self::num_348() + Self::num_1()
    }
    fn num_350() -> Self {
        Self::num_349() + Self::num_1()
    }
    fn num_351() -> Self {
        Self::num_350() + Self::num_1()
    }
    fn num_352() -> Self {
        Self::num_351() + Self::num_1()
    }
    fn num_353() -> Self {
        Self::num_352() + Self::num_1()
    }
    fn num_354() -> Self {
        Self::num_353() + Self::num_1()
    }
    fn num_355() -> Self {
        Self::num_354() + Self::num_1()
    }
    fn num_356() -> Self {
        Self::num_355() + Self::num_1()
    }
    fn num_357() -> Self {
        Self::num_356() + Self::num_1()
    }
    fn num_358() -> Self {
        Self::num_357() + Self::num_1()
    }
    fn num_359() -> Self {
        Self::num_358() + Self::num_1()
    }
    fn num_360() -> Self {
        Self::num_359() + Self::num_1()
    }
    fn num_361() -> Self {
        Self::num_360() + Self::num_1()
    }
    fn num_362() -> Self {
        Self::num_361() + Self::num_1()
    }
    fn num_363() -> Self {
        Self::num_362() + Self::num_1()
    }
    fn num_364() -> Self {
        Self::num_363() + Self::num_1()
    }
    fn num_365() -> Self {
        Self::num_364() + Self::num_1()
    }
    fn num_366() -> Self {
        Self::num_365() + Self::num_1()
    }
    fn num_367() -> Self {
        Self::num_366() + Self::num_1()
    }
    fn num_368() -> Self {
        Self::num_367() + Self::num_1()
    }
    fn num_369() -> Self {
        Self::num_368() + Self::num_1()
    }
    fn num_370() -> Self {
        Self::num_369() + Self::num_1()
    }
    fn num_371() -> Self {
        Self::num_370() + Self::num_1()
    }
    fn num_372() -> Self {
        Self::num_371() + Self::num_1()
    }
    fn num_373() -> Self {
        Self::num_372() + Self::num_1()
    }
    fn num_374() -> Self {
        Self::num_373() + Self::num_1()
    }
    fn num_375() -> Self {
        Self::num_374() + Self::num_1()
    }
    fn num_376() -> Self {
        Self::num_375() + Self::num_1()
    }
    fn num_377() -> Self {
        Self::num_376() + Self::num_1()
    }
    fn num_378() -> Self {
        Self::num_377() + Self::num_1()
    }
    fn num_379() -> Self {
        Self::num_378() + Self::num_1()
    }
    fn num_380() -> Self {
        Self::num_379() + Self::num_1()
    }
    fn num_381() -> Self {
        Self::num_380() + Self::num_1()
    }
    fn num_382() -> Self {
        Self::num_381() + Self::num_1()
    }
    fn num_383() -> Self {
        Self::num_382() + Self::num_1()
    }
    fn num_384() -> Self {
        Self::num_383() + Self::num_1()
    }
    fn num_385() -> Self {
        Self::num_384() + Self::num_1()
    }
    fn num_386() -> Self {
        Self::num_385() + Self::num_1()
    }
    fn num_387() -> Self {
        Self::num_386() + Self::num_1()
    }
    fn num_388() -> Self {
        Self::num_387() + Self::num_1()
    }
    fn num_389() -> Self {
        Self::num_388() + Self::num_1()
    }
    fn num_390() -> Self {
        Self::num_389() + Self::num_1()
    }
    fn num_391() -> Self {
        Self::num_390() + Self::num_1()
    }
    fn num_392() -> Self {
        Self::num_391() + Self::num_1()
    }
    fn num_393() -> Self {
        Self::num_392() + Self::num_1()
    }
    fn num_394() -> Self {
        Self::num_393() + Self::num_1()
    }
    fn num_395() -> Self {
        Self::num_394() + Self::num_1()
    }
    fn num_396() -> Self {
        Self::num_395() + Self::num_1()
    }
    fn num_397() -> Self {
        Self::num_396() + Self::num_1()
    }
    fn num_398() -> Self {
        Self::num_397() + Self::num_1()
    }
    fn num_399() -> Self {
        Self::num_398() + Self::num_1()
    }
    fn num_400() -> Self {
        Self::num_399() + Self::num_1()
    }
    fn num_401() -> Self {
        Self::num_400() + Self::num_1()
    }
    fn num_402() -> Self {
        Self::num_401() + Self::num_1()
    }
    fn num_403() -> Self {
        Self::num_402() + Self::num_1()
    }
    fn num_404() -> Self {
        Self::num_403() + Self::num_1()
    }
    fn num_405() -> Self {
        Self::num_404() + Self::num_1()
    }
    fn num_406() -> Self {
        Self::num_405() + Self::num_1()
    }
    fn num_407() -> Self {
        Self::num_406() + Self::num_1()
    }
    fn num_408() -> Self {
        Self::num_407() + Self::num_1()
    }
    fn num_409() -> Self {
        Self::num_408() + Self::num_1()
    }
    fn num_410() -> Self {
        Self::num_409() + Self::num_1()
    }
    fn num_411() -> Self {
        Self::num_410() + Self::num_1()
    }
    fn num_412() -> Self {
        Self::num_411() + Self::num_1()
    }
    fn num_413() -> Self {
        Self::num_412() + Self::num_1()
    }
    fn num_414() -> Self {
        Self::num_413() + Self::num_1()
    }
    fn num_415() -> Self {
        Self::num_414() + Self::num_1()
    }
    fn num_416() -> Self {
        Self::num_415() + Self::num_1()
    }
    fn num_417() -> Self {
        Self::num_416() + Self::num_1()
    }
    fn num_418() -> Self {
        Self::num_417() + Self::num_1()
    }
    fn num_419() -> Self {
        Self::num_418() + Self::num_1()
    }
    fn num_420() -> Self {
        Self::num_419() + Self::num_1()
    }
    fn num_421() -> Self {
        Self::num_420() + Self::num_1()
    }
    fn num_422() -> Self {
        Self::num_421() + Self::num_1()
    }
    fn num_423() -> Self {
        Self::num_422() + Self::num_1()
    }
    fn num_424() -> Self {
        Self::num_423() + Self::num_1()
    }
    fn num_425() -> Self {
        Self::num_424() + Self::num_1()
    }
    fn num_426() -> Self {
        Self::num_425() + Self::num_1()
    }
    fn num_427() -> Self {
        Self::num_426() + Self::num_1()
    }
    fn num_428() -> Self {
        Self::num_427() + Self::num_1()
    }
    fn num_429() -> Self {
        Self::num_428() + Self::num_1()
    }
    fn num_430() -> Self {
        Self::num_429() + Self::num_1()
    }
    fn num_431() -> Self {
        Self::num_430() + Self::num_1()
    }
    fn num_432() -> Self {
        Self::num_431() + Self::num_1()
    }
    fn num_433() -> Self {
        Self::num_432() + Self::num_1()
    }
    fn num_434() -> Self {
        Self::num_433() + Self::num_1()
    }
    fn num_435() -> Self {
        Self::num_434() + Self::num_1()
    }
    fn num_436() -> Self {
        Self::num_435() + Self::num_1()
    }
    fn num_437() -> Self {
        Self::num_436() + Self::num_1()
    }
    fn num_438() -> Self {
        Self::num_437() + Self::num_1()
    }
    fn num_439() -> Self {
        Self::num_438() + Self::num_1()
    }
    fn num_440() -> Self {
        Self::num_439() + Self::num_1()
    }
    fn num_441() -> Self {
        Self::num_440() + Self::num_1()
    }
    fn num_442() -> Self {
        Self::num_441() + Self::num_1()
    }
    fn num_443() -> Self {
        Self::num_442() + Self::num_1()
    }
    fn num_444() -> Self {
        Self::num_443() + Self::num_1()
    }
    fn num_445() -> Self {
        Self::num_444() + Self::num_1()
    }
    fn num_446() -> Self {
        Self::num_445() + Self::num_1()
    }
    fn num_447() -> Self {
        Self::num_446() + Self::num_1()
    }
    fn num_448() -> Self {
        Self::num_447() + Self::num_1()
    }
    fn num_449() -> Self {
        Self::num_448() + Self::num_1()
    }
    fn num_450() -> Self {
        Self::num_449() + Self::num_1()
    }
    fn num_451() -> Self {
        Self::num_450() + Self::num_1()
    }
    fn num_452() -> Self {
        Self::num_451() + Self::num_1()
    }
    fn num_453() -> Self {
        Self::num_452() + Self::num_1()
    }
    fn num_454() -> Self {
        Self::num_453() + Self::num_1()
    }
    fn num_455() -> Self {
        Self::num_454() + Self::num_1()
    }
    fn num_456() -> Self {
        Self::num_455() + Self::num_1()
    }
    fn num_457() -> Self {
        Self::num_456() + Self::num_1()
    }
    fn num_458() -> Self {
        Self::num_457() + Self::num_1()
    }
    fn num_459() -> Self {
        Self::num_458() + Self::num_1()
    }
    fn num_460() -> Self {
        Self::num_459() + Self::num_1()
    }
    fn num_461() -> Self {
        Self::num_460() + Self::num_1()
    }
    fn num_462() -> Self {
        Self::num_461() + Self::num_1()
    }
    fn num_463() -> Self {
        Self::num_462() + Self::num_1()
    }
    fn num_464() -> Self {
        Self::num_463() + Self::num_1()
    }
    fn num_465() -> Self {
        Self::num_464() + Self::num_1()
    }
    fn num_466() -> Self {
        Self::num_465() + Self::num_1()
    }
    fn num_467() -> Self {
        Self::num_466() + Self::num_1()
    }
    fn num_468() -> Self {
        Self::num_467() + Self::num_1()
    }
    fn num_469() -> Self {
        Self::num_468() + Self::num_1()
    }
    fn num_470() -> Self {
        Self::num_469() + Self::num_1()
    }
    fn num_471() -> Self {
        Self::num_470() + Self::num_1()
    }
    fn num_472() -> Self {
        Self::num_471() + Self::num_1()
    }
    fn num_473() -> Self {
        Self::num_472() + Self::num_1()
    }
    fn num_474() -> Self {
        Self::num_473() + Self::num_1()
    }
    fn num_475() -> Self {
        Self::num_474() + Self::num_1()
    }
    fn num_476() -> Self {
        Self::num_475() + Self::num_1()
    }
    fn num_477() -> Self {
        Self::num_476() + Self::num_1()
    }
    fn num_478() -> Self {
        Self::num_477() + Self::num_1()
    }
    fn num_479() -> Self {
        Self::num_478() + Self::num_1()
    }
    fn num_480() -> Self {
        Self::num_479() + Self::num_1()
    }
    fn num_481() -> Self {
        Self::num_480() + Self::num_1()
    }
    fn num_482() -> Self {
        Self::num_481() + Self::num_1()
    }
    fn num_483() -> Self {
        Self::num_482() + Self::num_1()
    }
    fn num_484() -> Self {
        Self::num_483() + Self::num_1()
    }
    fn num_485() -> Self {
        Self::num_484() + Self::num_1()
    }
    fn num_486() -> Self {
        Self::num_485() + Self::num_1()
    }
    fn num_487() -> Self {
        Self::num_486() + Self::num_1()
    }
    fn num_488() -> Self {
        Self::num_487() + Self::num_1()
    }
    fn num_489() -> Self {
        Self::num_488() + Self::num_1()
    }
    fn num_490() -> Self {
        Self::num_489() + Self::num_1()
    }
    fn num_491() -> Self {
        Self::num_490() + Self::num_1()
    }
    fn num_492() -> Self {
        Self::num_491() + Self::num_1()
    }
    fn num_493() -> Self {
        Self::num_492() + Self::num_1()
    }
    fn num_494() -> Self {
        Self::num_493() + Self::num_1()
    }
    fn num_495() -> Self {
        Self::num_494() + Self::num_1()
    }
    fn num_496() -> Self {
        Self::num_495() + Self::num_1()
    }
    fn num_497() -> Self {
        Self::num_496() + Self::num_1()
    }
    fn num_498() -> Self {
        Self::num_497() + Self::num_1()
    }
    fn num_499() -> Self {
        Self::num_498() + Self::num_1()
    }
    fn num_500() -> Self {
        Self::num_499() + Self::num_1()
    }
    fn num_501() -> Self {
        Self::num_500() + Self::num_1()
    }
    fn num_502() -> Self {
        Self::num_501() + Self::num_1()
    }
    fn num_503() -> Self {
        Self::num_502() + Self::num_1()
    }
    fn num_504() -> Self {
        Self::num_503() + Self::num_1()
    }
    fn num_505() -> Self {
        Self::num_504() + Self::num_1()
    }
    fn num_506() -> Self {
        Self::num_505() + Self::num_1()
    }
    fn num_507() -> Self {
        Self::num_506() + Self::num_1()
    }
    fn num_508() -> Self {
        Self::num_507() + Self::num_1()
    }
    fn num_509() -> Self {
        Self::num_508() + Self::num_1()
    }
    fn num_510() -> Self {
        Self::num_509() + Self::num_1()
    }
    fn num_511() -> Self {
        Self::num_510() + Self::num_1()
    }
}
