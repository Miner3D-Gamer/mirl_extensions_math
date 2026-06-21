/// Normalize a vector
pub const trait NormalizeVector<T> {
    #[must_use]
    /// Normalize the internal vector and return it
    fn normalize_vector(self) -> Self;
    /// Normalize the internal vector
    fn set_normalize_vector(&mut self);
}
const impl<
    T: [const] std::ops::Mul<Output = T>
        + [const] std::ops::Add<Output = T>
        + [const] std::ops::Div<Output = T>
        + [const] crate::Abs
        + [const] crate::Sqrt
        + Copy,
> NormalizeVector<T> for (T, T, T)
{
    default fn normalize_vector(self) -> Self {
        let v = (self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
            .abs()
            .sqrt();
        (self.0 / v, self.1 / v, self.2 / v)
    }
    default fn set_normalize_vector(&mut self) {
        let v = (self.0 * self.0 + self.1 * self.1 + self.2 * self.2)
            .abs()
            .sqrt();
        self.0 = self.0 / v;
        self.1 = self.1 / v;
        self.2 = self.2 / v;
    }
}
