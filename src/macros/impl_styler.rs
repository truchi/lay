#[macro_export]
macro_rules! impl_styler {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $foreground:expr,
        $background:expr,
        $weight:expr,
        $slant:expr,
        $blink:expr,
        $invert:expr,
        $strike:expr,
        $underline:expr,
        $overline:expr,
        $border:expr,
    }) => {
        impl_styler!(impl [mode:field] $(<$($G $(: $($B)+)?,)+>)? ($self: $Type) {
            $foreground,
            $background,
            $weight,
            $slant,
            $blink,
            $invert,
            $strike,
            $underline,
            $overline,
            $border,
        });
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) => $style:expr) => {
        impl_styler!(impl [mode:method] $(<$($G $(: $($B)+)?,)+>)? ($self: $Type) {
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
        });
    };

    // =====
    // Trait
    // =====
    (impl [mode:$mode:ident] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
        $foreground:expr,
        $background:expr,
        $weight:expr,
        $slant:expr,
        $blink:expr,
        $invert:expr,
        $strike:expr,
        $underline:expr,
        $overline:expr,
        $border:expr,
    }) => {
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Type {
            impl_styler!(impl [mode:$mode] $self {
                Foreground get_foreground foreground_mut $foreground
                Background get_background background_mut $background
                Weight     get_weight     weight_mut     $weight
                Slant      get_slant      slant_mut      $slant
                Blink      get_blink      blink_mut      $blink
                Invert     get_invert     invert_mut     $invert
                Strike     get_strike     strike_mut     $strike
                Underline  get_underline  underline_mut  $underline
                Overline   get_overline   overline_mut   $overline
                Border     get_border     border_mut     $border
            });
        }

        impl_styler!(impl [ops] $Type: $(<$($G $(: $($B)+)?,)+>)?);
    };
    (impl [mode:$mode:ident] $self:ident {
        $($Attr:ident $get:ident $set_mut:ident $field:expr)*
    }) => {
        $(impl_styler!(impl [mode:$mode] $Attr $get $set_mut $self $field);)*
    };
    (impl [mode:field] $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $field
        }

        fn $set_mut(&mut self, arg: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $field = arg.into();
        }
    };
    (impl [mode:method] $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $field.$get()
        }

        fn $set_mut(&mut self, arg: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $field.$set_mut(arg);
        }
    };

    // ===
    // Ops
    // ===
    (impl [ops] $Type:ty: $(<$($G:ident $(: $($B:path)+)?,)+>)?) => {
        impl_styler!(impl [ops:ops] $Type {
            <$($($G $(: $($B)+)?,)+)?> Mul(mul) MulAssign(mul_assign)
                foreground_mut(foreground: Color NoColor) $crate::Foreground(foreground),
            <$($($G $(: $($B)+)?,)+)?> Div(div) DivAssign(div_assign)
                background_mut(background: Color NoColor) $crate::Background(background),
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                foreground_mut(foreground: Foreground NoForeground) foreground,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                background_mut(background: Background NoBackground) background,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                weight_mut(weight: Weight NoWeight) weight,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                slant_mut(slant: Slant NoSlant) slant,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                blink_mut(blink: Blink NoBlink) blink,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                invert_mut(invert: Invert NoInvert) invert,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                strike_mut(strike: Strike NoStrike) strike,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                underline_mut(underline: Underline NoUnderline) underline,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                overline_mut(overline: Overline NoOverline) overline,
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                border_mut(border: Border NoBorder) border,
        });
        impl_styler!(impl [ops:bit] $Type {
            <$($($G $(: $($B)+)?,)+)?> BitAnd(bitand) BitAndAssign(bitand_assign) and_mut,
            <$($($G $(: $($B)+)?,)+)?> BitOr (bitor ) BitOrAssign (bitor_assign ) or_mut,
            <$($($G $(: $($B)+)?,)+)?> BitXor(bitxor) BitXorAssign(bitxor_assign) xor_mut,
            <$($($G $(: $($B)+)?,)+)?> Rem   (rem   ) RemAssign   (rem_assign   ) dedup_mut,
        });

        impl_styler!(impl trait unary $Type {
            <$($($G $(: $($B)+)?,)+)?> Not(not) reset_mut
        });
    };

    (impl [ops:ops] $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident($rhs:tt: $Rhs:ident $NoRhs:ident) $body:expr,
    )* }) => {
        impl_styler!(impl trait binary $Type { $(
            <$($G $(: $($B)+)?,)*> $Op($op) $OpAssign($op_assign)
            $fn($rhs: $crate::$Rhs) Some($body),
            <$($G $(: $($B)+)?,)*> $Op($op) $OpAssign($op_assign)
            $fn(_: $crate::$NoRhs) None,
        )* });
    };
   (impl [ops:bit] $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident,
    )* }) => {
        impl_styler!(impl trait binary $Type { $(
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            $Op($op) $OpAssign($op_assign) $fn(styler: &Styler) styler,
        )* });
    };

    (impl trait binary $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident($rhs:tt: $($Rhs:tt)*) $body:expr,
    )* }) => {
        $(
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op<$($Rhs)*> for $Type {
                type Output = Self;

                fn $op(mut self, $rhs: $($Rhs)*) -> Self {
                    $crate::Styler::$fn(&mut self, $body);
                    self
                }
            }

            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$OpAssign<$($Rhs)*> for $Type {
                fn $op_assign(&mut self, $rhs: $($Rhs)*) {
                    $crate::Styler::$fn(self, $body);
                }
            }
        )*
    };
    (impl trait unary $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $fn:ident
    )* }) => {
        $(
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for $Type {
                type Output = Self;

                fn $op(mut self) -> Self {
                    $crate::Styler::$fn(&mut self);
                    self
                }
            }

            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for &mut $Type {
                type Output = ();

                fn $op(self) {
                    $crate::Styler::$fn(self);
                }
            }
        )*
    };
}
