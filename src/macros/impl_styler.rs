/// Impl `Styler`.
#[macro_export]
macro_rules! impl_styler {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $(($attr:tt: $Attr:ident) $get:block $set:block)*
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Type {
            $(__impl_styler!(($self, $attr: $Attr) $get $set);)*
        }
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) => $style:expr) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Type {
            __impl_styler!(($self, foreground: Foreground)
                get_foreground { $style.get_foreground() }
                foreground_mut { $style.foreground_mut(foreground) }
            );
            __impl_styler!(($self, background: Background)
                get_background { $style.get_background() }
                background_mut { $style.background_mut(background) }
            );
            __impl_styler!(($self, weight: Weight)
                get_weight { $style.get_weight() }
                weight_mut { $style.weight_mut(weight) }
            );
            __impl_styler!(($self, slant: Slant)
                get_slant { $style.get_slant() }
                slant_mut { $style.slant_mut(slant) }
            );
            __impl_styler!(($self, blink: Blink)
                get_blink { $style.get_blink() }
                blink_mut { $style.blink_mut(blink) }
            );
            __impl_styler!(($self, invert: Invert)
                get_invert { $style.get_invert() }
                invert_mut { $style.invert_mut(invert) }
            );
            __impl_styler!(($self, strike: Strike)
                get_strike { $style.get_strike() }
                strike_mut { $style.strike_mut(strike) }
            );
            __impl_styler!(($self, underline: Underline)
                get_underline { $style.get_underline() }
                underline_mut { $style.underline_mut(underline) }
            );
            __impl_styler!(($self, overline: Overline)
                get_overline { $style.get_overline() }
                overline_mut { $style.overline_mut(overline) }
            );
            __impl_styler!(($self, border: Border)
                get_border { $style.get_border() }
                border_mut { $style.border_mut(border) }
            );
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler {
    (
        ($self:ident, $attr:tt: $Attr:ident)
        $get_fn:ident $get_body:block
        $set_fn:ident $set_body:block
    ) => {
        fn $get_fn(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $get_body
        }

        fn $set_fn(
            &mut self,
            $attr: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>
        ) {
            let $self = self;
            $set_body
        }
    };
    (($self:ident, $attr:tt: Foreground) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Foreground)
            get_foreground $get foreground_mut $set
        );
    };
    (($self:ident, $attr:tt: Background) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Background)
            get_background $get background_mut $set
        );
    };
    (($self:ident, $attr:tt: Weight) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Weight)
            get_weight $get weight_mut $set
        );
    };
    (($self:ident, $attr:tt: Slant) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Slant)
            get_slant $get slant_mut $set
        );
    };
    (($self:ident, $attr:tt: Blink) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Blink)
            get_blink $get blink_mut $set
        );
    };
    (($self:ident, $attr:tt: Invert) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Invert)
            get_invert $get invert_mut $set
        );
    };
    (($self:ident, $attr:tt: Strike) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Strike)
            get_strike $get strike_mut $set
        );
    };
    (($self:ident, $attr:tt: Underline) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Underline)
            get_underline $get underline_mut $set
        );
    };
    (($self:ident, $attr:tt: Overline) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Overline)
            get_overline $get overline_mut $set
        );
    };
    (($self:ident, $attr:tt: Border) $get:block $set:block) => {
        __impl_styler!(($self, $attr: Border)
            get_border $get border_mut $set
        );
    };
}
