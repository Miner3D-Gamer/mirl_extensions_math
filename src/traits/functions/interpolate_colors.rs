use mirl_extensions_conversion::TryIntoPatch;
use mirl_graphics::prelude::*;

use crate::InterpolateAsInterpolator;

/// Interpolate between 2 values with Self being the progress
pub const trait InterpolateColorAsInterpolator {
    /// Interpolate between 2 values with Self being the progress
    fn interpolate_colors(self, val1: u32, val2: u32) -> u32;
    /// Interpolate between 2 values with Self being the progress
    fn interpolate_colors_and_alpha(self, val1: u32, val2: u32, alpha: u32) -> u32;
}
/// Interpolate between this color and another color
///
/// Analogous to [`InterpolateBetween`] but specialized for RGBA colors packed as `u32`.
pub const trait InterpolateColorBetween<P> {
    /// Interpolate between this color and another, including alpha channel
    fn interpolate_color_with(self, other: u32, progress: P) -> u32;
    /// Interpolate between this color and another, overriding the alpha channel with a fixed value
    fn interpolate_color_with_alpha(self, other: u32, progress: P, alpha: u32) -> u32;
}

const impl<Progress: [const] InterpolateColorAsInterpolator> InterpolateColorBetween<Progress>
    for u32
{
    fn interpolate_color_with(self, other: u32, progress: Progress) -> u32 {
        progress.interpolate_colors(self, other)
    }

    fn interpolate_color_with_alpha(self, other: u32, progress: Progress, alpha: u32) -> u32 {
        progress.interpolate_colors_and_alpha(self, other, alpha)
    }
}

const impl<S: [const] InterpolateAsInterpolator<u32, u32, S> + Copy + [const] TryIntoPatch<u32>>
    InterpolateColorAsInterpolator for S
{
    fn interpolate_colors(self, val1: u32, val2: u32) -> u32 {
        (
            self.interpolate_values(val1.red(), val2.red())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.green(), val2.green())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.blue(), val2.blue())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.alpha(), val2.alpha())
                .try_into_value()
                .unwrap_or_default(),
        )
            .pack_rgba_u32()
    }
    fn interpolate_colors_and_alpha(self, val1: u32, val2: u32, alpha: u32) -> u32 {
        (
            self.interpolate_values(val1.red(), val2.red())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.green(), val2.green())
                .try_into_value()
                .unwrap_or_default(),
            self.interpolate_values(val1.blue(), val2.blue())
                .try_into_value()
                .unwrap_or_default(),
            alpha,
        )
            .pack_rgba_u32()
    }
}
