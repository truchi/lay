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
