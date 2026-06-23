use mirl_extensions_core::impl_trait;

/// Get the next bigger/smaller value that can be represented
pub const trait NextUpDown {
    #[must_use]
    /// The next bigger value
    fn next_up(self) -> Self;
    #[must_use]
    /// The next smaller value
    fn next_down(self) -> Self;
}

impl_trait!(NextUpDown {fn next_down(self)->Self{self.next_down()}fn next_up(self)->Self{self.next_up()}} for  f16,f32,f64,f128 );

impl_trait!(NextUpDown {
    /// The next bigger value, stops at the maximum value the type can represent
    fn next_down(self)->Self{self.saturating_sub(1)}
    /// The next smaller value, stops at the minimum value the type can represent
    fn next_up(self)->Self{self.saturating_add(1)}
} for  u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize );
