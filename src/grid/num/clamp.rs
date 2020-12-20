/// [`Clamp`](crate::Clamp)ing values to bounds.
///
/// `NAN` panics in debug, unspecified behavior in release.
pub trait Clamp<T = Self> {
    /// The resulting type after clamping.
    type Output;

    /// Clamps `self` above `min`.
    fn clamp_min(self, min: T) -> Self::Output;

    /// Clamps `self` below `max`.
    fn clamp_max(self, max: T) -> Self::Output;

    /// Clamps `self` in `[min(a, b), max(a, b)]`.
    fn clamp(self, a: T, b: T) -> Self::Output;
}

macro_rules! clamp {
    (int $($T:ty)*) => { $(
        impl Clamp for $T {
            type Output = Self;

            doc!("```\n"
                "# use lay::*;\n"
                "assert!(10_" s!($T) ".clamp_min(5_" s!($T) ") == 10_" s!($T) ");\n"
                "assert!(5_" s!($T) ".clamp_min(10_" s!($T) ") == 10_" s!($T) ");\n"
                "```\n",
            fn clamp_min(self, other: Self) -> Self { self.max(other) });

            doc!("```\n"
                "# use lay::*;\n"
                "assert!(10_" s!($T) ".clamp_max(5_" s!($T) ") == 5_" s!($T) ");\n"
                "assert!(5_" s!($T) ".clamp_max(10_" s!($T) ") == 5_" s!($T) ");\n"
                "```\n",
            fn clamp_max(self, other: Self) -> Self { self.min(other) });

            doc!("```\n"
                "# use lay::*;\n"
                "assert!(Clamp::clamp(0_" s!($T) ", 5_" s!($T) ", 15_" s!($T) ") == 5_" s!($T) ");\n"
                "assert!(Clamp::clamp(10_" s!($T) ", 5_" s!($T) ", 15_" s!($T) ") == 10_" s!($T) ");\n"
                "assert!(Clamp::clamp(20_" s!($T) ", 5_" s!($T) ", 15_" s!($T) ") == 15_" s!($T) ");\n"
                "\n"
                "assert!(Clamp::clamp(0_" s!($T) ", 15_" s!($T) ", 5_" s!($T) ") == 5_" s!($T) ");\n"
                "assert!(Clamp::clamp(10_" s!($T) ", 15_" s!($T) ", 5_" s!($T) ") == 10_" s!($T) ");\n"
                "assert!(Clamp::clamp(20_" s!($T) ", 15_" s!($T) ", 5_" s!($T) ") == 15_" s!($T) ");\n"
                "```\n",
            fn clamp(self, a: Self, b: Self) -> Self {
                let (min, max) = if a < b { (a, b) } else { (b, a) };
                self.clamp_min(min).clamp_max(max)
            });
        }
    )* };
    (float $($T:ty)*) => { $(
        impl Clamp for $T {
            type Output = Self;

            doc!("```\n"
                "# use lay::*;\n"
                "assert!(10.0_" s!($T) ".clamp_min(5.0_" s!($T) ") == 10.0_" s!($T) ");\n"
                "assert!(5.0_" s!($T) ".clamp_min(10.0_" s!($T) ") == 10.0_" s!($T) ");\n"
                "```\n",
            fn clamp_min(self, other: Self) -> Self {
                debug_assert!(self != <$T>::NAN && other != <$T>::NAN, "Clamp NAN");
                self.max(other)
            });

            doc!("```\n"
                "# use lay::*;\n"
                "assert!(10.0_" s!($T) ".clamp_max(5.0_" s!($T) ") == 5.0_" s!($T) ");\n"
                "assert!(5.0_" s!($T) ".clamp_max(10.0_" s!($T) ") == 5.0_" s!($T) ");\n"
                "```\n",
            fn clamp_max(self, other: Self) -> Self {
                debug_assert!(self != <$T>::NAN && other != <$T>::NAN, "Clamp NAN");
                self.min(other)
            });

            doc!("```\n"
                "# use lay::*;\n"
                "assert!(Clamp::clamp(0.0_" s!($T) ", 5_" s!($T) ", 15_" s!($T) ") == 5.0_" s!($T) ");\n"
                "assert!(Clamp::clamp(10.0_" s!($T) ", 5_" s!($T) ", 15_" s!($T) ") == 10.0_" s!($T) ");\n"
                "assert!(Clamp::clamp(20.0_" s!($T) ", 5_" s!($T) ", 15_" s!($T) ") == 15.0_" s!($T) ");\n"
                "\n"
                "assert!(Clamp::clamp(0.0_" s!($T) ", 15.0_" s!($T) ", 5.0_" s!($T) ") == 5.0_" s!($T) ");\n"
                "assert!(Clamp::clamp(10.0_" s!($T) ", 15.0_" s!($T) ", 5.0_" s!($T) ") == 10.0_" s!($T) ");\n"
                "assert!(Clamp::clamp(20.0_" s!($T) ", 15.0_" s!($T) ", 5.0_" s!($T) ") == 15.0_" s!($T) ");\n"
                "```\n",
            fn clamp(self, a: Self, b: Self) -> Self {
                debug_assert!(self != <$T>::NAN && a != <$T>::NAN && b != <$T>::NAN, "Clamp NAN");
                let (min, max) = if a < b { (a, b) } else { (b, a) };
                self.clamp_min(min).clamp_max(max)
            });
        }
    )* };
}

clamp!(int u8 i8 u16 i16 u32 i32 u128 i128 usize isize);
clamp!(float f32 f64);
