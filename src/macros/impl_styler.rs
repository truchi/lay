/// Implements `Styler`/`StylerMut`.
#[macro_export]
macro_rules! impl_styler {
    (
        // "" or "mut"
        $($mut:ident)?
        // Generics and corresponding bounds
        $(<$($G:ident $(: $($B:path)+)?,)+>)?
        // self argument and type
        ($self:tt: $Self:path)
        // Styler's Output
        $(-> $Output:path)?
        // One of:
            // List of arguments and setters to attributes
            $({
                ($foreground:tt) $foreground_expr:expr,
                ($background:tt) $background_expr:expr,
                ($weight:tt)     $weight_expr:expr,
                ($slant:tt)      $slant_expr:expr,
                ($blink:tt)      $blink_expr:expr,
                ($invert:tt)     $invert_expr:expr,
                ($strike:tt)     $strike_expr:expr,
                ($underline:tt)  $underline_expr:expr,
                ($overline:tt)   $overline_expr:expr,
                ($border:tt)     $border_expr:expr,
                $(
                    ($and:tt)    $and_expr:expr,
                    ($or:tt)     $or_expr:expr,
                    ($xor:tt)    $xor_expr:expr,
                    ($dedup:tt)  $dedup_expr:expr,
                                 $reset_expr:expr,
                )?
            })?
            // A getter to a Styler field
            $(=> $styler:expr)?
    ) => {
        $crate::priv_impl_styler!($($mut)? $(<$($G $(: $($B)+)?,)+>)? ($self: $Self) $(-> $Output)?
            $(=> $styler)?
            $({
                ($foreground) { let $foreground = $foreground.into(); $foreground_expr },
                ($background) { let $background = $background.into(); $background_expr },
                ($weight)     { let $weight     = $weight.into();     $weight_expr },
                ($slant)      { let $slant      = $slant.into();      $slant_expr },
                ($blink)      { let $blink      = $blink.into();      $blink_expr },
                ($invert)     { let $invert     = $invert.into();     $invert_expr },
                ($strike)     { let $strike     = $strike.into();     $strike_expr },
                ($underline)  { let $underline  = $underline.into();  $underline_expr },
                ($overline)   { let $overline   = $overline.into();   $overline_expr },
                ($border)     { let $border     = $border.into();     $border_expr },
                $(
                    ($and)    $and_expr,
                    ($or)     $or_expr,
                    ($xor)    $xor_expr,
                    ($dedup)  $dedup_expr,
                              $reset_expr,
                )?
            })?
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! priv_impl_styler {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) -> $Output:path => $styler:expr) => {
        $crate::priv_impl_styler!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) -> $Output {
            (foreground) { $crate::Styler::foreground($styler, foreground); $self },
            (background) { $crate::Styler::background($styler, background); $self },
            (weight)     { $crate::Styler::weight    ($styler, weight);     $self },
            (slant)      { $crate::Styler::slant     ($styler, slant);      $self },
            (blink)      { $crate::Styler::blink     ($styler, blink);      $self },
            (invert)     { $crate::Styler::invert    ($styler, invert);     $self },
            (strike)     { $crate::Styler::strike    ($styler, strike);     $self },
            (underline)  { $crate::Styler::underline ($styler, underline);  $self },
            (overline)   { $crate::Styler::overline  ($styler, overline);   $self },
            (border)     { $crate::Styler::border    ($styler, border);     $self },
            (and)        { $crate::Styler::and       ($styler, and);        $self },
            (or)         { $crate::Styler::or        ($styler, or);         $self },
            (xor)        { $crate::Styler::xor       ($styler, xor);        $self },
            (dedup)      { $crate::Styler::dedup     ($styler, dedup);      $self },
                         { $crate::Styler::reset     ($styler);             $self },
        });
    };
    (mut $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) => $styler:expr) => {
        $crate::priv_impl_styler!(mut $(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            (foreground) $crate::StylerMut::foreground_mut(&mut $styler, foreground),
            (background) $crate::StylerMut::background_mut(&mut $styler, background),
            (weight)     $crate::StylerMut::weight_mut    (&mut $styler, weight),
            (slant)      $crate::StylerMut::slant_mut     (&mut $styler, slant),
            (blink)      $crate::StylerMut::blink_mut     (&mut $styler, blink),
            (invert)     $crate::StylerMut::invert_mut    (&mut $styler, invert),
            (strike)     $crate::StylerMut::strike_mut    (&mut $styler, strike),
            (underline)  $crate::StylerMut::underline_mut (&mut $styler, underline),
            (overline)   $crate::StylerMut::overline_mut  (&mut $styler, overline),
            (border)     $crate::StylerMut::border_mut    (&mut $styler, border),
            (and)        $crate::StylerMut::and_mut       (&mut $styler, and),
            (or)         $crate::StylerMut::or_mut        (&mut $styler, or),
            (xor)        $crate::StylerMut::xor_mut       (&mut $styler, xor),
            (dedup)      $crate::StylerMut::dedup_mut     (&mut $styler, dedup),
                         $crate::StylerMut::reset_mut     (&mut $styler),
        });
    };

    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) -> $Output:path {
        ($foreground:tt) $foreground_expr:expr,
        ($background:tt) $background_expr:expr,
        ($weight:tt)     $weight_expr:expr,
        ($slant:tt)      $slant_expr:expr,
        ($blink:tt)      $blink_expr:expr,
        ($invert:tt)     $invert_expr:expr,
        ($strike:tt)     $strike_expr:expr,
        ($underline:tt)  $underline_expr:expr,
        ($overline:tt)   $overline_expr:expr,
        ($border:tt)     $border_expr:expr,
        $(
            ($and:tt)    $and_expr:expr,
            ($or:tt)     $or_expr:expr,
            ($xor:tt)    $xor_expr:expr,
            ($dedup:tt)  $dedup_expr:expr,
                         $reset_expr:expr,
        )?
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Self {
            type Output = $Output;

            $crate::priv_impl_styler!($self
                foreground($foreground: Foreground) $foreground_expr
                background($background: Background) $background_expr
                weight    ($weight:     Weight)     $weight_expr
                slant     ($slant:      Slant)      $slant_expr
                blink     ($blink:      Blink)      $blink_expr
                invert    ($invert:     Invert)     $invert_expr
                strike    ($strike:     Strike)     $strike_expr
                underline ($underline:  Underline)  $underline_expr
                overline  ($overline:   Overline)   $overline_expr
                border    ($border:     Border)     $border_expr
            );

            $crate::priv_impl_styler!($self
                get_foreground foreground($crate::Foreground($crate::Color::ResetColor))
                get_background background($crate::Background($crate::Color::ResetColor))
                get_weight weight($crate::Weight::ResetWeight)
                get_slant slant($crate::Slant::ResetSlant)
                get_blink blink($crate::Blink::ResetBlink)
                get_invert invert($crate::Invert::ResetInvert)
                get_strike strike($crate::Strike::ResetStrike)
                get_underline underline($crate::Underline::ResetUnderline)
                get_overline overline($crate::Overline::ResetOverline)
                get_border border($crate::Border::ResetBorder),
                $(
                    and   ($and  )  $and_expr,
                    or    ($or   )  $or_expr,
                    xor   ($xor  )  $xor_expr,
                    dedup ($dedup)  $dedup_expr,
                    reset ()        $reset_expr,
                )?
            );
        }
    };
    (mut $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) {
        ($foreground:tt) $foreground_expr:expr,
        ($background:tt) $background_expr:expr,
        ($weight:tt)     $weight_expr:expr,
        ($slant:tt)      $slant_expr:expr,
        ($blink:tt)      $blink_expr:expr,
        ($invert:tt)     $invert_expr:expr,
        ($strike:tt)     $strike_expr:expr,
        ($underline:tt)  $underline_expr:expr,
        ($overline:tt)   $overline_expr:expr,
        ($border:tt)     $border_expr:expr,
        $(
            ($and:tt)    $and_expr:expr,
            ($or:tt)     $or_expr:expr,
            ($xor:tt)    $xor_expr:expr,
            ($dedup:tt)  $dedup_expr:expr,
                         $reset_expr:expr,
        )?
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerMut for $Self {
            $crate::priv_impl_styler!(mut $self
                foreground_mut($foreground: Foreground) $foreground_expr
                background_mut($background: Background) $background_expr
                weight_mut    ($weight:     Weight)     $weight_expr
                slant_mut     ($slant:      Slant)      $slant_expr
                blink_mut     ($blink:      Blink)      $blink_expr
                invert_mut    ($invert:     Invert)     $invert_expr
                strike_mut    ($strike:     Strike)     $strike_expr
                underline_mut ($underline:  Underline)  $underline_expr
                overline_mut  ($overline:   Overline)   $overline_expr
                border_mut    ($border:     Border)     $border_expr
            );

            $crate::priv_impl_styler!(mut $self
                get_foreground foreground_mut($crate::Foreground($crate::Color::ResetColor))
                get_background background_mut($crate::Background($crate::Color::ResetColor))
                get_weight weight_mut($crate::Weight::ResetWeight)
                get_slant slant_mut($crate::Slant::ResetSlant)
                get_blink blink_mut($crate::Blink::ResetBlink)
                get_invert invert_mut($crate::Invert::ResetInvert)
                get_strike strike_mut($crate::Strike::ResetStrike)
                get_underline underline_mut($crate::Underline::ResetUnderline)
                get_overline overline_mut($crate::Overline::ResetOverline)
                get_border border_mut($crate::Border::ResetBorder),
                $(
                    and_mut   ($and  )  $and_expr,
                    or_mut    ($or   )  $or_expr,
                    xor_mut   ($xor  )  $xor_expr,
                    dedup_mut ($dedup)  $dedup_expr,
                    reset_mut ()        $reset_expr,
                )?
            );
        }
    };

    ($self:tt $($get:ident $set:ident($body:expr))*,) => {
        $crate::priv_impl_styler!($self _1 _2(_3),
            and(other) {
                $($self = $self.$set($self.$get().and(other.$get()));)*
                $self
            },
            or(other) {
                $($self = $self.$set($self.$get().or(other.$get()));)*
                $self
            },
            xor(other) {
                $($self = $self.$set($self.$get().xor(other.$get()));)*
                $self
            },
            dedup(before) {
                $(if $self.$get() == before.$get() {
                    $self = $self.$set(None);
                })*
                $self
            },
            reset() {
                $(if let Some(_) = $self.$get() {
                    $self = $self.$set(Some($body));
                })*
                $self
            },
        );
    };
    (mut $self:tt $($get:ident $set:ident($body:expr))*,) => {
        $crate::priv_impl_styler!(mut $self _1 _2(_3),
            and_mut(other) {
                $($self.$set($self.$get().and(other.$get()));)*
            },
            or_mut(other) {
                $($self.$set($self.$get().or(other.$get()));)*
            },
            xor_mut(other) {
                $($self.$set($self.$get().xor(other.$get()));)*
            },
            dedup_mut(before) {
                $(if $self.$get() == before.$get() {
                    $self.$set(None);
                })*
            },
            reset_mut() {
                $(if let Some(_) = $self.$get() {
                    $self.$set(Some($body));
                })*
            },
        );
    };

    ($self:tt $($set:ident($attr:tt: $Attr:ident) $body:expr)*) => {
        $(fn $set(self, $attr: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) -> Self::Output {
            #[allow(unused_mut)]
            let mut $self = self;
            $body
        })*
    };
    (mut $self:tt $($set:ident($attr:tt: $Attr:ident) $body:expr)*) => {
        $(fn $set(&mut self, $attr: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $body;
        })*
    };

    ($self:tt $($_1:ident $_2:ident($_3:expr))*, $($fn:ident ($($other:tt)?) $body:expr,)*) => {
        $(fn $fn(self $(, $other: &impl $crate::StylerIndex)?) -> Self::Output {
            #[allow(unused_mut)]
            let mut $self = self;
            $body
        })*
    };
    (mut $self:tt $($_1:ident $_2:ident($_3:expr))*, $($fn:ident ($($other:tt)?) $body:expr,)*) => {
        $(fn $fn(&mut self $(, $other: &impl $crate::StylerIndex)?) {
            let $self = self;
            $body
        })*
    };
}
