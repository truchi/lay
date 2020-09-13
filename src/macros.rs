#![allow(unused_macros)]

macro_rules! doc {
    ($($doc:expr)*, $item:item) => { doc!{impl concat![" ", $($doc,)*], $item} };
    (impl $doc:expr, $item:item) => { #[doc = $doc] $item };
}

macro_rules! impl_styler {
    ($Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?) => {
        impl_styler!($Type $(<$($G $(: $B)?,)+>)?,
            fields:
                foreground background weight slant blink invert strike underline overline border
        );
    };
    ($Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?,
        field: $field:ident
    ) => {
        impl_styler!(impl $Type $(<$($G $(: $B)?,)+>)?,
            field: $field,
            fields:
                foreground background weight slant blink invert strike underline overline border
        );
    };
    ($Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?,
        fields:
            $foreground:ident $background:ident $weight:ident $slant:ident $blink:ident
            $invert:ident $strike:ident $underline:ident $overline:ident $border:ident
    ) => {
        impl_styler!(impl $Type $(<$($G $(: $B)?,)+>)?,
            fields:
                $foreground $background $weight $slant $blink $invert $strike $underline $overline $border
        );
    };
    (impl $Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?,
        $(field: $field:ident,)?
        fields:
            $foreground:ident $background:ident $weight:ident $slant:ident $blink:ident
            $invert:ident $strike:ident $underline:ident $overline:ident $border:ident
    ) => {
        impl $(<$($G $(: $B)?,)+>)? $crate::Styler for $Type $(<$($G,)+>)? {
            impl_styler!(impl $($field)?: get_foreground get_foreground_mut $foreground Foreground);
            impl_styler!(impl $($field)?: get_background get_background_mut $background Background);
            impl_styler!(impl $($field)?: get_weight get_weight_mut $weight Weighted);
            impl_styler!(impl $($field)?: get_slant get_slant_mut $slant Slanted);
            impl_styler!(impl $($field)?: get_blink get_blink_mut $blink Blinking);
            impl_styler!(impl $($field)?: get_invert get_invert_mut $invert Inverted);
            impl_styler!(impl $($field)?: get_strike get_strike_mut $strike Striked);
            impl_styler!(impl $($field)?: get_underline get_underline_mut $underline Underlined);
            impl_styler!(impl $($field)?: get_overline get_overline_mut $overline Overlined);
            impl_styler!(impl $($field)?: get_border get_border_mut $border Bordered);
        }
    };
    (impl $($field:ident)?: $get:ident $get_mut:ident $field2:ident $ReturnType:ident) => {
        fn $get(&self) -> &::std::option::Option<$crate::$ReturnType> {
            &self$(.$field)?.$field2
        }

        fn $get_mut(&mut self) -> &mut ::std::option::Option<$crate::$ReturnType> {
            &mut self$(.$field)?.$field2
        }
    };
}

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
