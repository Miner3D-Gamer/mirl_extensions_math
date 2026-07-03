use mirl_extensions_core::One;

/// A single byte (8 bits)
///
/// 1024 B make one [`KiloByte`]
pub trait Byte {
    /// One byte is 1 B
    const B: Self;
}
/// A kilobyte is 1024 [`Byte`]s
///
/// One kilobyte is 1024 B = 8192 bits, and 1024 KB make one [`MegaByte`]
pub trait KiloByte {
    /// One kilobyte is 1024 B
    const KB: Self;
}
/// A megabyte is 1024 [`KiloByte`]s
///
/// One megabyte is 1024 KB = 1,048,576 B, and 1024 MB make one [`GigaByte`]
pub trait MegaByte {
    /// One megabyte is 1024 KB
    const MB: Self;
}
/// A gigabyte is 1024 [`MegaByte`]s
///
/// One gigabyte is 1024 MB = 1,073,741,824 B, and 1024 GB make one [`TeraByte`]
pub trait GigaByte {
    /// One gigabyte is 1024 MB
    const GB: Self;
}
/// A terabyte is 1024 [`GigaByte`]s
///
/// One terabyte is 1024 GB = 1,099,511,627,776 B, and 1024 TB make one [`PetaByte`]
pub trait TeraByte {
    /// One terabyte is 1024 GB
    const TB: Self;
}
/// A petabyte is 1024 [`TeraByte`]s
///
/// One petabyte is 1024 TB = 1,125,899,906,842,624 B, and 1024 PB make one [`ExaByte`]
pub trait PetaByte {
    /// One petabyte is 1024 TB
    const PB: Self;
}
/// An exabyte is 1024 [`PetaByte`]s
///
/// One exabyte is 1024 PB = 1,152,921,504,606,846,976 B, and 1024 EB make one [`ZettaByte`]
pub trait ExaByte {
    /// One exabyte is 1024 PB
    const EB: Self;
}
/// A zettabyte is 1024 [`ExaByte`]s
///
/// One zettabyte is 1024 EB = 1,180,591,620,717,411,303,424 B, and 1024 ZB make one [`YottaByte`]
pub trait ZettaByte {
    /// One zettabyte is 1024 EB
    const ZB: Self;
}
/// A yottabyte is 1024 [`ZettaByte`]s
///
/// One yottabyte is 1024 ZB = 1,208,925,819,614,629,174,706,176 B, and 1024 YB make one [`RonnaByte`]
pub trait YottaByte {
    /// One yottabyte is 1024 ZB
    const YB: Self;
}
/// A ronnabyte is 1024 [`YottaByte`]s
///
/// One ronnabyte is 1024 YB = 1,237,940,039,285,380,274,899,124,224 B, and 1024 RB make one [`QuettaByte`]
pub trait RonnaByte {
    /// One ronnabyte is 1024 YB
    const RB: Self;
}
/// A quettabyte is 1024 [`RonnaByte`]s
///
/// One quettabyte is 1024 RB = 1,267,650,600,228,229,401,496,703,205,376 B, and 1024 QB make one [`BrontoByte`]
pub trait QuettaByte {
    /// One quettabyte is 1024 RB
    const QB: Self;
}
/// A brontobyte is 1024 [`QuettaByte`]s
///
/// One brontobyte is 1024 QB = 1,298,074,214,633,706,907,132,624,082,305,024 B, and 1024 BB make one [`GeopByte`]
pub trait BrontoByte {
    /// One brontobyte is 1024 QB
    const BB: Self;
}
/// A geopbyte is 1024 [`BrontoByte`]s
///
/// One geopbyte is 1024 BB = 1,329,227,995,784,915,872,903,807,060,280,344,576 B, and 1024 GPB make one [`XenottaByte`]
pub trait GeopByte {
    /// One geopbyte is 1024 BB
    const GPB: Self;
}
/// A xenottabyte is 1024 [`GeopByte`]s
///
/// One xenottabyte is 1024 GPB = 1,361,129,467,683,678,213,509,615,783,783,677,824 B
pub trait XenottaByte {
    /// One xenottabyte is 1024 GPB
    const XAB: Self;
}

impl<T: const One> Byte for T {
    const B: Self = T::one();
}
