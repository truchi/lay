/// Implements `StylerIndexMut`.
#[macro_export]
macro_rules! impl_styler_index_mut {
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
        $crate::__impl_styler_index_mut!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self)
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
macro_rules! __impl_styler_index_mut {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path) => $styler:expr) => {
        $crate::__impl_styler_index_mut!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            $crate::StylerIndexMut::get_foreground_mut(&mut $styler),
            $crate::StylerIndexMut::get_background_mut(&mut $styler),
            $crate::StylerIndexMut::get_weight_mut    (&mut $styler),
            $crate::StylerIndexMut::get_slant_mut     (&mut $styler),
            $crate::StylerIndexMut::get_blink_mut     (&mut $styler),
            $crate::StylerIndexMut::get_invert_mut    (&mut $styler),
            $crate::StylerIndexMut::get_strike_mut    (&mut $styler),
            $crate::StylerIndexMut::get_underline_mut (&mut $styler),
            $crate::StylerIndexMut::get_overline_mut  (&mut $styler),
            $crate::StylerIndexMut::get_border_mut    (&mut $styler),
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
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerIndexMut for $Self {
            $crate::__impl_styler_index_mut!(get_foreground_mut($self, Foreground) $foreground_expr);
            $crate::__impl_styler_index_mut!(get_background_mut($self, Background) $background_expr);
            $crate::__impl_styler_index_mut!(get_weight_mut    ($self, Weight)     $weight_expr);
            $crate::__impl_styler_index_mut!(get_slant_mut     ($self, Slant)      $slant_expr);
            $crate::__impl_styler_index_mut!(get_blink_mut     ($self, Blink)      $blink_expr);
            $crate::__impl_styler_index_mut!(get_invert_mut    ($self, Invert)     $invert_expr);
            $crate::__impl_styler_index_mut!(get_strike_mut    ($self, Strike)     $strike_expr);
            $crate::__impl_styler_index_mut!(get_underline_mut ($self, Underline)  $underline_expr);
            $crate::__impl_styler_index_mut!(get_overline_mut  ($self, Overline)   $overline_expr);
            $crate::__impl_styler_index_mut!(get_border_mut    ($self, Border)     $border_expr);
        }
    };

    ($get:ident($self:ident, $Attr:ident) $body:expr) => {
        fn $get(&mut self) -> &mut ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $body
        }
    };
}
