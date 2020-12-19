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

        $(impl<Rhs: Into<Self>> $Trait<Rhs> for $T {
            type Output = Self;
            fn $fn(self, rhs: Rhs) -> Option<Self::Output> { self.$fn(rhs.into()) }
        })*
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

/// [`Clamp`](crate::Clamp)ing values to bounds.
///
/// `NAN` panics in debug, unspecified behavior in release.
pub trait Clamp {
    /// Clamps `self` above `min`.
    fn clamp_min(self, min: Self) -> Self;

    /// Clamps `self` below `max`.
    fn clamp_max(self, max: Self) -> Self;

    /// Clamps `self` in `[min(a, b), max(a, b)]`.
    fn clamp(self, a: Self, b: Self) -> Self;
}

macro_rules! clamp {
    (int $($int:ty)*) => { $(
        impl Clamp for $int {
            fn clamp_min(self, other: Self) -> Self { self.max(other) }

            fn clamp_max(self, other: Self) -> Self { self.min(other) }

            fn clamp(self, a: Self, b: Self) -> Self {
                let (min, max) = if a < b { (a, b) } else { (b, a) };
                self.clamp_min(min).clamp_max(max)
            }
        }
    )* };
    (float $($float:ty)*) => { $(
        impl Clamp for $float {
            fn clamp_min(self, other: Self) -> Self {
                debug_assert!(self != <$float>::NAN && other != <$float>::NAN, "Clamp NAN");
                self.max(other)
            }

            fn clamp_max(self, other: Self) -> Self {
                debug_assert!(self != <$float>::NAN && other != <$float>::NAN, "Clamp NAN");
                self.min(other)
            }

            fn clamp(self, a: Self, b: Self) -> Self {
                debug_assert!(self != <$float>::NAN && a != <$float>::NAN && b != <$float>::NAN, "Clamp NAN");
                let (min, max) = if a < b { (a, b) } else { (b, a) };
                self.clamp_min(min).clamp_max(max)
            }
        }
    )* };
}

clamp!(int u8 i8 u16 i16 u32 i32 u128 i128 usize isize);
clamp!(float f32 f64);

/// [`Zero`](crate::Zero).
pub trait Zero {
    /// [`Zero::ZERO`](crate::Zero::ZERO).
    const ZERO: Self;
}

/// [`One`](crate::One).
pub trait One {
    /// [`One::ONE`](crate::One::ONE).
    const ONE: Self;
}

macro_rules! zero_one {
    (int $($T:ty)*) => { $(
        impl Zero for $T { const ZERO: Self = 0; }
        impl One  for $T { const ONE : Self = 1; }
    )* };
    (float $($T:ty)*) => { $(
        impl Zero for $T { const ZERO: Self = 0.; }
        impl One  for $T { const ONE : Self = 1.; }
    )* };
}

zero_one!(int u8 i8 u16 i16 u32 i32 u128 i128 usize isize);
zero_one!(float f32 f64);
