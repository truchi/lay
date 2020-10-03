/// Impl `Styler`.
#[macro_export]
macro_rules! impl_styler {
    ([StylerIndex] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $foreground_block:block
        $background_block:block
        $weight_block:block
        $slant_block:block
        $blink_block:block
        $invert_block:block
        $strike_block:block
        $underline_block:block
        $overline_block:block
        $border_block:block
    }) => {
        $crate::__impl_styler!([StylerIndex] $(<$($G $(: $($B +)+)?,)+>)? ($self: $Type) {
            $foreground_block
            $background_block
            $weight_block
            $slant_block
            $blink_block
            $invert_block
            $strike_block
            $underline_block
            $overline_block
            $border_block
        });
    };
    ([StylerIndex] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) => $style:expr) => {
        $crate::__impl_styler!([StylerIndex] $(<$($G $(: $($B +)+)?,)+>)? ($self: $Type) {
            { $style.get_foreground() }
            { $style.get_background() }
            { $style.get_weight() }
            { $style.get_slant() }
            { $style.get_blink() }
            { $style.get_invert() }
            { $style.get_strike() }
            { $style.get_underline() }
            { $style.get_overline() }
            { $style.get_border() }
        });
    };
    ([StylerIndexMut] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $foreground_block:block
        $background_block:block
        $weight_block:block
        $slant_block:block
        $blink_block:block
        $invert_block:block
        $strike_block:block
        $underline_block:block
        $overline_block:block
        $border_block:block
    }) => {
        $crate::__impl_styler!([StylerIndexMut] $(<$($G $(: $($B +)+)?,)+>)? ($self: $Type) {
            $foreground_block
            $background_block
            $weight_block
            $slant_block
            $blink_block
            $invert_block
            $strike_block
            $underline_block
            $overline_block
            $border_block
        });
    };
    ([Styler] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        ($foreground:tt: Foreground) $foreground_block:block
        ($background:tt: Background) $background_block:block
        ($weight:tt: Weight) $weight_block:block
        ($slant:tt: Slant) $slant_block:block
        ($blink:tt: Blink) $blink_block:block
        ($invert:tt: Invert) $invert_block:block
        ($strike:tt: Strike) $strike_block:block
        ($underline:tt: Underline) $underline_block:block
        ($overline:tt: Overline) $overline_block:block
        ($border:tt: Border) $border_block:block
    }) => {
        $crate::__impl_styler!([Styler] $(<$($G $(: $($B +)+)?,)+>)? ($self: $Type) {
            ($foreground: Foreground) { let $foreground = $foreground.into(); $foreground_block }
            ($background: Background) { let $background = $background.into(); $background_block }
            ($weight: Weight) { let $weight  = $weight.into(); $weight_block }
            ($slant: Slant) { let $slant = $slant.into(); $slant_block }
            ($blink: Blink) { let $blink = $blink.into(); $blink_block }
            ($invert: Invert) { let $invert = $invert.into(); $invert_block }
            ($strike: Strike) { let $strike = $strike.into(); $strike_block }
            ($underline: Underline) { let $underline = $underline.into(); $underline_block }
            ($overline: Overline) { let $overline = $overline.into(); $overline_block }
            ($border: Border) { let $border = $border.into(); $border_block }
        });
    };
    ([Styler] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) => $style:expr) => {
        $crate::__impl_styler!([Styler] $(<$($G $(: $($B +)+)?,)+>)? ($self: $Type) {
            ($foreground: Foreground) { $style.foreground(foreground); }
            ($background: Background) { $style.background(background); }
            ($weight: Weight) { $style.weight(weight); }
            ($slant: Slant) { $style.slant(slant); }
            ($blink: Blink) { $style.blink(blink); }
            ($invert: Invert) { $style.invert(invert); }
            ($strike: Strike) { $style.strike(strike); }
            ($underline: Underline) { $style.underline(underline); }
            ($overline: Overline) { $style.overline(overline); }
            ($border: Border) { $style.border(border); }
        });
    };
    ([StylerMut] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        ($foreground:tt: Foreground) $foreground_block:block
        ($background:tt: Background) $background_block:block
        ($weight:tt: Weight) $weight_block:block
        ($slant:tt: Slant) $slant_block:block
        ($blink:tt: Blink) $blink_block:block
        ($invert:tt: Invert) $invert_block:block
        ($strike:tt: Strike) $strike_block:block
        ($underline:tt: Underline) $underline_block:block
        ($overline:tt: Overline) $overline_block:block
        ($border:tt: Border) $border_block:block
    }) => {
        $crate::__impl_styler!([StylerMut] $(<$($G $(: $($B +)+)?,)+>)? ($self: $Type) {
            ($foreground: Foreground) { let $foreground = $foreground.into(); $foreground_block }
            ($background: Background) { let $background = $background.into(); $background_block }
            ($weight: Weight) { let $weight  = $weight.into(); $weight_block }
            ($slant: Slant) { let $slant = $slant.into(); $slant_block }
            ($blink: Blink) { let $blink = $blink.into(); $blink_block }
            ($invert: Invert) { let $invert = $invert.into(); $invert_block }
            ($strike: Strike) { let $strike = $strike.into(); $strike_block }
            ($underline: Underline) { let $underline = $underline.into(); $underline_block }
            ($overline: Overline) { let $overline = $overline.into(); $overline_block }
            ($border: Border) { let $border = $border.into(); $border_block }
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler {
    ([StylerIndex] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $foreground_block:block
        $background_block:block
        $weight_block:block
        $slant_block:block
        $blink_block:block
        $invert_block:block
        $strike_block:block
        $underline_block:block
        $overline_block:block
        $border_block:block
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerIndex for $Type {
            __impl_styler!([get] get_foreground($self, Foreground) $foreground_block);
            __impl_styler!([get] get_background($self, Background) $background_block);
            __impl_styler!([get] get_weight($self, Weight) $weight_block);
            __impl_styler!([get] get_slant($self, Slant) $slant_block);
            __impl_styler!([get] get_blink($self, Blink) $blink_block);
            __impl_styler!([get] get_invert($self, Invert) $invert_block);
            __impl_styler!([get] get_strike($self, Strike) $strike_block);
            __impl_styler!([get] get_underline($self, Underline) $underline_block);
            __impl_styler!([get] get_overline($self, Overline) $overline_block);
            __impl_styler!([get] get_border($self, Border) $border_block);
        }
    };
    ([StylerIndexMut] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $foreground_block:block
        $background_block:block
        $weight_block:block
        $slant_block:block
        $blink_block:block
        $invert_block:block
        $strike_block:block
        $underline_block:block
        $overline_block:block
        $border_block:block
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerIndexMut for $Type {
            __impl_styler!([get mut] get_foreground_mut($self, Foreground) $foreground_block);
            __impl_styler!([get mut] get_background_mut($self, Background) $background_block);
            __impl_styler!([get mut] get_weight_mut($self, Weight) $weight_block);
            __impl_styler!([get mut] get_slant_mut($self, Slant) $slant_block);
            __impl_styler!([get mut] get_blink_mut($self, Blink) $blink_block);
            __impl_styler!([get mut] get_invert_mut($self, Invert) $invert_block);
            __impl_styler!([get mut] get_strike_mut($self, Strike) $strike_block);
            __impl_styler!([get mut] get_underline_mut($self, Underline) $underline_block);
            __impl_styler!([get mut] get_overline_mut($self, Overline) $overline_block);
            __impl_styler!([get mut] get_border_mut($self, Border) $border_block);
        }
    };
    ([Styler] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        ($foreground:tt: Foreground) $foreground_block:block
        ($background:tt: Background) $background_block:block
        ($weight:tt: Weight) $weight_block:block
        ($slant:tt: Slant) $slant_block:block
        ($blink:tt: Blink) $blink_block:block
        ($invert:tt: Invert) $invert_block:block
        ($strike:tt: Strike) $strike_block:block
        ($underline:tt: Underline) $underline_block:block
        ($overline:tt: Overline) $overline_block:block
        ($border:tt: Border) $border_block:block
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Type {
            __impl_styler!([set] foreground($self, $foreground: Foreground) $foreground_block);
            __impl_styler!([set] background($self, $background: Background) $background_block);
            __impl_styler!([set] weight($self, $weight: Weight) $weight_block);
            __impl_styler!([set] slant($self, $slant: Slant) $slant_block);
            __impl_styler!([set] blink($self, $blink: Blink) $blink_block);
            __impl_styler!([set] invert($self, $invert: Invert) $invert_block);
            __impl_styler!([set] strike($self, $strike: Strike) $strike_block);
            __impl_styler!([set] underline($self, $underline: Underline) $underline_block);
            __impl_styler!([set] overline($self, $overline: Overline) $overline_block);
            __impl_styler!([set] border($self, $border: Border) $border_block);
        }
    };
    ([StylerMut] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        ($foreground:tt: Foreground) $foreground_block:block
        ($background:tt: Background) $background_block:block
        ($weight:tt: Weight) $weight_block:block
        ($slant:tt: Slant) $slant_block:block
        ($blink:tt: Blink) $blink_block:block
        ($invert:tt: Invert) $invert_block:block
        ($strike:tt: Strike) $strike_block:block
        ($underline:tt: Underline) $underline_block:block
        ($overline:tt: Overline) $overline_block:block
        ($border:tt: Border) $border_block:block
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerMut for $Type {
            __impl_styler!([set mut] foreground_mut($self, $foreground: Foreground) $foreground_block);
            __impl_styler!([set mut] background_mut($self, $background: Background) $background_block);
            __impl_styler!([set mut] weight_mut($self, $weight: Weight) $weight_block);
            __impl_styler!([set mut] slant_mut($self, $slant: Slant) $slant_block);
            __impl_styler!([set mut] blink_mut($self, $blink: Blink) $blink_block);
            __impl_styler!([set mut] invert_mut($self, $invert: Invert) $invert_block);
            __impl_styler!([set mut] strike_mut($self, $strike: Strike) $strike_block);
            __impl_styler!([set mut] underline_mut($self, $underline: Underline) $underline_block);
            __impl_styler!([set mut] overline_mut($self, $overline: Overline) $overline_block);
            __impl_styler!([set mut] border_mut($self, $border: Border) $border_block);
        }
    };
    ([get] $get:ident($self:ident, $Attr:ident) $body:block) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $body
        }
    };
    ([get mut] $get:ident($self:ident, $Attr:ident) $body:block) => {
        fn $get(&mut self) -> &mut ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $body
        }
    };
    ([set] $set:ident($self:ident, $attr:tt: $Attr:ident) $body:block) => {
        fn $set(self, $attr: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) -> Self {
            let mut $self = self;
            $body
        }
    };
    ([set mut] $set:ident($self:ident, $attr:tt: $Attr:ident) $body:block) => {
        fn $set(&mut self, $attr: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $body;
        }
    };
}
