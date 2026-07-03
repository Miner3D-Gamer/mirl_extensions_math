use crate::Numbers128;
use crate::Numbers512;
use mirl_extensions_core::{One, SupportsRange1024, Zero};

/// Get any number between 512 and 1023
///
/// An extended version of the [`Numbers512`] trait covering all numbers from 512 to 1023
#[allow(missing_docs)]
pub const trait Numbers1024: SupportsRange1024 + Numbers512 {
    fn num_512() -> Self;
    fn num_513() -> Self;
    fn num_514() -> Self;
    fn num_515() -> Self;
    fn num_516() -> Self;
    fn num_517() -> Self;
    fn num_518() -> Self;
    fn num_519() -> Self;
    fn num_520() -> Self;
    fn num_521() -> Self;
    fn num_522() -> Self;
    fn num_523() -> Self;
    fn num_524() -> Self;
    fn num_525() -> Self;
    fn num_526() -> Self;
    fn num_527() -> Self;
    fn num_528() -> Self;
    fn num_529() -> Self;
    fn num_530() -> Self;
    fn num_531() -> Self;
    fn num_532() -> Self;
    fn num_533() -> Self;
    fn num_534() -> Self;
    fn num_535() -> Self;
    fn num_536() -> Self;
    fn num_537() -> Self;
    fn num_538() -> Self;
    fn num_539() -> Self;
    fn num_540() -> Self;
    fn num_541() -> Self;
    fn num_542() -> Self;
    fn num_543() -> Self;
    fn num_544() -> Self;
    fn num_545() -> Self;
    fn num_546() -> Self;
    fn num_547() -> Self;
    fn num_548() -> Self;
    fn num_549() -> Self;
    fn num_550() -> Self;
    fn num_551() -> Self;
    fn num_552() -> Self;
    fn num_553() -> Self;
    fn num_554() -> Self;
    fn num_555() -> Self;
    fn num_556() -> Self;
    fn num_557() -> Self;
    fn num_558() -> Self;
    fn num_559() -> Self;
    fn num_560() -> Self;
    fn num_561() -> Self;
    fn num_562() -> Self;
    fn num_563() -> Self;
    fn num_564() -> Self;
    fn num_565() -> Self;
    fn num_566() -> Self;
    fn num_567() -> Self;
    fn num_568() -> Self;
    fn num_569() -> Self;
    fn num_570() -> Self;
    fn num_571() -> Self;
    fn num_572() -> Self;
    fn num_573() -> Self;
    fn num_574() -> Self;
    fn num_575() -> Self;
    fn num_576() -> Self;
    fn num_577() -> Self;
    fn num_578() -> Self;
    fn num_579() -> Self;
    fn num_580() -> Self;
    fn num_581() -> Self;
    fn num_582() -> Self;
    fn num_583() -> Self;
    fn num_584() -> Self;
    fn num_585() -> Self;
    fn num_586() -> Self;
    fn num_587() -> Self;
    fn num_588() -> Self;
    fn num_589() -> Self;
    fn num_590() -> Self;
    fn num_591() -> Self;
    fn num_592() -> Self;
    fn num_593() -> Self;
    fn num_594() -> Self;
    fn num_595() -> Self;
    fn num_596() -> Self;
    fn num_597() -> Self;
    fn num_598() -> Self;
    fn num_599() -> Self;
    fn num_600() -> Self;
    fn num_601() -> Self;
    fn num_602() -> Self;
    fn num_603() -> Self;
    fn num_604() -> Self;
    fn num_605() -> Self;
    fn num_606() -> Self;
    fn num_607() -> Self;
    fn num_608() -> Self;
    fn num_609() -> Self;
    fn num_610() -> Self;
    fn num_611() -> Self;
    fn num_612() -> Self;
    fn num_613() -> Self;
    fn num_614() -> Self;
    fn num_615() -> Self;
    fn num_616() -> Self;
    fn num_617() -> Self;
    fn num_618() -> Self;
    fn num_619() -> Self;
    fn num_620() -> Self;
    fn num_621() -> Self;
    fn num_622() -> Self;
    fn num_623() -> Self;
    fn num_624() -> Self;
    fn num_625() -> Self;
    fn num_626() -> Self;
    fn num_627() -> Self;
    fn num_628() -> Self;
    fn num_629() -> Self;
    fn num_630() -> Self;
    fn num_631() -> Self;
    fn num_632() -> Self;
    fn num_633() -> Self;
    fn num_634() -> Self;
    fn num_635() -> Self;
    fn num_636() -> Self;
    fn num_637() -> Self;
    fn num_638() -> Self;
    fn num_639() -> Self;
    fn num_640() -> Self;
    fn num_641() -> Self;
    fn num_642() -> Self;
    fn num_643() -> Self;
    fn num_644() -> Self;
    fn num_645() -> Self;
    fn num_646() -> Self;
    fn num_647() -> Self;
    fn num_648() -> Self;
    fn num_649() -> Self;
    fn num_650() -> Self;
    fn num_651() -> Self;
    fn num_652() -> Self;
    fn num_653() -> Self;
    fn num_654() -> Self;
    fn num_655() -> Self;
    fn num_656() -> Self;
    fn num_657() -> Self;
    fn num_658() -> Self;
    fn num_659() -> Self;
    fn num_660() -> Self;
    fn num_661() -> Self;
    fn num_662() -> Self;
    fn num_663() -> Self;
    fn num_664() -> Self;
    fn num_665() -> Self;
    fn num_666() -> Self;
    fn num_667() -> Self;
    fn num_668() -> Self;
    fn num_669() -> Self;
    fn num_670() -> Self;
    fn num_671() -> Self;
    fn num_672() -> Self;
    fn num_673() -> Self;
    fn num_674() -> Self;
    fn num_675() -> Self;
    fn num_676() -> Self;
    fn num_677() -> Self;
    fn num_678() -> Self;
    fn num_679() -> Self;
    fn num_680() -> Self;
    fn num_681() -> Self;
    fn num_682() -> Self;
    fn num_683() -> Self;
    fn num_684() -> Self;
    fn num_685() -> Self;
    fn num_686() -> Self;
    fn num_687() -> Self;
    fn num_688() -> Self;
    fn num_689() -> Self;
    fn num_690() -> Self;
    fn num_691() -> Self;
    fn num_692() -> Self;
    fn num_693() -> Self;
    fn num_694() -> Self;
    fn num_695() -> Self;
    fn num_696() -> Self;
    fn num_697() -> Self;
    fn num_698() -> Self;
    fn num_699() -> Self;
    fn num_700() -> Self;
    fn num_701() -> Self;
    fn num_702() -> Self;
    fn num_703() -> Self;
    fn num_704() -> Self;
    fn num_705() -> Self;
    fn num_706() -> Self;
    fn num_707() -> Self;
    fn num_708() -> Self;
    fn num_709() -> Self;
    fn num_710() -> Self;
    fn num_711() -> Self;
    fn num_712() -> Self;
    fn num_713() -> Self;
    fn num_714() -> Self;
    fn num_715() -> Self;
    fn num_716() -> Self;
    fn num_717() -> Self;
    fn num_718() -> Self;
    fn num_719() -> Self;
    fn num_720() -> Self;
    fn num_721() -> Self;
    fn num_722() -> Self;
    fn num_723() -> Self;
    fn num_724() -> Self;
    fn num_725() -> Self;
    fn num_726() -> Self;
    fn num_727() -> Self;
    fn num_728() -> Self;
    fn num_729() -> Self;
    fn num_730() -> Self;
    fn num_731() -> Self;
    fn num_732() -> Self;
    fn num_733() -> Self;
    fn num_734() -> Self;
    fn num_735() -> Self;
    fn num_736() -> Self;
    fn num_737() -> Self;
    fn num_738() -> Self;
    fn num_739() -> Self;
    fn num_740() -> Self;
    fn num_741() -> Self;
    fn num_742() -> Self;
    fn num_743() -> Self;
    fn num_744() -> Self;
    fn num_745() -> Self;
    fn num_746() -> Self;
    fn num_747() -> Self;
    fn num_748() -> Self;
    fn num_749() -> Self;
    fn num_750() -> Self;
    fn num_751() -> Self;
    fn num_752() -> Self;
    fn num_753() -> Self;
    fn num_754() -> Self;
    fn num_755() -> Self;
    fn num_756() -> Self;
    fn num_757() -> Self;
    fn num_758() -> Self;
    fn num_759() -> Self;
    fn num_760() -> Self;
    fn num_761() -> Self;
    fn num_762() -> Self;
    fn num_763() -> Self;
    fn num_764() -> Self;
    fn num_765() -> Self;
    fn num_766() -> Self;
    fn num_767() -> Self;
    fn num_768() -> Self;
    fn num_769() -> Self;
    fn num_770() -> Self;
    fn num_771() -> Self;
    fn num_772() -> Self;
    fn num_773() -> Self;
    fn num_774() -> Self;
    fn num_775() -> Self;
    fn num_776() -> Self;
    fn num_777() -> Self;
    fn num_778() -> Self;
    fn num_779() -> Self;
    fn num_780() -> Self;
    fn num_781() -> Self;
    fn num_782() -> Self;
    fn num_783() -> Self;
    fn num_784() -> Self;
    fn num_785() -> Self;
    fn num_786() -> Self;
    fn num_787() -> Self;
    fn num_788() -> Self;
    fn num_789() -> Self;
    fn num_790() -> Self;
    fn num_791() -> Self;
    fn num_792() -> Self;
    fn num_793() -> Self;
    fn num_794() -> Self;
    fn num_795() -> Self;
    fn num_796() -> Self;
    fn num_797() -> Self;
    fn num_798() -> Self;
    fn num_799() -> Self;
    fn num_800() -> Self;
    fn num_801() -> Self;
    fn num_802() -> Self;
    fn num_803() -> Self;
    fn num_804() -> Self;
    fn num_805() -> Self;
    fn num_806() -> Self;
    fn num_807() -> Self;
    fn num_808() -> Self;
    fn num_809() -> Self;
    fn num_810() -> Self;
    fn num_811() -> Self;
    fn num_812() -> Self;
    fn num_813() -> Self;
    fn num_814() -> Self;
    fn num_815() -> Self;
    fn num_816() -> Self;
    fn num_817() -> Self;
    fn num_818() -> Self;
    fn num_819() -> Self;
    fn num_820() -> Self;
    fn num_821() -> Self;
    fn num_822() -> Self;
    fn num_823() -> Self;
    fn num_824() -> Self;
    fn num_825() -> Self;
    fn num_826() -> Self;
    fn num_827() -> Self;
    fn num_828() -> Self;
    fn num_829() -> Self;
    fn num_830() -> Self;
    fn num_831() -> Self;
    fn num_832() -> Self;
    fn num_833() -> Self;
    fn num_834() -> Self;
    fn num_835() -> Self;
    fn num_836() -> Self;
    fn num_837() -> Self;
    fn num_838() -> Self;
    fn num_839() -> Self;
    fn num_840() -> Self;
    fn num_841() -> Self;
    fn num_842() -> Self;
    fn num_843() -> Self;
    fn num_844() -> Self;
    fn num_845() -> Self;
    fn num_846() -> Self;
    fn num_847() -> Self;
    fn num_848() -> Self;
    fn num_849() -> Self;
    fn num_850() -> Self;
    fn num_851() -> Self;
    fn num_852() -> Self;
    fn num_853() -> Self;
    fn num_854() -> Self;
    fn num_855() -> Self;
    fn num_856() -> Self;
    fn num_857() -> Self;
    fn num_858() -> Self;
    fn num_859() -> Self;
    fn num_860() -> Self;
    fn num_861() -> Self;
    fn num_862() -> Self;
    fn num_863() -> Self;
    fn num_864() -> Self;
    fn num_865() -> Self;
    fn num_866() -> Self;
    fn num_867() -> Self;
    fn num_868() -> Self;
    fn num_869() -> Self;
    fn num_870() -> Self;
    fn num_871() -> Self;
    fn num_872() -> Self;
    fn num_873() -> Self;
    fn num_874() -> Self;
    fn num_875() -> Self;
    fn num_876() -> Self;
    fn num_877() -> Self;
    fn num_878() -> Self;
    fn num_879() -> Self;
    fn num_880() -> Self;
    fn num_881() -> Self;
    fn num_882() -> Self;
    fn num_883() -> Self;
    fn num_884() -> Self;
    fn num_885() -> Self;
    fn num_886() -> Self;
    fn num_887() -> Self;
    fn num_888() -> Self;
    fn num_889() -> Self;
    fn num_890() -> Self;
    fn num_891() -> Self;
    fn num_892() -> Self;
    fn num_893() -> Self;
    fn num_894() -> Self;
    fn num_895() -> Self;
    fn num_896() -> Self;
    fn num_897() -> Self;
    fn num_898() -> Self;
    fn num_899() -> Self;
    fn num_900() -> Self;
    fn num_901() -> Self;
    fn num_902() -> Self;
    fn num_903() -> Self;
    fn num_904() -> Self;
    fn num_905() -> Self;
    fn num_906() -> Self;
    fn num_907() -> Self;
    fn num_908() -> Self;
    fn num_909() -> Self;
    fn num_910() -> Self;
    fn num_911() -> Self;
    fn num_912() -> Self;
    fn num_913() -> Self;
    fn num_914() -> Self;
    fn num_915() -> Self;
    fn num_916() -> Self;
    fn num_917() -> Self;
    fn num_918() -> Self;
    fn num_919() -> Self;
    fn num_920() -> Self;
    fn num_921() -> Self;
    fn num_922() -> Self;
    fn num_923() -> Self;
    fn num_924() -> Self;
    fn num_925() -> Self;
    fn num_926() -> Self;
    fn num_927() -> Self;
    fn num_928() -> Self;
    fn num_929() -> Self;
    fn num_930() -> Self;
    fn num_931() -> Self;
    fn num_932() -> Self;
    fn num_933() -> Self;
    fn num_934() -> Self;
    fn num_935() -> Self;
    fn num_936() -> Self;
    fn num_937() -> Self;
    fn num_938() -> Self;
    fn num_939() -> Self;
    fn num_940() -> Self;
    fn num_941() -> Self;
    fn num_942() -> Self;
    fn num_943() -> Self;
    fn num_944() -> Self;
    fn num_945() -> Self;
    fn num_946() -> Self;
    fn num_947() -> Self;
    fn num_948() -> Self;
    fn num_949() -> Self;
    fn num_950() -> Self;
    fn num_951() -> Self;
    fn num_952() -> Self;
    fn num_953() -> Self;
    fn num_954() -> Self;
    fn num_955() -> Self;
    fn num_956() -> Self;
    fn num_957() -> Self;
    fn num_958() -> Self;
    fn num_959() -> Self;
    fn num_960() -> Self;
    fn num_961() -> Self;
    fn num_962() -> Self;
    fn num_963() -> Self;
    fn num_964() -> Self;
    fn num_965() -> Self;
    fn num_966() -> Self;
    fn num_967() -> Self;
    fn num_968() -> Self;
    fn num_969() -> Self;
    fn num_970() -> Self;
    fn num_971() -> Self;
    fn num_972() -> Self;
    fn num_973() -> Self;
    fn num_974() -> Self;
    fn num_975() -> Self;
    fn num_976() -> Self;
    fn num_977() -> Self;
    fn num_978() -> Self;
    fn num_979() -> Self;
    fn num_980() -> Self;
    fn num_981() -> Self;
    fn num_982() -> Self;
    fn num_983() -> Self;
    fn num_984() -> Self;
    fn num_985() -> Self;
    fn num_986() -> Self;
    fn num_987() -> Self;
    fn num_988() -> Self;
    fn num_989() -> Self;
    fn num_990() -> Self;
    fn num_991() -> Self;
    fn num_992() -> Self;
    fn num_993() -> Self;
    fn num_994() -> Self;
    fn num_995() -> Self;
    fn num_996() -> Self;
    fn num_997() -> Self;
    fn num_998() -> Self;
    fn num_999() -> Self;
    fn num_1000() -> Self;
    fn num_1001() -> Self;
    fn num_1002() -> Self;
    fn num_1003() -> Self;
    fn num_1004() -> Self;
    fn num_1005() -> Self;
    fn num_1006() -> Self;
    fn num_1007() -> Self;
    fn num_1008() -> Self;
    fn num_1009() -> Self;
    fn num_1010() -> Self;
    fn num_1011() -> Self;
    fn num_1012() -> Self;
    fn num_1013() -> Self;
    fn num_1014() -> Self;
    fn num_1015() -> Self;
    fn num_1016() -> Self;
    fn num_1017() -> Self;
    fn num_1018() -> Self;
    fn num_1019() -> Self;
    fn num_1020() -> Self;
    fn num_1021() -> Self;
    fn num_1022() -> Self;
    fn num_1023() -> Self;
}
const impl<
    T: SupportsRange1024
        + [const] One
        + [const] Zero
        + [const] core::ops::Add<Output = T>
        + [const] Numbers512
        + [const] Numbers128,
> Numbers1024 for T
{
    fn num_512() -> Self {
        Self::num_511() + Self::num_1()
    }
    fn num_513() -> Self {
        Self::num_512() + Self::num_1()
    }
    fn num_514() -> Self {
        Self::num_513() + Self::num_1()
    }
    fn num_515() -> Self {
        Self::num_514() + Self::num_1()
    }
    fn num_516() -> Self {
        Self::num_515() + Self::num_1()
    }
    fn num_517() -> Self {
        Self::num_516() + Self::num_1()
    }
    fn num_518() -> Self {
        Self::num_517() + Self::num_1()
    }
    fn num_519() -> Self {
        Self::num_518() + Self::num_1()
    }
    fn num_520() -> Self {
        Self::num_519() + Self::num_1()
    }
    fn num_521() -> Self {
        Self::num_520() + Self::num_1()
    }
    fn num_522() -> Self {
        Self::num_521() + Self::num_1()
    }
    fn num_523() -> Self {
        Self::num_522() + Self::num_1()
    }
    fn num_524() -> Self {
        Self::num_523() + Self::num_1()
    }
    fn num_525() -> Self {
        Self::num_524() + Self::num_1()
    }
    fn num_526() -> Self {
        Self::num_525() + Self::num_1()
    }
    fn num_527() -> Self {
        Self::num_526() + Self::num_1()
    }
    fn num_528() -> Self {
        Self::num_527() + Self::num_1()
    }
    fn num_529() -> Self {
        Self::num_528() + Self::num_1()
    }
    fn num_530() -> Self {
        Self::num_529() + Self::num_1()
    }
    fn num_531() -> Self {
        Self::num_530() + Self::num_1()
    }
    fn num_532() -> Self {
        Self::num_531() + Self::num_1()
    }
    fn num_533() -> Self {
        Self::num_532() + Self::num_1()
    }
    fn num_534() -> Self {
        Self::num_533() + Self::num_1()
    }
    fn num_535() -> Self {
        Self::num_534() + Self::num_1()
    }
    fn num_536() -> Self {
        Self::num_535() + Self::num_1()
    }
    fn num_537() -> Self {
        Self::num_536() + Self::num_1()
    }
    fn num_538() -> Self {
        Self::num_537() + Self::num_1()
    }
    fn num_539() -> Self {
        Self::num_538() + Self::num_1()
    }
    fn num_540() -> Self {
        Self::num_539() + Self::num_1()
    }
    fn num_541() -> Self {
        Self::num_540() + Self::num_1()
    }
    fn num_542() -> Self {
        Self::num_541() + Self::num_1()
    }
    fn num_543() -> Self {
        Self::num_542() + Self::num_1()
    }
    fn num_544() -> Self {
        Self::num_543() + Self::num_1()
    }
    fn num_545() -> Self {
        Self::num_544() + Self::num_1()
    }
    fn num_546() -> Self {
        Self::num_545() + Self::num_1()
    }
    fn num_547() -> Self {
        Self::num_546() + Self::num_1()
    }
    fn num_548() -> Self {
        Self::num_547() + Self::num_1()
    }
    fn num_549() -> Self {
        Self::num_548() + Self::num_1()
    }
    fn num_550() -> Self {
        Self::num_549() + Self::num_1()
    }
    fn num_551() -> Self {
        Self::num_550() + Self::num_1()
    }
    fn num_552() -> Self {
        Self::num_551() + Self::num_1()
    }
    fn num_553() -> Self {
        Self::num_552() + Self::num_1()
    }
    fn num_554() -> Self {
        Self::num_553() + Self::num_1()
    }
    fn num_555() -> Self {
        Self::num_554() + Self::num_1()
    }
    fn num_556() -> Self {
        Self::num_555() + Self::num_1()
    }
    fn num_557() -> Self {
        Self::num_556() + Self::num_1()
    }
    fn num_558() -> Self {
        Self::num_557() + Self::num_1()
    }
    fn num_559() -> Self {
        Self::num_558() + Self::num_1()
    }
    fn num_560() -> Self {
        Self::num_559() + Self::num_1()
    }
    fn num_561() -> Self {
        Self::num_560() + Self::num_1()
    }
    fn num_562() -> Self {
        Self::num_561() + Self::num_1()
    }
    fn num_563() -> Self {
        Self::num_562() + Self::num_1()
    }
    fn num_564() -> Self {
        Self::num_563() + Self::num_1()
    }
    fn num_565() -> Self {
        Self::num_564() + Self::num_1()
    }
    fn num_566() -> Self {
        Self::num_565() + Self::num_1()
    }
    fn num_567() -> Self {
        Self::num_566() + Self::num_1()
    }
    fn num_568() -> Self {
        Self::num_567() + Self::num_1()
    }
    fn num_569() -> Self {
        Self::num_568() + Self::num_1()
    }
    fn num_570() -> Self {
        Self::num_569() + Self::num_1()
    }
    fn num_571() -> Self {
        Self::num_570() + Self::num_1()
    }
    fn num_572() -> Self {
        Self::num_571() + Self::num_1()
    }
    fn num_573() -> Self {
        Self::num_572() + Self::num_1()
    }
    fn num_574() -> Self {
        Self::num_573() + Self::num_1()
    }
    fn num_575() -> Self {
        Self::num_574() + Self::num_1()
    }
    fn num_576() -> Self {
        Self::num_575() + Self::num_1()
    }
    fn num_577() -> Self {
        Self::num_576() + Self::num_1()
    }
    fn num_578() -> Self {
        Self::num_577() + Self::num_1()
    }
    fn num_579() -> Self {
        Self::num_578() + Self::num_1()
    }
    fn num_580() -> Self {
        Self::num_579() + Self::num_1()
    }
    fn num_581() -> Self {
        Self::num_580() + Self::num_1()
    }
    fn num_582() -> Self {
        Self::num_581() + Self::num_1()
    }
    fn num_583() -> Self {
        Self::num_582() + Self::num_1()
    }
    fn num_584() -> Self {
        Self::num_583() + Self::num_1()
    }
    fn num_585() -> Self {
        Self::num_584() + Self::num_1()
    }
    fn num_586() -> Self {
        Self::num_585() + Self::num_1()
    }
    fn num_587() -> Self {
        Self::num_586() + Self::num_1()
    }
    fn num_588() -> Self {
        Self::num_587() + Self::num_1()
    }
    fn num_589() -> Self {
        Self::num_588() + Self::num_1()
    }
    fn num_590() -> Self {
        Self::num_589() + Self::num_1()
    }
    fn num_591() -> Self {
        Self::num_590() + Self::num_1()
    }
    fn num_592() -> Self {
        Self::num_591() + Self::num_1()
    }
    fn num_593() -> Self {
        Self::num_592() + Self::num_1()
    }
    fn num_594() -> Self {
        Self::num_593() + Self::num_1()
    }
    fn num_595() -> Self {
        Self::num_594() + Self::num_1()
    }
    fn num_596() -> Self {
        Self::num_595() + Self::num_1()
    }
    fn num_597() -> Self {
        Self::num_596() + Self::num_1()
    }
    fn num_598() -> Self {
        Self::num_597() + Self::num_1()
    }
    fn num_599() -> Self {
        Self::num_598() + Self::num_1()
    }
    fn num_600() -> Self {
        Self::num_599() + Self::num_1()
    }
    fn num_601() -> Self {
        Self::num_600() + Self::num_1()
    }
    fn num_602() -> Self {
        Self::num_601() + Self::num_1()
    }
    fn num_603() -> Self {
        Self::num_602() + Self::num_1()
    }
    fn num_604() -> Self {
        Self::num_603() + Self::num_1()
    }
    fn num_605() -> Self {
        Self::num_604() + Self::num_1()
    }
    fn num_606() -> Self {
        Self::num_605() + Self::num_1()
    }
    fn num_607() -> Self {
        Self::num_606() + Self::num_1()
    }
    fn num_608() -> Self {
        Self::num_607() + Self::num_1()
    }
    fn num_609() -> Self {
        Self::num_608() + Self::num_1()
    }
    fn num_610() -> Self {
        Self::num_609() + Self::num_1()
    }
    fn num_611() -> Self {
        Self::num_610() + Self::num_1()
    }
    fn num_612() -> Self {
        Self::num_611() + Self::num_1()
    }
    fn num_613() -> Self {
        Self::num_612() + Self::num_1()
    }
    fn num_614() -> Self {
        Self::num_613() + Self::num_1()
    }
    fn num_615() -> Self {
        Self::num_614() + Self::num_1()
    }
    fn num_616() -> Self {
        Self::num_615() + Self::num_1()
    }
    fn num_617() -> Self {
        Self::num_616() + Self::num_1()
    }
    fn num_618() -> Self {
        Self::num_617() + Self::num_1()
    }
    fn num_619() -> Self {
        Self::num_618() + Self::num_1()
    }
    fn num_620() -> Self {
        Self::num_619() + Self::num_1()
    }
    fn num_621() -> Self {
        Self::num_620() + Self::num_1()
    }
    fn num_622() -> Self {
        Self::num_621() + Self::num_1()
    }
    fn num_623() -> Self {
        Self::num_622() + Self::num_1()
    }
    fn num_624() -> Self {
        Self::num_623() + Self::num_1()
    }
    fn num_625() -> Self {
        Self::num_624() + Self::num_1()
    }
    fn num_626() -> Self {
        Self::num_625() + Self::num_1()
    }
    fn num_627() -> Self {
        Self::num_626() + Self::num_1()
    }
    fn num_628() -> Self {
        Self::num_627() + Self::num_1()
    }
    fn num_629() -> Self {
        Self::num_628() + Self::num_1()
    }
    fn num_630() -> Self {
        Self::num_629() + Self::num_1()
    }
    fn num_631() -> Self {
        Self::num_630() + Self::num_1()
    }
    fn num_632() -> Self {
        Self::num_631() + Self::num_1()
    }
    fn num_633() -> Self {
        Self::num_632() + Self::num_1()
    }
    fn num_634() -> Self {
        Self::num_633() + Self::num_1()
    }
    fn num_635() -> Self {
        Self::num_634() + Self::num_1()
    }
    fn num_636() -> Self {
        Self::num_635() + Self::num_1()
    }
    fn num_637() -> Self {
        Self::num_636() + Self::num_1()
    }
    fn num_638() -> Self {
        Self::num_637() + Self::num_1()
    }
    fn num_639() -> Self {
        Self::num_638() + Self::num_1()
    }
    fn num_640() -> Self {
        Self::num_639() + Self::num_1()
    }
    fn num_641() -> Self {
        Self::num_640() + Self::num_1()
    }
    fn num_642() -> Self {
        Self::num_641() + Self::num_1()
    }
    fn num_643() -> Self {
        Self::num_642() + Self::num_1()
    }
    fn num_644() -> Self {
        Self::num_643() + Self::num_1()
    }
    fn num_645() -> Self {
        Self::num_644() + Self::num_1()
    }
    fn num_646() -> Self {
        Self::num_645() + Self::num_1()
    }
    fn num_647() -> Self {
        Self::num_646() + Self::num_1()
    }
    fn num_648() -> Self {
        Self::num_647() + Self::num_1()
    }
    fn num_649() -> Self {
        Self::num_648() + Self::num_1()
    }
    fn num_650() -> Self {
        Self::num_649() + Self::num_1()
    }
    fn num_651() -> Self {
        Self::num_650() + Self::num_1()
    }
    fn num_652() -> Self {
        Self::num_651() + Self::num_1()
    }
    fn num_653() -> Self {
        Self::num_652() + Self::num_1()
    }
    fn num_654() -> Self {
        Self::num_653() + Self::num_1()
    }
    fn num_655() -> Self {
        Self::num_654() + Self::num_1()
    }
    fn num_656() -> Self {
        Self::num_655() + Self::num_1()
    }
    fn num_657() -> Self {
        Self::num_656() + Self::num_1()
    }
    fn num_658() -> Self {
        Self::num_657() + Self::num_1()
    }
    fn num_659() -> Self {
        Self::num_658() + Self::num_1()
    }
    fn num_660() -> Self {
        Self::num_659() + Self::num_1()
    }
    fn num_661() -> Self {
        Self::num_660() + Self::num_1()
    }
    fn num_662() -> Self {
        Self::num_661() + Self::num_1()
    }
    fn num_663() -> Self {
        Self::num_662() + Self::num_1()
    }
    fn num_664() -> Self {
        Self::num_663() + Self::num_1()
    }
    fn num_665() -> Self {
        Self::num_664() + Self::num_1()
    }
    fn num_666() -> Self {
        Self::num_665() + Self::num_1()
    }
    fn num_667() -> Self {
        Self::num_666() + Self::num_1()
    }
    fn num_668() -> Self {
        Self::num_667() + Self::num_1()
    }
    fn num_669() -> Self {
        Self::num_668() + Self::num_1()
    }
    fn num_670() -> Self {
        Self::num_669() + Self::num_1()
    }
    fn num_671() -> Self {
        Self::num_670() + Self::num_1()
    }
    fn num_672() -> Self {
        Self::num_671() + Self::num_1()
    }
    fn num_673() -> Self {
        Self::num_672() + Self::num_1()
    }
    fn num_674() -> Self {
        Self::num_673() + Self::num_1()
    }
    fn num_675() -> Self {
        Self::num_674() + Self::num_1()
    }
    fn num_676() -> Self {
        Self::num_675() + Self::num_1()
    }
    fn num_677() -> Self {
        Self::num_676() + Self::num_1()
    }
    fn num_678() -> Self {
        Self::num_677() + Self::num_1()
    }
    fn num_679() -> Self {
        Self::num_678() + Self::num_1()
    }
    fn num_680() -> Self {
        Self::num_679() + Self::num_1()
    }
    fn num_681() -> Self {
        Self::num_680() + Self::num_1()
    }
    fn num_682() -> Self {
        Self::num_681() + Self::num_1()
    }
    fn num_683() -> Self {
        Self::num_682() + Self::num_1()
    }
    fn num_684() -> Self {
        Self::num_683() + Self::num_1()
    }
    fn num_685() -> Self {
        Self::num_684() + Self::num_1()
    }
    fn num_686() -> Self {
        Self::num_685() + Self::num_1()
    }
    fn num_687() -> Self {
        Self::num_686() + Self::num_1()
    }
    fn num_688() -> Self {
        Self::num_687() + Self::num_1()
    }
    fn num_689() -> Self {
        Self::num_688() + Self::num_1()
    }
    fn num_690() -> Self {
        Self::num_689() + Self::num_1()
    }
    fn num_691() -> Self {
        Self::num_690() + Self::num_1()
    }
    fn num_692() -> Self {
        Self::num_691() + Self::num_1()
    }
    fn num_693() -> Self {
        Self::num_692() + Self::num_1()
    }
    fn num_694() -> Self {
        Self::num_693() + Self::num_1()
    }
    fn num_695() -> Self {
        Self::num_694() + Self::num_1()
    }
    fn num_696() -> Self {
        Self::num_695() + Self::num_1()
    }
    fn num_697() -> Self {
        Self::num_696() + Self::num_1()
    }
    fn num_698() -> Self {
        Self::num_697() + Self::num_1()
    }
    fn num_699() -> Self {
        Self::num_698() + Self::num_1()
    }
    fn num_700() -> Self {
        Self::num_699() + Self::num_1()
    }
    fn num_701() -> Self {
        Self::num_700() + Self::num_1()
    }
    fn num_702() -> Self {
        Self::num_701() + Self::num_1()
    }
    fn num_703() -> Self {
        Self::num_702() + Self::num_1()
    }
    fn num_704() -> Self {
        Self::num_703() + Self::num_1()
    }
    fn num_705() -> Self {
        Self::num_704() + Self::num_1()
    }
    fn num_706() -> Self {
        Self::num_705() + Self::num_1()
    }
    fn num_707() -> Self {
        Self::num_706() + Self::num_1()
    }
    fn num_708() -> Self {
        Self::num_707() + Self::num_1()
    }
    fn num_709() -> Self {
        Self::num_708() + Self::num_1()
    }
    fn num_710() -> Self {
        Self::num_709() + Self::num_1()
    }
    fn num_711() -> Self {
        Self::num_710() + Self::num_1()
    }
    fn num_712() -> Self {
        Self::num_711() + Self::num_1()
    }
    fn num_713() -> Self {
        Self::num_712() + Self::num_1()
    }
    fn num_714() -> Self {
        Self::num_713() + Self::num_1()
    }
    fn num_715() -> Self {
        Self::num_714() + Self::num_1()
    }
    fn num_716() -> Self {
        Self::num_715() + Self::num_1()
    }
    fn num_717() -> Self {
        Self::num_716() + Self::num_1()
    }
    fn num_718() -> Self {
        Self::num_717() + Self::num_1()
    }
    fn num_719() -> Self {
        Self::num_718() + Self::num_1()
    }
    fn num_720() -> Self {
        Self::num_719() + Self::num_1()
    }
    fn num_721() -> Self {
        Self::num_720() + Self::num_1()
    }
    fn num_722() -> Self {
        Self::num_721() + Self::num_1()
    }
    fn num_723() -> Self {
        Self::num_722() + Self::num_1()
    }
    fn num_724() -> Self {
        Self::num_723() + Self::num_1()
    }
    fn num_725() -> Self {
        Self::num_724() + Self::num_1()
    }
    fn num_726() -> Self {
        Self::num_725() + Self::num_1()
    }
    fn num_727() -> Self {
        Self::num_726() + Self::num_1()
    }
    fn num_728() -> Self {
        Self::num_727() + Self::num_1()
    }
    fn num_729() -> Self {
        Self::num_728() + Self::num_1()
    }
    fn num_730() -> Self {
        Self::num_729() + Self::num_1()
    }
    fn num_731() -> Self {
        Self::num_730() + Self::num_1()
    }
    fn num_732() -> Self {
        Self::num_731() + Self::num_1()
    }
    fn num_733() -> Self {
        Self::num_732() + Self::num_1()
    }
    fn num_734() -> Self {
        Self::num_733() + Self::num_1()
    }
    fn num_735() -> Self {
        Self::num_734() + Self::num_1()
    }
    fn num_736() -> Self {
        Self::num_735() + Self::num_1()
    }
    fn num_737() -> Self {
        Self::num_736() + Self::num_1()
    }
    fn num_738() -> Self {
        Self::num_737() + Self::num_1()
    }
    fn num_739() -> Self {
        Self::num_738() + Self::num_1()
    }
    fn num_740() -> Self {
        Self::num_739() + Self::num_1()
    }
    fn num_741() -> Self {
        Self::num_740() + Self::num_1()
    }
    fn num_742() -> Self {
        Self::num_741() + Self::num_1()
    }
    fn num_743() -> Self {
        Self::num_742() + Self::num_1()
    }
    fn num_744() -> Self {
        Self::num_743() + Self::num_1()
    }
    fn num_745() -> Self {
        Self::num_744() + Self::num_1()
    }
    fn num_746() -> Self {
        Self::num_745() + Self::num_1()
    }
    fn num_747() -> Self {
        Self::num_746() + Self::num_1()
    }
    fn num_748() -> Self {
        Self::num_747() + Self::num_1()
    }
    fn num_749() -> Self {
        Self::num_748() + Self::num_1()
    }
    fn num_750() -> Self {
        Self::num_749() + Self::num_1()
    }
    fn num_751() -> Self {
        Self::num_750() + Self::num_1()
    }
    fn num_752() -> Self {
        Self::num_751() + Self::num_1()
    }
    fn num_753() -> Self {
        Self::num_752() + Self::num_1()
    }
    fn num_754() -> Self {
        Self::num_753() + Self::num_1()
    }
    fn num_755() -> Self {
        Self::num_754() + Self::num_1()
    }
    fn num_756() -> Self {
        Self::num_755() + Self::num_1()
    }
    fn num_757() -> Self {
        Self::num_756() + Self::num_1()
    }
    fn num_758() -> Self {
        Self::num_757() + Self::num_1()
    }
    fn num_759() -> Self {
        Self::num_758() + Self::num_1()
    }
    fn num_760() -> Self {
        Self::num_759() + Self::num_1()
    }
    fn num_761() -> Self {
        Self::num_760() + Self::num_1()
    }
    fn num_762() -> Self {
        Self::num_761() + Self::num_1()
    }
    fn num_763() -> Self {
        Self::num_762() + Self::num_1()
    }
    fn num_764() -> Self {
        Self::num_763() + Self::num_1()
    }
    fn num_765() -> Self {
        Self::num_764() + Self::num_1()
    }
    fn num_766() -> Self {
        Self::num_765() + Self::num_1()
    }
    fn num_767() -> Self {
        Self::num_766() + Self::num_1()
    }
    fn num_768() -> Self {
        Self::num_767() + Self::num_1()
    }
    fn num_769() -> Self {
        Self::num_768() + Self::num_1()
    }
    fn num_770() -> Self {
        Self::num_769() + Self::num_1()
    }
    fn num_771() -> Self {
        Self::num_770() + Self::num_1()
    }
    fn num_772() -> Self {
        Self::num_771() + Self::num_1()
    }
    fn num_773() -> Self {
        Self::num_772() + Self::num_1()
    }
    fn num_774() -> Self {
        Self::num_773() + Self::num_1()
    }
    fn num_775() -> Self {
        Self::num_774() + Self::num_1()
    }
    fn num_776() -> Self {
        Self::num_775() + Self::num_1()
    }
    fn num_777() -> Self {
        Self::num_776() + Self::num_1()
    }
    fn num_778() -> Self {
        Self::num_777() + Self::num_1()
    }
    fn num_779() -> Self {
        Self::num_778() + Self::num_1()
    }
    fn num_780() -> Self {
        Self::num_779() + Self::num_1()
    }
    fn num_781() -> Self {
        Self::num_780() + Self::num_1()
    }
    fn num_782() -> Self {
        Self::num_781() + Self::num_1()
    }
    fn num_783() -> Self {
        Self::num_782() + Self::num_1()
    }
    fn num_784() -> Self {
        Self::num_783() + Self::num_1()
    }
    fn num_785() -> Self {
        Self::num_784() + Self::num_1()
    }
    fn num_786() -> Self {
        Self::num_785() + Self::num_1()
    }
    fn num_787() -> Self {
        Self::num_786() + Self::num_1()
    }
    fn num_788() -> Self {
        Self::num_787() + Self::num_1()
    }
    fn num_789() -> Self {
        Self::num_788() + Self::num_1()
    }
    fn num_790() -> Self {
        Self::num_789() + Self::num_1()
    }
    fn num_791() -> Self {
        Self::num_790() + Self::num_1()
    }
    fn num_792() -> Self {
        Self::num_791() + Self::num_1()
    }
    fn num_793() -> Self {
        Self::num_792() + Self::num_1()
    }
    fn num_794() -> Self {
        Self::num_793() + Self::num_1()
    }
    fn num_795() -> Self {
        Self::num_794() + Self::num_1()
    }
    fn num_796() -> Self {
        Self::num_795() + Self::num_1()
    }
    fn num_797() -> Self {
        Self::num_796() + Self::num_1()
    }
    fn num_798() -> Self {
        Self::num_797() + Self::num_1()
    }
    fn num_799() -> Self {
        Self::num_798() + Self::num_1()
    }
    fn num_800() -> Self {
        Self::num_799() + Self::num_1()
    }
    fn num_801() -> Self {
        Self::num_800() + Self::num_1()
    }
    fn num_802() -> Self {
        Self::num_801() + Self::num_1()
    }
    fn num_803() -> Self {
        Self::num_802() + Self::num_1()
    }
    fn num_804() -> Self {
        Self::num_803() + Self::num_1()
    }
    fn num_805() -> Self {
        Self::num_804() + Self::num_1()
    }
    fn num_806() -> Self {
        Self::num_805() + Self::num_1()
    }
    fn num_807() -> Self {
        Self::num_806() + Self::num_1()
    }
    fn num_808() -> Self {
        Self::num_807() + Self::num_1()
    }
    fn num_809() -> Self {
        Self::num_808() + Self::num_1()
    }
    fn num_810() -> Self {
        Self::num_809() + Self::num_1()
    }
    fn num_811() -> Self {
        Self::num_810() + Self::num_1()
    }
    fn num_812() -> Self {
        Self::num_811() + Self::num_1()
    }
    fn num_813() -> Self {
        Self::num_812() + Self::num_1()
    }
    fn num_814() -> Self {
        Self::num_813() + Self::num_1()
    }
    fn num_815() -> Self {
        Self::num_814() + Self::num_1()
    }
    fn num_816() -> Self {
        Self::num_815() + Self::num_1()
    }
    fn num_817() -> Self {
        Self::num_816() + Self::num_1()
    }
    fn num_818() -> Self {
        Self::num_817() + Self::num_1()
    }
    fn num_819() -> Self {
        Self::num_818() + Self::num_1()
    }
    fn num_820() -> Self {
        Self::num_819() + Self::num_1()
    }
    fn num_821() -> Self {
        Self::num_820() + Self::num_1()
    }
    fn num_822() -> Self {
        Self::num_821() + Self::num_1()
    }
    fn num_823() -> Self {
        Self::num_822() + Self::num_1()
    }
    fn num_824() -> Self {
        Self::num_823() + Self::num_1()
    }
    fn num_825() -> Self {
        Self::num_824() + Self::num_1()
    }
    fn num_826() -> Self {
        Self::num_825() + Self::num_1()
    }
    fn num_827() -> Self {
        Self::num_826() + Self::num_1()
    }
    fn num_828() -> Self {
        Self::num_827() + Self::num_1()
    }
    fn num_829() -> Self {
        Self::num_828() + Self::num_1()
    }
    fn num_830() -> Self {
        Self::num_829() + Self::num_1()
    }
    fn num_831() -> Self {
        Self::num_830() + Self::num_1()
    }
    fn num_832() -> Self {
        Self::num_831() + Self::num_1()
    }
    fn num_833() -> Self {
        Self::num_832() + Self::num_1()
    }
    fn num_834() -> Self {
        Self::num_833() + Self::num_1()
    }
    fn num_835() -> Self {
        Self::num_834() + Self::num_1()
    }
    fn num_836() -> Self {
        Self::num_835() + Self::num_1()
    }
    fn num_837() -> Self {
        Self::num_836() + Self::num_1()
    }
    fn num_838() -> Self {
        Self::num_837() + Self::num_1()
    }
    fn num_839() -> Self {
        Self::num_838() + Self::num_1()
    }
    fn num_840() -> Self {
        Self::num_839() + Self::num_1()
    }
    fn num_841() -> Self {
        Self::num_840() + Self::num_1()
    }
    fn num_842() -> Self {
        Self::num_841() + Self::num_1()
    }
    fn num_843() -> Self {
        Self::num_842() + Self::num_1()
    }
    fn num_844() -> Self {
        Self::num_843() + Self::num_1()
    }
    fn num_845() -> Self {
        Self::num_844() + Self::num_1()
    }
    fn num_846() -> Self {
        Self::num_845() + Self::num_1()
    }
    fn num_847() -> Self {
        Self::num_846() + Self::num_1()
    }
    fn num_848() -> Self {
        Self::num_847() + Self::num_1()
    }
    fn num_849() -> Self {
        Self::num_848() + Self::num_1()
    }
    fn num_850() -> Self {
        Self::num_849() + Self::num_1()
    }
    fn num_851() -> Self {
        Self::num_850() + Self::num_1()
    }
    fn num_852() -> Self {
        Self::num_851() + Self::num_1()
    }
    fn num_853() -> Self {
        Self::num_852() + Self::num_1()
    }
    fn num_854() -> Self {
        Self::num_853() + Self::num_1()
    }
    fn num_855() -> Self {
        Self::num_854() + Self::num_1()
    }
    fn num_856() -> Self {
        Self::num_855() + Self::num_1()
    }
    fn num_857() -> Self {
        Self::num_856() + Self::num_1()
    }
    fn num_858() -> Self {
        Self::num_857() + Self::num_1()
    }
    fn num_859() -> Self {
        Self::num_858() + Self::num_1()
    }
    fn num_860() -> Self {
        Self::num_859() + Self::num_1()
    }
    fn num_861() -> Self {
        Self::num_860() + Self::num_1()
    }
    fn num_862() -> Self {
        Self::num_861() + Self::num_1()
    }
    fn num_863() -> Self {
        Self::num_862() + Self::num_1()
    }
    fn num_864() -> Self {
        Self::num_863() + Self::num_1()
    }
    fn num_865() -> Self {
        Self::num_864() + Self::num_1()
    }
    fn num_866() -> Self {
        Self::num_865() + Self::num_1()
    }
    fn num_867() -> Self {
        Self::num_866() + Self::num_1()
    }
    fn num_868() -> Self {
        Self::num_867() + Self::num_1()
    }
    fn num_869() -> Self {
        Self::num_868() + Self::num_1()
    }
    fn num_870() -> Self {
        Self::num_869() + Self::num_1()
    }
    fn num_871() -> Self {
        Self::num_870() + Self::num_1()
    }
    fn num_872() -> Self {
        Self::num_871() + Self::num_1()
    }
    fn num_873() -> Self {
        Self::num_872() + Self::num_1()
    }
    fn num_874() -> Self {
        Self::num_873() + Self::num_1()
    }
    fn num_875() -> Self {
        Self::num_874() + Self::num_1()
    }
    fn num_876() -> Self {
        Self::num_875() + Self::num_1()
    }
    fn num_877() -> Self {
        Self::num_876() + Self::num_1()
    }
    fn num_878() -> Self {
        Self::num_877() + Self::num_1()
    }
    fn num_879() -> Self {
        Self::num_878() + Self::num_1()
    }
    fn num_880() -> Self {
        Self::num_879() + Self::num_1()
    }
    fn num_881() -> Self {
        Self::num_880() + Self::num_1()
    }
    fn num_882() -> Self {
        Self::num_881() + Self::num_1()
    }
    fn num_883() -> Self {
        Self::num_882() + Self::num_1()
    }
    fn num_884() -> Self {
        Self::num_883() + Self::num_1()
    }
    fn num_885() -> Self {
        Self::num_884() + Self::num_1()
    }
    fn num_886() -> Self {
        Self::num_885() + Self::num_1()
    }
    fn num_887() -> Self {
        Self::num_886() + Self::num_1()
    }
    fn num_888() -> Self {
        Self::num_887() + Self::num_1()
    }
    fn num_889() -> Self {
        Self::num_888() + Self::num_1()
    }
    fn num_890() -> Self {
        Self::num_889() + Self::num_1()
    }
    fn num_891() -> Self {
        Self::num_890() + Self::num_1()
    }
    fn num_892() -> Self {
        Self::num_891() + Self::num_1()
    }
    fn num_893() -> Self {
        Self::num_892() + Self::num_1()
    }
    fn num_894() -> Self {
        Self::num_893() + Self::num_1()
    }
    fn num_895() -> Self {
        Self::num_894() + Self::num_1()
    }
    fn num_896() -> Self {
        Self::num_895() + Self::num_1()
    }
    fn num_897() -> Self {
        Self::num_896() + Self::num_1()
    }
    fn num_898() -> Self {
        Self::num_897() + Self::num_1()
    }
    fn num_899() -> Self {
        Self::num_898() + Self::num_1()
    }
    fn num_900() -> Self {
        Self::num_899() + Self::num_1()
    }
    fn num_901() -> Self {
        Self::num_900() + Self::num_1()
    }
    fn num_902() -> Self {
        Self::num_901() + Self::num_1()
    }
    fn num_903() -> Self {
        Self::num_902() + Self::num_1()
    }
    fn num_904() -> Self {
        Self::num_903() + Self::num_1()
    }
    fn num_905() -> Self {
        Self::num_904() + Self::num_1()
    }
    fn num_906() -> Self {
        Self::num_905() + Self::num_1()
    }
    fn num_907() -> Self {
        Self::num_906() + Self::num_1()
    }
    fn num_908() -> Self {
        Self::num_907() + Self::num_1()
    }
    fn num_909() -> Self {
        Self::num_908() + Self::num_1()
    }
    fn num_910() -> Self {
        Self::num_909() + Self::num_1()
    }
    fn num_911() -> Self {
        Self::num_910() + Self::num_1()
    }
    fn num_912() -> Self {
        Self::num_911() + Self::num_1()
    }
    fn num_913() -> Self {
        Self::num_912() + Self::num_1()
    }
    fn num_914() -> Self {
        Self::num_913() + Self::num_1()
    }
    fn num_915() -> Self {
        Self::num_914() + Self::num_1()
    }
    fn num_916() -> Self {
        Self::num_915() + Self::num_1()
    }
    fn num_917() -> Self {
        Self::num_916() + Self::num_1()
    }
    fn num_918() -> Self {
        Self::num_917() + Self::num_1()
    }
    fn num_919() -> Self {
        Self::num_918() + Self::num_1()
    }
    fn num_920() -> Self {
        Self::num_919() + Self::num_1()
    }
    fn num_921() -> Self {
        Self::num_920() + Self::num_1()
    }
    fn num_922() -> Self {
        Self::num_921() + Self::num_1()
    }
    fn num_923() -> Self {
        Self::num_922() + Self::num_1()
    }
    fn num_924() -> Self {
        Self::num_923() + Self::num_1()
    }
    fn num_925() -> Self {
        Self::num_924() + Self::num_1()
    }
    fn num_926() -> Self {
        Self::num_925() + Self::num_1()
    }
    fn num_927() -> Self {
        Self::num_926() + Self::num_1()
    }
    fn num_928() -> Self {
        Self::num_927() + Self::num_1()
    }
    fn num_929() -> Self {
        Self::num_928() + Self::num_1()
    }
    fn num_930() -> Self {
        Self::num_929() + Self::num_1()
    }
    fn num_931() -> Self {
        Self::num_930() + Self::num_1()
    }
    fn num_932() -> Self {
        Self::num_931() + Self::num_1()
    }
    fn num_933() -> Self {
        Self::num_932() + Self::num_1()
    }
    fn num_934() -> Self {
        Self::num_933() + Self::num_1()
    }
    fn num_935() -> Self {
        Self::num_934() + Self::num_1()
    }
    fn num_936() -> Self {
        Self::num_935() + Self::num_1()
    }
    fn num_937() -> Self {
        Self::num_936() + Self::num_1()
    }
    fn num_938() -> Self {
        Self::num_937() + Self::num_1()
    }
    fn num_939() -> Self {
        Self::num_938() + Self::num_1()
    }
    fn num_940() -> Self {
        Self::num_939() + Self::num_1()
    }
    fn num_941() -> Self {
        Self::num_940() + Self::num_1()
    }
    fn num_942() -> Self {
        Self::num_941() + Self::num_1()
    }
    fn num_943() -> Self {
        Self::num_942() + Self::num_1()
    }
    fn num_944() -> Self {
        Self::num_943() + Self::num_1()
    }
    fn num_945() -> Self {
        Self::num_944() + Self::num_1()
    }
    fn num_946() -> Self {
        Self::num_945() + Self::num_1()
    }
    fn num_947() -> Self {
        Self::num_946() + Self::num_1()
    }
    fn num_948() -> Self {
        Self::num_947() + Self::num_1()
    }
    fn num_949() -> Self {
        Self::num_948() + Self::num_1()
    }
    fn num_950() -> Self {
        Self::num_949() + Self::num_1()
    }
    fn num_951() -> Self {
        Self::num_950() + Self::num_1()
    }
    fn num_952() -> Self {
        Self::num_951() + Self::num_1()
    }
    fn num_953() -> Self {
        Self::num_952() + Self::num_1()
    }
    fn num_954() -> Self {
        Self::num_953() + Self::num_1()
    }
    fn num_955() -> Self {
        Self::num_954() + Self::num_1()
    }
    fn num_956() -> Self {
        Self::num_955() + Self::num_1()
    }
    fn num_957() -> Self {
        Self::num_956() + Self::num_1()
    }
    fn num_958() -> Self {
        Self::num_957() + Self::num_1()
    }
    fn num_959() -> Self {
        Self::num_958() + Self::num_1()
    }
    fn num_960() -> Self {
        Self::num_959() + Self::num_1()
    }
    fn num_961() -> Self {
        Self::num_960() + Self::num_1()
    }
    fn num_962() -> Self {
        Self::num_961() + Self::num_1()
    }
    fn num_963() -> Self {
        Self::num_962() + Self::num_1()
    }
    fn num_964() -> Self {
        Self::num_963() + Self::num_1()
    }
    fn num_965() -> Self {
        Self::num_964() + Self::num_1()
    }
    fn num_966() -> Self {
        Self::num_965() + Self::num_1()
    }
    fn num_967() -> Self {
        Self::num_966() + Self::num_1()
    }
    fn num_968() -> Self {
        Self::num_967() + Self::num_1()
    }
    fn num_969() -> Self {
        Self::num_968() + Self::num_1()
    }
    fn num_970() -> Self {
        Self::num_969() + Self::num_1()
    }
    fn num_971() -> Self {
        Self::num_970() + Self::num_1()
    }
    fn num_972() -> Self {
        Self::num_971() + Self::num_1()
    }
    fn num_973() -> Self {
        Self::num_972() + Self::num_1()
    }
    fn num_974() -> Self {
        Self::num_973() + Self::num_1()
    }
    fn num_975() -> Self {
        Self::num_974() + Self::num_1()
    }
    fn num_976() -> Self {
        Self::num_975() + Self::num_1()
    }
    fn num_977() -> Self {
        Self::num_976() + Self::num_1()
    }
    fn num_978() -> Self {
        Self::num_977() + Self::num_1()
    }
    fn num_979() -> Self {
        Self::num_978() + Self::num_1()
    }
    fn num_980() -> Self {
        Self::num_979() + Self::num_1()
    }
    fn num_981() -> Self {
        Self::num_980() + Self::num_1()
    }
    fn num_982() -> Self {
        Self::num_981() + Self::num_1()
    }
    fn num_983() -> Self {
        Self::num_982() + Self::num_1()
    }
    fn num_984() -> Self {
        Self::num_983() + Self::num_1()
    }
    fn num_985() -> Self {
        Self::num_984() + Self::num_1()
    }
    fn num_986() -> Self {
        Self::num_985() + Self::num_1()
    }
    fn num_987() -> Self {
        Self::num_986() + Self::num_1()
    }
    fn num_988() -> Self {
        Self::num_987() + Self::num_1()
    }
    fn num_989() -> Self {
        Self::num_988() + Self::num_1()
    }
    fn num_990() -> Self {
        Self::num_989() + Self::num_1()
    }
    fn num_991() -> Self {
        Self::num_990() + Self::num_1()
    }
    fn num_992() -> Self {
        Self::num_991() + Self::num_1()
    }
    fn num_993() -> Self {
        Self::num_992() + Self::num_1()
    }
    fn num_994() -> Self {
        Self::num_993() + Self::num_1()
    }
    fn num_995() -> Self {
        Self::num_994() + Self::num_1()
    }
    fn num_996() -> Self {
        Self::num_995() + Self::num_1()
    }
    fn num_997() -> Self {
        Self::num_996() + Self::num_1()
    }
    fn num_998() -> Self {
        Self::num_997() + Self::num_1()
    }
    fn num_999() -> Self {
        Self::num_998() + Self::num_1()
    }
    fn num_1000() -> Self {
        Self::num_999() + Self::num_1()
    }
    fn num_1001() -> Self {
        Self::num_1000() + Self::num_1()
    }
    fn num_1002() -> Self {
        Self::num_1001() + Self::num_1()
    }
    fn num_1003() -> Self {
        Self::num_1002() + Self::num_1()
    }
    fn num_1004() -> Self {
        Self::num_1003() + Self::num_1()
    }
    fn num_1005() -> Self {
        Self::num_1004() + Self::num_1()
    }
    fn num_1006() -> Self {
        Self::num_1005() + Self::num_1()
    }
    fn num_1007() -> Self {
        Self::num_1006() + Self::num_1()
    }
    fn num_1008() -> Self {
        Self::num_1007() + Self::num_1()
    }
    fn num_1009() -> Self {
        Self::num_1008() + Self::num_1()
    }
    fn num_1010() -> Self {
        Self::num_1009() + Self::num_1()
    }
    fn num_1011() -> Self {
        Self::num_1010() + Self::num_1()
    }
    fn num_1012() -> Self {
        Self::num_1011() + Self::num_1()
    }
    fn num_1013() -> Self {
        Self::num_1012() + Self::num_1()
    }
    fn num_1014() -> Self {
        Self::num_1013() + Self::num_1()
    }
    fn num_1015() -> Self {
        Self::num_1014() + Self::num_1()
    }
    fn num_1016() -> Self {
        Self::num_1015() + Self::num_1()
    }
    fn num_1017() -> Self {
        Self::num_1016() + Self::num_1()
    }
    fn num_1018() -> Self {
        Self::num_1017() + Self::num_1()
    }
    fn num_1019() -> Self {
        Self::num_1018() + Self::num_1()
    }
    fn num_1020() -> Self {
        Self::num_1019() + Self::num_1()
    }
    fn num_1021() -> Self {
        Self::num_1020() + Self::num_1()
    }
    fn num_1022() -> Self {
        Self::num_1021() + Self::num_1()
    }
    fn num_1023() -> Self {
        Self::num_1022() + Self::num_1()
    }
}
