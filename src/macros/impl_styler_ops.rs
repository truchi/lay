macro_rules! impl_styler_ops {
    ($Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?) => {
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Foreground(get_foreground_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Background(get_background_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Weighted(get_weight_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Slanted(get_slant_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Blinking(get_blink_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Inverted(get_invert_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Striked(get_strike_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Underlined(get_underline_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Overlined(get_overline_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Bordered(get_border_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Mul(mul), MulAssign(mul_assign), Foreground(get_foreground_mut));
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?: Div(div), DivAssign(div_assign), Background(get_background_mut));
    };
    (impl $Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?: $Rhs:ident($get_mut:ident)) => {
        impl $(<$($G $(: $B)?,)+>)? ::std::ops::Add<$crate::$Rhs> for $Type $(<$($G,)+>)? {
            type Output = Self;

            fn add(mut self, rhs: $crate::$Rhs) -> Self {
                <Self as ::std::ops::AddAssign<$crate::$Rhs>>::add_assign(&mut self, rhs);
                self
            }
        }

        impl $(<$($G $(: $B)?,)+>)? ::std::ops::AddAssign<$crate::$Rhs> for $Type $(<$($G,)+>)? {
            fn add_assign(&mut self, rhs: $crate::$Rhs) {
                *<Self as $crate::Styler>::$get_mut(self) = Some(rhs);
            }
        }
    };
    (
        impl
        $Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?:
        $Ops:ident($ops:ident),
        $OpsAssign:ident($ops_assign:ident),
        $Wrapper:ident($get_mut:ident)
    ) => {
        impl $(<$($G $(: $B)?,)+>)? ::std::ops::$Ops<$crate::Color> for $Type $(<$($G,)+>)? {
            type Output = Self;

            fn $ops(mut self, rhs: $crate::Color) -> Self {
                <Self as ::std::ops::$OpsAssign<$crate::Color>>::$ops_assign(&mut self, rhs);
                self
            }
        }

        impl $(<$($G $(: $B)?,)+>)? ::std::ops::$OpsAssign<$crate::Color> for $Type $(<$($G,)+>)? {
            fn $ops_assign(&mut self, rhs: $crate::Color) {
                *<Self as $crate::Styler>::$get_mut(self) = Some($crate::$Wrapper(rhs));
            }
        }
    };
}
