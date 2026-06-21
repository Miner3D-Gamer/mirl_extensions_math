use crate::Sqrt;

/// Get the hypotenuse of a value
pub const trait Hypot {
    #[must_use]
    /// Get the hypotenuse of a value
    fn hypot(self, other: Self) -> Self;
}
const impl<
    T: Copy + [const] Sqrt + [const] core::ops::Add<Output = T> + [const] core::ops::Mul<Output = T>,
> Hypot for T
{
    fn hypot(self, other: Self) -> Self {
        (self * self + other * other).sqrt()
    }
}
