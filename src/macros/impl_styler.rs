/// Implements `Styler`.
#[macro_export]
macro_rules! impl_styler {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path)
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
        $crate::__impl_styler!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self)
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
macro_rules! __impl_styler {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path) => $styler:expr) => {
        $crate::__impl_styler!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            foreground { $crate::Styler::foreground($styler, foreground); $self },
            background { $crate::Styler::background($styler, background); $self },
            weight     { $crate::Styler::weight    ($styler, weight);     $self },
            slant      { $crate::Styler::slant     ($styler, slant);      $self },
            blink      { $crate::Styler::blink     ($styler, blink);      $self },
            invert     { $crate::Styler::invert    ($styler, invert);     $self },
            strike     { $crate::Styler::strike    ($styler, strike);     $self },
            underline  { $crate::Styler::underline ($styler, underline);  $self },
            overline   { $crate::Styler::overline  ($styler, overline);   $self },
            border     { $crate::Styler::border    ($styler, border);     $self },
        });
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path) {
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
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Self {
            $crate::__impl_styler!(
                foreground($self, $foreground: Foreground) $foreground_expr
                background($self, $background: Background) $background_expr
                weight    ($self, $weight:     Weight)     $weight_expr
                slant     ($self, $slant:      Slant)      $slant_expr
                blink     ($self, $blink:      Blink)      $blink_expr
                invert    ($self, $invert:     Invert)     $invert_expr
                strike    ($self, $strike:     Strike)     $strike_expr
                underline ($self, $underline:  Underline)  $underline_expr
                overline  ($self, $overline:   Overline)   $overline_expr
                border    ($self, $border:     Border)     $border_expr
            );
        }
    };

    ($($set:ident($self:ident, $attr:tt: $Attr:ident) $body:expr)*) => {
        $(fn $set(self, $attr: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) -> Self {
            #[allow(unused_mut)]
            let mut $self = self;
            $body
        })*
    };
}
