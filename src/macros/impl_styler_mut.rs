/// Implements `StylerMut`.
#[macro_export]
macro_rules! impl_styler_mut {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path)
        $({
            $foreground:tt $foreground_expr:expr,
            $background:tt $background_expr:expr,
            $weight:tt     $weight_expr:expr,
            $slant:tt      $slant_expr:expr,
            $blink:tt      $blink_expr:expr,
            $invert:tt     $invert_expr:expr,
            $strike:tt     $strike_expr:expr,
            $underline:tt  $underline_expr:expr,
            $overline:tt   $overline_expr:expr,
            $border:tt     $border_expr:expr,
        })?
        $(=> $styler:expr)?
    ) => {
        $crate::__impl_styler_mut!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self)
            $(=> $styler)?
            $({
                $foreground { let $foreground = $foreground.into(); $foreground_expr },
                $background { let $background = $background.into(); $background_expr },
                $weight     { let $weight     = $weight.into();     $weight_expr },
                $slant      { let $slant      = $slant.into();      $slant_expr },
                $blink      { let $blink      = $blink.into();      $blink_expr },
                $invert     { let $invert     = $invert.into();     $invert_expr },
                $strike     { let $strike     = $strike.into();     $strike_expr },
                $underline  { let $underline  = $underline.into();  $underline_expr },
                $overline   { let $overline   = $overline.into();   $overline_expr },
                $border     { let $border     = $border.into();     $border_expr },
            })?
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler_mut {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) => $styler:expr) => {
        $crate::__impl_styler_mut!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            foreground $crate::StylerMut::foreground_mut(&mut $styler, foreground),
            background $crate::StylerMut::background_mut(&mut $styler, background),
            weight     $crate::StylerMut::weight_mut    (&mut $styler, weight),
            slant      $crate::StylerMut::slant_mut     (&mut $styler, slant),
            blink      $crate::StylerMut::blink_mut     (&mut $styler, blink),
            invert     $crate::StylerMut::invert_mut    (&mut $styler, invert),
            strike     $crate::StylerMut::strike_mut    (&mut $styler, strike),
            underline  $crate::StylerMut::underline_mut (&mut $styler, underline),
            overline   $crate::StylerMut::overline_mut  (&mut $styler, overline),
            border     $crate::StylerMut::border_mut    (&mut $styler, border),
        });
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) {
        $foreground:tt $foreground_expr:expr,
        $background:tt $background_expr:expr,
        $weight:tt     $weight_expr:expr,
        $slant:tt      $slant_expr:expr,
        $blink:tt      $blink_expr:expr,
        $invert:tt     $invert_expr:expr,
        $strike:tt     $strike_expr:expr,
        $underline:tt  $underline_expr:expr,
        $overline:tt   $overline_expr:expr,
        $border:tt     $border_expr:expr,
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerMut for $Self {
            $crate::__impl_styler_mut!(
                foreground_mut($self, $foreground: Foreground) $foreground_expr
                background_mut($self, $background: Background) $background_expr
                weight_mut    ($self, $weight:     Weight)     $weight_expr
                slant_mut     ($self, $slant:      Slant)      $slant_expr
                blink_mut     ($self, $blink:      Blink)      $blink_expr
                invert_mut    ($self, $invert:     Invert)     $invert_expr
                strike_mut    ($self, $strike:     Strike)     $strike_expr
                underline_mut ($self, $underline:  Underline)  $underline_expr
                overline_mut  ($self, $overline:   Overline)   $overline_expr
                border_mut    ($self, $border:     Border)     $border_expr
            );
        }
    };

    ($($set:ident($self:tt, $attr:tt: $Attr:ident) $body:expr)*) => {
        $(fn $set(&mut self, $attr: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $body;
        })*
    };
}
