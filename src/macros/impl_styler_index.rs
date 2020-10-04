/// Implements `StylerIndex`.
#[macro_export]
macro_rules! impl_styler_index {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path)
        $({
            $foreground_expr:expr,
            $background_expr:expr,
            $weight_expr:expr,
            $slant_expr:expr,
            $blink_expr:expr,
            $invert_expr:expr,
            $strike_expr:expr,
            $underline_expr:expr,
            $overline_expr:expr,
            $border_expr:expr,
        })?
        $(=> $styler:expr)?
    ) => {
        $crate::__impl_styler_index!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self)
            $(=> $styler)?
            $({
                $foreground_expr,
                $background_expr,
                $weight_expr,
                $slant_expr,
                $blink_expr,
                $invert_expr,
                $strike_expr,
                $underline_expr,
                $overline_expr,
                $border_expr,
            })?
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler_index {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path) => $styler:expr) => {
        $crate::__impl_styler_index!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            $crate::StylerIndex::get_foreground(&$styler),
            $crate::StylerIndex::get_background(&$styler),
            $crate::StylerIndex::get_weight(&$styler),
            $crate::StylerIndex::get_slant(&$styler),
            $crate::StylerIndex::get_blink(&$styler),
            $crate::StylerIndex::get_invert(&$styler),
            $crate::StylerIndex::get_strike(&$styler),
            $crate::StylerIndex::get_underline(&$styler),
            $crate::StylerIndex::get_overline(&$styler),
            $crate::StylerIndex::get_border(&$styler),
        });
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path) {
        $foreground_expr:expr,
        $background_expr:expr,
        $weight_expr:expr,
        $slant_expr:expr,
        $blink_expr:expr,
        $invert_expr:expr,
        $strike_expr:expr,
        $underline_expr:expr,
        $overline_expr:expr,
        $border_expr:expr,
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerIndex for $Self {
            $crate::__impl_styler_index!(
                get_foreground($self, Foreground) $foreground_expr
                get_background($self, Background) $background_expr
                get_weight    ($self, Weight)     $weight_expr
                get_slant     ($self, Slant)      $slant_expr
                get_blink     ($self, Blink)      $blink_expr
                get_invert    ($self, Invert)     $invert_expr
                get_strike    ($self, Strike)     $strike_expr
                get_underline ($self, Underline)  $underline_expr
                get_overline  ($self, Overline)   $overline_expr
                get_border    ($self, Border)     $border_expr
            );
        }
    };

    ($($get:ident($self:ident, $Attr:ident) $body:expr)*) => {
        $(fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $body
        })*
    };
}
