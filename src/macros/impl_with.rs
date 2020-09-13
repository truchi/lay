macro_rules! impl_with {
    (
        $Type:ident :
        $self:ident {
            $(foreground  $([$foreground_op:tt])?          : $foreground:expr,)?
            $(foreground? $([$optional_foreground_op:tt])? : $optional_foreground:expr,)?
            $(background  $([$background_op:tt])?          : $background:expr,)?
            $(background? $([$optional_background_op:tt])? : $optional_background:expr,)?
            $(weight      $([$weight_op:tt])?              : $weight:expr,)?
            $(weight?     $([$optional_weight_op:tt])?     : $optional_weight:expr,)?
            $(slant       $([$slant_op:tt])?               : $slant:expr,)?
            $(slant?      $([$optional_slant_op:tt])?      : $optional_slant:expr,)?
            $(blink       $([$blink_op:tt])?               : $blink:expr,)?
            $(blink?      $([$optional_blink_op:tt])?      : $optional_blink:expr,)?
            $(invert      $([$invert_op:tt])?              : $invert:expr,)?
            $(invert?     $([$optional_invert_op:tt])?     : $optional_invert:expr,)?
            $(strike      $([$strike_op:tt])?              : $strike:expr,)?
            $(strike?     $([$optional_strike_op:tt])?     : $optional_strike:expr,)?
            $(underline   $([$underline_op:tt])?           : $underline:expr,)?
            $(underline?  $([$optional_underline_op:tt])?  : $optional_underline:expr,)?
            $(overline    $([$overline_op:tt])?            : $overline:expr,)?
            $(overline?   $([$optional_overline_op:tt])?   : $optional_overline:expr,)?
            $(border      $([$border_op:tt])?              : $border:expr,)?
            $(border?     $([$optional_border_op:tt])?     : $optional_border:expr,)?
        }
    ) => {
        // Foreground
        $(impl_with!(impl trait Color $Type $self $foreground, [$($foreground_op)?]
            WithForeground Foreground get_foreground get_foreground_mut foreground foreground_mut);)?
        $(impl_with!(impl trait? Color $Type $self $optional_foreground [$($optional_foreground_op)?]
           WithOptionalForeground Foreground get_foreground get_foreground_mut foreground foreground_mut);)?

        // Background
        $(impl_with!(impl trait Color $Type $self $background, [$($background_op)?]
            WithBackground Background get_background get_background_mut background background_mut);)?
        $(impl_with!(impl trait? Color $Type $self $optional_background, [$($optional_background_op)?]
            WithOptionalBackground Background get_background get_background_mut background background_mut);)?

        // Weighted
        $(impl_with!(impl trait $Type $self $weight, [$($weight_op)?]
            WithWeight Weighted get_weight get_weight_mut weighted weighted_mut);)?
        $(impl_with!(impl trait? $Type $self $optional_weight, [$($optional_weight_op)?]
            WithOptionalWeight Weighted get_weight get_weight_mut weighted weighted_mut);)?

        // Slanted
        $(impl_with!(impl trait $Type $self $slant, [$($slant_op)?]
            WithSlant Slanted get_slant get_slant_mut slanted slanted_mut);)?
        $(impl_with!(impl trait? $Type $self $optional_slant, [$($optional_slant_op)?]
            WithOptionalSlant Slanted get_slant get_slant_mut slanted slanted_mut);)?

        // Blinking
        $(impl_with!(impl trait $Type $self $blink, [$($blink_op)?]
            WithBlink Blinking get_blink get_blink_mut blinking blinking_mut);)?
        $(impl_with!(impl trait? $Type $self $optional_blink, [$($optional_blink_op)?]
            WithOptionalBlink Blinking get_blink get_blink_mut blinking blinking_mut);)?

        // Inverted
        $(impl_with!(impl trait $Type $self $invert, [$($invert_op)?]
            WithInvert Inverted get_invert get_invert_mut inverted inverted_mut);)?
        $(impl_with!(impl trait? $Type $self $optional_invert, [$($optional_invert_op)?]
            WithOptionalInvert $crate::Inverted get_invert get_invert_mut inverted inverted_mut);)?

        // Striked
        $(impl_with!(impl trait $Type $self $strike, [$($strike_op)?]
            WithStrike Striked get_strike get_strike_mut striked striked_mut);)?
        $(impl_with!(impl trait? $Type $self$optional_strike, [$($optional_strike_op)?]
            WithOptionalStrike Striked get_strike get_strike_mut striked striked_mut);)?

        // Underlined
        $(impl_with!(impl trait $Type $self $underline, [$($underline_op)?]
            WithUnderline Underlined get_underline get_underline_mut underlined underlined_mut);)?
        $(impl_with!(impl trait? $Type $self $optional_underline, [$($optional_underline_op)?]
            WithOptionalUnderline Underlined get_underline get_underline_mut underlined underlined_mut);)?

        // Overlined
        $(impl_with!(impl trait $Type $self $overline, [$($overline_op)?]
            WithOverline Overlined get_overline get_overline_mut overlined overlined_mut);)?
        $(impl_with!(impl trait? $Type $self $optional_overline, [$($optional_overline_op)?]
            WithOptionalOverline Overlined get_overline get_overline_mut overlined overlined_mut);)?

        // Bordered
        $(impl_with!(impl trait $Type $self $border, [$($border_op)?]
            WithBorder Bordered get_border get_border_mut bordered bordered_mut);)?
        $(impl_with!(impl trait $Type $self $optional_border, [$($optional_border_op)?]
            WithOptionalBorder Bordered get_border get_border_mut bordered bordered_mut);)?
    };
    // Color
    (
        impl trait Color $Type:ident $self:ident $field:expr, [$($op:tt)?]
            $Trait:ident $Attr:ident $get:ident $get_mut:ident $set:ident $set_mut:ident
    ) => {
        impl_with!(impl impl trait $Type $self $field; $Trait $crate::$Attr; $get $get_mut $set $set_mut);
        $(impl_with!(impl op $op $Type $Trait Color $set $set_mut);)?
    };
    // Optional color
    (
        impl trait? Color $Type:ident $self:ident $field:expr, [$($op:tt)?]
            $Trait:ident $Attr:ident $get:ident $get_mut:ident $set:ident $set_mut:ident
    ) => {
        impl_with!(impl impl trait $Type $self $field; $Trait ::std::option::Option<$crate::$Attr>; $get $get_mut $set $set_mut);
        $(impl_with!(impl op $op $Type $Trait Color $set $set_mut);)?
    };
    // Attribute
    (
        impl trait $Type:ident $self:ident $field:expr, [$($op:tt)?]
            $Trait:ident $Attr:ident $get:ident $get_mut:ident $set:ident $set_mut:ident
    ) => {
        impl_with!(impl impl trait $Type $self $field; $Trait $crate::$Attr; $get $get_mut $set $set_mut);
        $(impl_with!(impl op $op $Type $Trait $Attr $set $set_mut);)?
    };
    // Optional attribute
    (
        impl trait? $Type:ident $self:ident $field:expr, [$($op:tt)?]
            $Trait:ident $Attr:ident $get:ident $get_mut:ident $set:ident $set_mut:ident
    ) => {
        impl_with!(impl impl trait $Type $self $field; $Trait ::std::option::Option<$crate::$Attr>; $get $get_mut $set $set_mut);
        $(impl_with!(impl op $op $Type $Trait $Attr $set $set_mut);)?
    };
    // WithX trait implementation
    (
        impl impl trait $Type:ident $self:ident $field:expr;
            $Trait:ident $Return:ty; $get:ident $get_mut:ident $set:ident $set_mut:ident
    ) => {
        impl $crate::$Trait for $Type {
            fn $get(&self) -> $Return {
                let $self = self;
                $field
            }

            fn $get_mut(&mut self) -> &mut $Return {
                let $self = self;
                &mut $field
            }
        }
    };
    // Ops trait implementation
    (impl op $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $Op:ident $OpAssign:ident $op:ident $op_assign:ident) => {
        impl ::std::ops::$Op<$crate::$Attr> for $Type {
            type Output = Self;

            fn $op(self, rhs: $crate::$Attr) -> Self {
                use $crate::$Trait;
                self.$set(rhs)
            }
        }

        impl ::std::ops::$OpAssign<$crate::$Attr> for $Type {
            fn $op_assign(&mut self, rhs: $crate::$Attr) {
                use $crate::$Trait;
                self.$set_mut(rhs);
            }
        }
    };
    // Ops...
    (impl op + $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut Add AddAssign add add_assign);
    };
    (impl op - $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut Sub SubAssign sub sub_assign);
    };
    (impl op * $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut Mul MulAssign mul mul_assign);
    };
    (impl op / $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut Div DivAssign div div_assign);
    };
    (impl op % $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut Rem RemAssign rem rem_assign);
    };
    (impl op << $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut Shl ShlAssign shl shl_assign);
    };
    (impl op >> $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut Shr ShrAssign shr shr_assign);
    };
    (impl op & $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut BitAnd BitAndAssign bitand bitand_assign);
    };
    (impl op | $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut BitOr BitOrAssign bitor bitor_assign);
    };
    (impl op ^ $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident) => {
        impl_with!(impl op $Type $Trait $Attr $set $set_mut BitXor BitXorAssign bitxor bitxor_assign);
    };
}
