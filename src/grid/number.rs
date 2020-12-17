macro_rules! checked {
    ($($(#[$meta:meta])* $Trait:ident $fn:ident)*) => {
        checked!($($(#[$meta])* $Trait $fn [u8 i8 u16 i16 u32 i32 u128 i128 usize isize f32 f64])*);
    };
    ($($(#[$meta:meta])* $Trait:ident $fn:ident [$($T:ty)*])*) => { $(
        $(#[$meta])*
        pub trait $Trait<Rhs = Self> {
            /// The resulting type.
            type Output;
            $(#[$meta])*
            fn $fn(self, rhs: Rhs) -> Option<Self::Output>;
        }

        $(
            impl<Rhs: Into<Self>> $Trait<Rhs> for $T {
                type Output = Self;
                fn $fn(self, rhs: Rhs) -> Option<Self::Output> { self.$fn(rhs.into()) }
            }
            impl<Rhs: Into<Self>> $Trait<Rhs> for &$T {
                type Output = Self;
                fn $fn(self, rhs: Rhs) -> Option<Self::Output> { self.$fn(rhs.into()) }
            }
        )*
    )* };
}

checked!(
    /// Checked addition.
    CheckedAdd checked_add
    /// Checked substraction.
    CheckedSub checked_sub
    /// Checked multiplication.
    CheckedMul checked_mul
    /// Checked division.
    CheckedDiv checked_div
);
