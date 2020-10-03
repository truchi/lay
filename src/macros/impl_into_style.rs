/// Impl `Into` `Style`.
#[macro_export]
macro_rules! impl_into_style {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($Type:ty)) => {
        impl $(<$($G $(: $($B+)+,)?)+>)? ::std::convert::Into<$crate::Style> for $Type {
            fn into(self) -> $crate::Style {
                $crate::Style {
                    foreground: <Self as $crate::StylerIndex>::get_foreground(&self),
                    background: <Self as $crate::StylerIndex>::get_background(&self),
                    weight: <Self as $crate::StylerIndex>::get_weight(&self),
                    slant: <Self as $crate::StylerIndex>::get_slant(&self),
                    blink: <Self as $crate::StylerIndex>::get_blink(&self),
                    invert: <Self as $crate::StylerIndex>::get_invert(&self),
                    strike: <Self as $crate::StylerIndex>::get_strike(&self),
                    underline: <Self as $crate::StylerIndex>::get_underline(&self),
                    overline: <Self as $crate::StylerIndex>::get_overline(&self),
                    border: <Self as $crate::StylerIndex>::get_border(&self),
                }
            }
        }
    };
}
