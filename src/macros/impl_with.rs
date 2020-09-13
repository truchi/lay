macro_rules! impl_with {
    (
        $Type:ident: $self:ident {
            $($Trait:ident $([$op:tt])?: $field:expr,)+
        }
    ) => {
        $(impl_with!(impl $Trait $Type $self $field, $($op)?);)+
    };

    // Foreground
    (impl WithForeground $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithForeground $crate::Foreground;
            get_foreground get_foreground_mut foreground foreground_mut
            $($op Color)?
        );
    };
    (impl WithOptionalForeground $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalForeground ::std::option::Option<$crate::Foreground>;
            get_foreground get_foreground_mut foreground foreground_mut
            $($op Color NoColor no_foreground no_foreground_mut)?
        );
    };

    // Background
    (impl WithBackground $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithBackground $crate::Background;
            get_background get_background_mut background background_mut
            $($op Color)?
        );
    };
    (impl WithOptionalBackground $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalBackground ::std::option::Option<$crate::Background>;
            get_background get_background_mut background background_mut
            $($op Color NoColor no_background no_background_mut)?
        );
    };

    // Weighted
    (impl WithWeight $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithWeight $crate::Weighted;
            get_weight get_weight_mut weighted weighted_mut
            $($op Weighted)?
        );
    };
    (impl WithOptionalWeight $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalWeight ::std::option::Option<$crate::Weighted>;
            get_weight get_weight_mut weighted weighted_mut
            $($op Weighted NoWeight no_weight no_weight_mut)?
        );
    };

    // Slanted
    (impl WithSlant $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithSlant $crate::Slanted;
            get_slant get_slant_mut slanted slanted_mut
            $($op Slanted)?
        );
    };
    (impl WithOptionalSlant $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalSlant ::std::option::Option<$crate::Slanted>;
            get_slant get_slant_mut slanted slanted_mut
            $($op Slanted NoSlant no_slant no_slant_mut)?
        );
    };

    // Blinking
    (impl WithBlink $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithBlink $crate::Blinking;
            get_blink get_blink_mut blinking blinking_mut
            $($op Blinking)?
        );
    };
    (impl WithOptionalBlink $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalBlink ::std::option::Option<$crate::Blinking>;
            get_blink get_blink_mut blinking blinking_mut
            $($op Blinking NoBlink no_blink no_blink_mut)?
        );
    };

    // Inverted
    (impl WithInvert $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithInvert $crate::Inverted;
            get_invert get_invert_mut inverted inverted_mut
            $($op Inverted)?
        );
    };
    (impl WithOptionalInvert $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalInvert ::std::option::Option<$crate::Inverted>;
            get_invert get_invert_mut inverted inverted_mut
            $($op Inverted NoInvert no_invert no_invert_mut)?
        );
    };

    // Striked
    (impl WithStrike $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithStrike $crate::Striked;
            get_strike get_strike_mut striked striked_mut
            $($op Striked)?
        );
    };
    (impl WithOptionalStrike $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalStrike ::std::option::Option<$crate::Striked>;
            get_strike get_strike_mut striked striked_mut
            $($op Striked NoStrike no_strike no_strike_mut)?
        );
    };

    // Underlined
    (impl WithUnderline $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithUnderline $crate::Underlined;
            get_underline get_underline_mut underlined underlined_mut
            $($op Underlined)?
        );
    };
    (impl WithOptionalUnderline $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalUnderline ::std::option::Option<$crate::Underlined>;
            get_underline get_underline_mut underlined underlined_mut
            $($op Underlined NoUnderline no_underline no_underline_mut)?
        );
    };

    // Overlined
    (impl WithOverline $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOverline $crate::Overlined;
            get_overline get_overline_mut overlined overlined_mut
            $($op Overlined)?
        );
    };
    (impl WithOptionalOverline $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalOverline ::std::option::Option<$crate::Overlined>;
            get_overline get_overline_mut overlined overlined_mut
            $($op Overlined NoOverline no_overline no_overline_mut)?
        );
    };

    // Bordered
    (impl WithBorder $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithBorder $crate::Bordered;
            get_border get_border_mut bordered bordered_mut
            $($op Bordered)?
        );
    };
    (impl WithOptionalBorder $Type:ident $self:ident $field:expr, $($op:tt)?) => {
        impl_with!(impl $Type $self $field;
            WithOptionalBorder ::std::option::Option<$crate::Bordered>;
            get_border get_border_mut bordered bordered_mut
            $($op Bordered NoBorder no_border no_border_mut)?
        );
    };

    // Dispatch to impl trait & impl op $op
    (impl $Type:ident $self:ident $field:expr;
        $Trait:ident $Return:ty;
        $get:ident $get_mut:ident $set:ident $set_mut:ident
        $($op:tt $Rhs:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?)?
    ) => {
        impl_with!(impl trait $Type $self $field; $Trait $Return; $get $get_mut $set $set_mut);
        $(impl_with!(impl op $op $Type $Trait $Rhs $set $set_mut $($NoAttr $unset $unset_mut)?);)?
    };

    // WithX trait implementation
    (
        impl trait $Type:ident $self:ident $field:expr;
            $Trait:ident $Return:ty;
            $get:ident $get_mut:ident $set:ident $set_mut:ident
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
    (impl op
        $Type:ident $Trait:ident $Rhs:ident
        $set:ident $set_mut:ident
        $Op:ident $OpAssign:ident $op:ident $op_assign:ident
    ) => {
        impl ::std::ops::$Op<$crate::$Rhs> for $Type {
            type Output = Self;

            fn $op(self, rhs: $crate::$Rhs) -> Self {
                use $crate::$Trait;
                self.$set(rhs)
            }
        }

        impl ::std::ops::$OpAssign<$crate::$Rhs> for $Type {
            fn $op_assign(&mut self, rhs: $crate::$Rhs) {
                use $crate::$Trait;
                self.$set_mut(rhs);
            }
        }
    };

    // Ops trait implementation for NoX on optional attributes
    (impl no op
        $Type:ident $Trait:ident $Rhs:ident
        $unset:ident $unset_mut:ident
        $Op:ident $OpAssign:ident $op:ident $op_assign:ident
    ) => {
        impl ::std::ops::$Op<$crate::$Rhs> for $Type {
            type Output = Self;

            fn $op(self, _: $crate::$Rhs) -> Self {
                use $crate::$Trait;
                self.$unset()
            }
        }

        impl ::std::ops::$OpAssign<$crate::$Rhs> for $Type {
            fn $op_assign(&mut self, _: $crate::$Rhs) {
                use $crate::$Trait;
                self.$unset_mut();
            }
        }
    };

    // Ops...
    (impl op + $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr   $set   $set_mut   Add AddAssign add add_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr $unset $unset_mut Add AddAssign add add_assign);)?
    };
    (impl op - $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr   $set   $set_mut   Sub SubAssign sub sub_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr $unset $unset_mut Sub SubAssign sub sub_assign);)?
    };
    (impl op * $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr   $set   $set_mut   Mul MulAssign mul mul_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr $unset $unset_mut Mul MulAssign mul mul_assign);)?
    };
    (impl op / $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr   $set   $set_mut   Div DivAssign div div_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr $unset $unset_mut Div DivAssign div div_assign);)?
    };
    (impl op % $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr   $set   $set_mut   Rem RemAssign rem rem_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr $unset $unset_mut Rem RemAssign rem rem_assign);)?
    };
    (impl op << $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr;  $set   $set_mut   Shl ShlAssign shl shl_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr;$unset $unset_mut Shl ShlAssign shl shl_assign);)?
    };
    (impl op >> $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr;  $set   $set_mut   Shr ShrAssign shr shr_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr;$unset $unset_mut Shr ShrAssign shr shr_assign);)?
    };
    (impl op & $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr   $set   $set_mut   BitAnd BitAndAssign bitand bitand_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr $unset $unset_mut BitAnd BitAndAssign bitand bitand_assign);)?
    };
    (impl op | $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr   $set   $set_mut   BitOr BitOrAssign bitor bitor_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr $unset $unset_mut BitOr BitOrAssign bitor bitor_assign);)?
    };
    (impl op ^ $Type:ident $Trait:ident $Attr:ident $set:ident $set_mut:ident $($NoAttr:ident $unset:ident $unset_mut:ident)?) => {
        impl_with!(  impl    op $Type $Trait $Attr   $set   $set_mut   BitXor BitXorAssign bitxor bitxor_assign);
        $(impl_with!(impl no op $Type $Trait $NoAttr $unset $unset_mut BitXor BitXorAssign bitxor bitxor_assign);)?
    };
}
