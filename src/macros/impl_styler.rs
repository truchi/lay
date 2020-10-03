/// Impl `Styler`.
#[macro_export]
macro_rules! impl_styler {
    (($self:ident: $Self:ty) {
        $(
            [StylerIndex] $(<$($G1:ident $(: $($B1:path)+)?,)+>)?
            $(=> $styler1:expr,)?
            $({
                $foreground_block1:block
                $background_block1:block
                $weight_block1:block
                $slant_block1:block
                $blink_block1:block
                $invert_block1:block
                $strike_block1:block
                $underline_block1:block
                $overline_block1:block
                $border_block1:block
            })?
        )?
        $(
            [StylerIndexMut] $(<$($G2:ident $(: $($B2:path)+)?,)+>)?
            $(=> $styler2:expr,)?
            $({
                $foreground_block2:block
                $background_block2:block
                $weight_block2:block
                $slant_block2:block
                $blink_block2:block
                $invert_block2:block
                $strike_block2:block
                $underline_block2:block
                $overline_block2:block
                $border_block2:block
            })?
        )?
        $(
            [Styler] $(<$($G3:ident $(: $($B3:path)+)?,)+>)?
            $(=> $styler3:expr,)?
            $({
                $foreground3:tt $foreground_block3:block
                $background3:tt $background_block3:block
                $weight3:tt     $weight_block3:block
                $slant3:tt      $slant_block3:block
                $blink3:tt      $blink_block3:block
                $invert3:tt     $invert_block3:block
                $strike3:tt     $strike_block3:block
                $underline3:tt  $underline_block3:block
                $overline3:tt   $overline_block3:block
                $border3:tt     $border_block3:block
            })?
        )?
        $(
            [StylerMut] $(<$($G4:ident $(: $($B4:path)+)?,)+>)?
            $(=> $styler4:expr,)?
            $({
                $foreground4:tt $foreground_block4:block
                $background4:tt $background_block4:block
                $weight4:tt     $weight_block4:block
                $slant4:tt      $slant_block4:block
                $blink4:tt      $blink_block4:block
                $invert4:tt     $invert_block4:block
                $strike4:tt     $strike_block4:block
                $underline4:tt  $underline_block4:block
                $overline4:tt   $overline_block4:block
                $border4:tt     $border_block4:block
            })?
        )?
    }) => {
        $(
            $crate::__impl_styler!([StylerIndex] $(<$($G1 $(: $($B1)+)?,)+>)? ($self: $Self)
            $(=> $styler1)?
            $({
                $foreground_block1
                $background_block1
                $weight_block1
                $slant_block1
                $blink_block1
                $invert_block1
                $strike_block1
                $underline_block1
                $overline_block1
                $border_block1
            })?);
        )?
        $(
            $crate::__impl_styler!([StylerIndexMut] $(<$($G2 $(: $($B2)+)?,)+>)? ($self: $Self)
            $(=> $styler2)?
            $({
                $foreground_block2
                $background_block2
                $weight_block2
                $slant_block2
                $blink_block2
                $invert_block2
                $strike_block2
                $underline_block2
                $overline_block2
                $border_block2
            })?);
        )?
        $(
            $crate::__impl_styler!([Styler] $(<$($G3 $(: $($B3)+)?,)+>)? ($self: $Self)
            $(=> $styler3)?
            $({
                $foreground3 { let $foreground3 = $foreground3.into(); $foreground_block3 }
                $background3 { let $background3 = $background3.into(); $background_block3 }
                $weight3     { let $weight3     = $weight3.into();     $weight_block3 }
                $slant3      { let $slant3      = $slant3.into();      $slant_block3 }
                $blink3      { let $blink3      = $blink3.into();      $blink_block3 }
                $invert3     { let $invert3     = $invert3.into();     $invert_block3 }
                $strike3     { let $strike3     = $strike3.into();     $strike_block3 }
                $underline3  { let $underline3  = $underline3.into();  $underline_block3 }
                $overline3   { let $overline3   = $overline3.into();   $overline_block3 }
                $border3     { let $border3     = $border3.into();     $border_block3 }
            })?);
        )?
        $(
            $crate::__impl_styler!([StylerMut] $(<$($G4 $(: $($B4)+)?,)+>)? ($self: $Self)
            $(=> $styler4)?
            $({
                $foreground4 { let $foreground4 = $foreground4.into(); $foreground_block4 }
                $background4 { let $background4 = $background4.into(); $background_block4 }
                $weight4     { let $weight4     = $weight4.into();     $weight_block4 }
                $slant4      { let $slant4      = $slant4.into();      $slant_block4 }
                $blink4      { let $blink4      = $blink4.into();      $blink_block4 }
                $invert4     { let $invert4     = $invert4.into();     $invert_block4 }
                $strike4     { let $strike4     = $strike4.into();     $strike_block4 }
                $underline4  { let $underline4  = $underline4.into();  $underline_block4 }
                $overline4   { let $overline4   = $overline4.into();   $overline_block4 }
                $border4     { let $border4     = $border4.into();     $border_block4 }
            })?);
        )?
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler {
    ([StylerIndex] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:ty) => $styler:expr) => {
        __impl_styler!([StylerIndex] $(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            { $styler.get_foreground() }
            { $styler.get_background() }
            { $styler.get_weight() }
            { $styler.get_slant() }
            { $styler.get_blink() }
            { $styler.get_invert() }
            { $styler.get_strike() }
            { $styler.get_underline() }
            { $styler.get_overline() }
            { $styler.get_border() }
        });
    };
    ([StylerIndex] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:ty) {
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
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerIndex for $Self {
            __impl_styler!([get] get_foreground($self, Foreground) $foreground_block);
            __impl_styler!([get] get_background($self, Background) $background_block);
            __impl_styler!([get] get_weight    ($self, Weight)     $weight_block);
            __impl_styler!([get] get_slant     ($self, Slant)      $slant_block);
            __impl_styler!([get] get_blink     ($self, Blink)      $blink_block);
            __impl_styler!([get] get_invert    ($self, Invert)     $invert_block);
            __impl_styler!([get] get_strike    ($self, Strike)     $strike_block);
            __impl_styler!([get] get_underline ($self, Underline)  $underline_block);
            __impl_styler!([get] get_overline  ($self, Overline)   $overline_block);
            __impl_styler!([get] get_border    ($self, Border)     $border_block);
        }
    };

    ([StylerIndexMut] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:ty) => $styler:expr) => {
        __impl_styler!([StylerIndexMut] $(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            { $styler.get_foreground_mut() }
            { $styler.get_background_mut() }
            { $styler.get_weight_mut() }
            { $styler.get_slant_mut() }
            { $styler.get_blink_mut() }
            { $styler.get_invert_mut() }
            { $styler.get_strike_mut() }
            { $styler.get_underline_mut() }
            { $styler.get_overline_mut() }
            { $styler.get_border_mut() }
        });
    };
    ([StylerIndexMut] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:ty) {
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
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerIndexMut for $Self {
            __impl_styler!([get mut] get_foreground_mut($self, Foreground) $foreground_block);
            __impl_styler!([get mut] get_background_mut($self, Background) $background_block);
            __impl_styler!([get mut] get_weight_mut    ($self, Weight)     $weight_block);
            __impl_styler!([get mut] get_slant_mut     ($self, Slant)      $slant_block);
            __impl_styler!([get mut] get_blink_mut     ($self, Blink)      $blink_block);
            __impl_styler!([get mut] get_invert_mut    ($self, Invert)     $invert_block);
            __impl_styler!([get mut] get_strike_mut    ($self, Strike)     $strike_block);
            __impl_styler!([get mut] get_underline_mut ($self, Underline)  $underline_block);
            __impl_styler!([get mut] get_overline_mut  ($self, Overline)   $overline_block);
            __impl_styler!([get mut] get_border_mut    ($self, Border)     $border_block);
        }
    };

    ([Styler] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:ty) => $styler:expr) => {
        __impl_styler!([Styler] $(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            foreground { $styler.foreground(foreground); $self }
            background { $styler.background(background); $self }
            weight     { $styler.weight(weight);         $self }
            slant      { $styler.slant(slant);           $self }
            blink      { $styler.blink(blink);           $self }
            invert     { $styler.invert(invert);         $self }
            strike     { $styler.strike(strike);         $self }
            underline  { $styler.underline(underline);   $self }
            overline   { $styler.overline(overline);     $self }
            border     { $styler.border(border);         $self }
        });
    };
    ([Styler] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:ty) {
        $foreground:tt $foreground_block:block
        $background:tt $background_block:block
        $weight:tt     $weight_block:block
        $slant:tt      $slant_block:block
        $blink:tt      $blink_block:block
        $invert:tt     $invert_block:block
        $strike:tt     $strike_block:block
        $underline:tt  $underline_block:block
        $overline:tt   $overline_block:block
        $border:tt     $border_block:block
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Self {
            __impl_styler!([set] foreground($self, $foreground: Foreground) $foreground_block);
            __impl_styler!([set] background($self, $background: Background) $background_block);
            __impl_styler!([set] weight    ($self, $weight:     Weight)     $weight_block);
            __impl_styler!([set] slant     ($self, $slant:      Slant)      $slant_block);
            __impl_styler!([set] blink     ($self, $blink:      Blink)      $blink_block);
            __impl_styler!([set] invert    ($self, $invert:     Invert)     $invert_block);
            __impl_styler!([set] strike    ($self, $strike:     Strike)     $strike_block);
            __impl_styler!([set] underline ($self, $underline:  Underline)  $underline_block);
            __impl_styler!([set] overline  ($self, $overline:   Overline)   $overline_block);
            __impl_styler!([set] border    ($self, $border:     Border)     $border_block);
        }
    };

    ([StylerMut] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:ty) => $styler:expr) => {
        __impl_styler!([StylerMut] $(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            foreground { $styler.foreground_mut(foreground); }
            background { $styler.background_mut(background); }
            weight     { $styler.weight_mut    (weight); }
            slant      { $styler.slant_mut     (slant); }
            blink      { $styler.blink_mut     (blink); }
            invert     { $styler.invert_mut    (invert); }
            strike     { $styler.strike_mut    (strike); }
            underline  { $styler.underline_mut (underline); }
            overline   { $styler.overline_mut  (overline); }
            border     { $styler.border_mut    (border); }
        });
    };
    ([StylerMut] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:ty) {
        $foreground:tt $foreground_block:block
        $background:tt $background_block:block
        $weight:tt     $weight_block:block
        $slant:tt      $slant_block:block
        $blink:tt      $blink_block:block
        $invert:tt     $invert_block:block
        $strike:tt     $strike_block:block
        $underline:tt  $underline_block:block
        $overline:tt   $overline_block:block
        $border:tt     $border_block:block
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::StylerMut for $Self {
            __impl_styler!([set mut] foreground_mut($self, $foreground: Foreground) $foreground_block);
            __impl_styler!([set mut] background_mut($self, $background: Background) $background_block);
            __impl_styler!([set mut] weight_mut    ($self, $weight:     Weight)     $weight_block);
            __impl_styler!([set mut] slant_mut     ($self, $slant:      Slant)      $slant_block);
            __impl_styler!([set mut] blink_mut     ($self, $blink:      Blink)      $blink_block);
            __impl_styler!([set mut] invert_mut    ($self, $invert:     Invert)     $invert_block);
            __impl_styler!([set mut] strike_mut    ($self, $strike:     Strike)     $strike_block);
            __impl_styler!([set mut] underline_mut ($self, $underline:  Underline)  $underline_block);
            __impl_styler!([set mut] overline_mut  ($self, $overline:   Overline)   $overline_block);
            __impl_styler!([set mut] border_mut    ($self, $border:     Border)     $border_block);
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
            #[allow(unused_mut)]
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
