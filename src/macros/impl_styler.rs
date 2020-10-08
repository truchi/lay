/// Implements `Styler`/`StylerMut`.
#[macro_export]
macro_rules! impl_styler {
    ($(
        // "" or "mut"
        $($mut:ident)?
        // Generics and corresponding bounds
        $(<($($bounds:tt)+)>)?
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
                $(&($and:tt)     $and_expr:expr,)?
                $(|($or:tt)      $or_expr:expr,)?
                $(^($xor:tt)     $xor_expr:expr,)?
                $(%($dedup:tt)   $dedup_expr:expr,)?
                $(!()            $reset_expr:expr,)?
            })?
            // A getter to a Styler field
            $(=> $styler:expr)?
    )+) => {
        $($crate::priv_impl_styler!($($mut)? $(<($($bounds)+)>)? ($self: $Self) $(-> $Output)?
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
                $(&($and)     $and_expr,)?
                $(|($or)      $or_expr,)?
                $(^($xor)     $xor_expr,)?
                $(%($dedup)   $dedup_expr,)?
                $(!()         $reset_expr,)?
            })?
        );)+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! priv_impl_styler {
    ($(<($($bounds:tt)+)>)? ($self:tt: $Self:path) -> $Output:path => $styler:expr) => {
        $crate::priv_impl_styler!($(<($($bounds)+)>)? ($self: $Self) -> $Output {
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
        });
    };
    (mut $(<($($bounds:tt)+)>)? ($self:tt: $Self:path) => $styler:expr) => {
        $crate::priv_impl_styler!(mut $(<($($bounds)+)>)? ($self: $Self) {
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
        });
    };

    ($(<($($bounds:tt)+)>)? ($self:tt: $Self:path) -> $Output:path {
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
        $(&($and:tt)     $and_expr:expr,)?
        $(|($or:tt)      $or_expr:expr,)?
        $(^($xor:tt)     $xor_expr:expr,)?
        $(%($dedup:tt)   $dedup_expr:expr,)?
        $(!()            $reset_expr:expr,)?
    }) => {
        impl $(<$($bounds)+>)? $crate::Styler for $Self {
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

            $(priv_impl_styler!($self and   $and   $and_expr);)?
            $(priv_impl_styler!($self or    $or    $or_expr);)?
            $(priv_impl_styler!($self xor   $xor   $xor_expr);)?
            $(priv_impl_styler!($self dedup $dedup $dedup_expr);)?
            $(priv_impl_styler!($self reset        $reset_expr);)?
        }
    };
    (mut $(<($($bounds:tt)+)>)? ($self:tt: $Self:path) {
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
        $(&($and:tt)     $and_expr:expr,)?
        $(|($or:tt)      $or_expr:expr,)?
        $(^($xor:tt)     $xor_expr:expr,)?
        $(%($dedup:tt)   $dedup_expr:expr,)?
        $(!()            $reset_expr:expr,)?
    }) => {
        impl $(<$($bounds)+>)? $crate::StylerMut for $Self {
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

            $(priv_impl_styler!(mut $self and_mut   $and   $and_expr);)?
            $(priv_impl_styler!(mut $self or_mut    $or    $or_expr);)?
            $(priv_impl_styler!(mut $self xor_mut   $xor   $xor_expr);)?
            $(priv_impl_styler!(mut $self dedup_mut $dedup $dedup_expr);)?
            $(priv_impl_styler!(mut $self reset_mut        $reset_expr);)?
        }
    };

    ($self:tt $($set:ident($attr:tt: $Attr:ident) $body:expr)*) => {
        $(fn $set(self, $attr: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) -> Self::Output {
            #[allow(unused)]
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

    ($self:tt dedup $arg:tt $expr:expr) => {
        fn dedup(self, $arg: &impl StylerIndex) -> Self
        where
            Self: Styler<Output = Self>
        {
            #[allow(unused)]
            let mut $self = self;
            $expr
        }
    };
    (mut $self:tt dedup_mut $arg:tt $expr:expr) => {
        fn dedup_mut(&mut self, $arg: &impl StylerIndex) {
            #[allow(unused)]
            let $self = self;
            $expr;
        }
    };

    ($self:tt reset $expr:expr) => {
        fn reset(self) -> Self
        where
            Self: Styler<Output = Self>
        {
            #[allow(unused)]
            let mut $self = self;
            $expr
        }
    };
    (mut $self:tt reset_mut $expr:expr) => {
        fn reset_mut(&mut self) {
            #[allow(unused)]
            let $self = self;
            $expr
        }
    };

    ($self:tt $fn:ident $arg:tt $expr:expr) => {
        fn $fn(self, $arg: &impl StylerIndex) -> <Self::Output as Styler>::Output
        where
            Self::Output: Styler<Output = Self::Output>,
        {
            #[allow(unused)]
            let mut $self = self;
            $expr
        }
    };
    (mut $self:tt $fn:ident $arg:tt $expr:expr) => {
        fn $fn(&mut self, $arg: &impl StylerIndex) {
            #[allow(unused)]
            let $self = self;
            $expr;
        }
    };
}
