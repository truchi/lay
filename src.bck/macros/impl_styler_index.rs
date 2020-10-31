/// Implements `StylerIndex`/ `StylerIndexMut`.
#[macro_export]
macro_rules! impl_styler_index {
    ($(
        // "" or "mut"
        $($mut:ident)?
        // Generics and corresponding bounds
        $(<($($bounds:tt)+)>)?
        // self argument and type
        ($self:tt: $Self:ty)
        // One of:
            // List of getters to attributes
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
            // A getter to a Styler field
            $(=> $styler:expr)?
    )+) => {
        $($crate::priv_impl_styler_index!($($mut)? $(<($($bounds)+)>)? ($self: $Self)
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
        );)+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! priv_impl_styler_index {
    ($(<($($bounds:tt)+)>)? ($self:tt: $Self:ty) => $styler:expr) => {
        $crate::priv_impl_styler_index!($(<($($bounds)+)>)? ($self: $Self) {
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
    (mut $(<($($bounds:tt)+)>)? ($self:tt: $Self:ty) => $styler:expr) => {
        $crate::priv_impl_styler_index!(mut $(<($($bounds)+)>)? ($self: $Self) {
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

    ($(<($($bounds:tt)+)>)? ($self:tt: $Self:ty) {
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
        impl $(<$($bounds)+>)? $crate::StylerIndex for $Self {
            $crate::priv_impl_styler_index!(
                get_foreground($self, Foreground) $foreground_expr,
                get_background($self, Background) $background_expr,
                get_weight    ($self, Weight)     $weight_expr,
                get_slant     ($self, Slant)      $slant_expr,
                get_blink     ($self, Blink)      $blink_expr,
                get_invert    ($self, Invert)     $invert_expr,
                get_strike    ($self, Strike)     $strike_expr,
                get_underline ($self, Underline)  $underline_expr,
                get_overline  ($self, Overline)   $overline_expr,
                get_border    ($self, Border)     $border_expr,
            );
        }
    };
    (mut $(<($($bounds:tt)+)>)? ($self:tt: $Self:ty) {
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
        impl $(<$($bounds)+>)? $crate::StylerIndexMut for $Self {
            $crate::priv_impl_styler_index!(mut
                get_foreground_mut($self, Foreground) $foreground_expr,
                get_background_mut($self, Background) $background_expr,
                get_weight_mut    ($self, Weight)     $weight_expr,
                get_slant_mut     ($self, Slant)      $slant_expr,
                get_blink_mut     ($self, Blink)      $blink_expr,
                get_invert_mut    ($self, Invert)     $invert_expr,
                get_strike_mut    ($self, Strike)     $strike_expr,
                get_underline_mut ($self, Underline)  $underline_expr,
                get_overline_mut  ($self, Overline)   $overline_expr,
                get_border_mut    ($self, Border)     $border_expr,
            );
        }
    };

    ($($get:ident($self:tt, $Attr:ident) $body:expr,)*) => {
        $(fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            #[allow(unused)]
            let $self = self;
            $body
        })*
    };
    (mut $($get:ident($self:tt, $Attr:ident) $body:expr,)*) => {
        $(fn $get(&mut self) -> &mut ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $body
        })*
    };
}
