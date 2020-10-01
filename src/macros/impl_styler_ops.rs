/// Impl `Styler` ops.
#[macro_export]
macro_rules! impl_styler_ops {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($Type:ty)) => {
        $crate::__impl_styler_ops!([ops] $Type {
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

        $crate::__impl_styler_ops!([bit] $Type {
            <$($($G $(: $($B)+)?,)+)?> BitAnd(bitand) BitAndAssign(bitand_assign) and_mut,
            <$($($G $(: $($B)+)?,)+)?> BitOr (bitor ) BitOrAssign (bitor_assign ) or_mut,
            <$($($G $(: $($B)+)?,)+)?> BitXor(bitxor) BitXorAssign(bitxor_assign) xor_mut,
            <$($($G $(: $($B)+)?,)+)?> Rem   (rem   ) RemAssign   (rem_assign   ) dedup_mut,
        });

        $crate::__impl_styler_ops!(trait unary $Type {
            <$($($G $(: $($B)+)?,)+)?> Not(not) reset_mut
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler_ops {
    ([ops] $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident($rhs:tt: $Rhs:ident $NoRhs:ident) $body:expr,
    )* }) => {
        __impl_styler_ops!(trait binary $Type { $(
            <$($G $(: $($B)+)?,)*> $Op($op) $OpAssign($op_assign)
            $fn($rhs: $crate::$Rhs) Some($body),
            <$($G $(: $($B)+)?,)*> $Op($op) $OpAssign($op_assign)
            $fn(_: $crate::$NoRhs) None,
        )* });
    };
   ([bit] $Type:ty { $(
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident,
    )* }) => {
        __impl_styler_ops!(trait binary $Type { $(
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            $Op($op) $OpAssign($op_assign) $fn(styler: &Styler) styler,
        )* });
    };
    (trait binary $Type:ty { $(
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
    (trait unary $Type:ty { $(
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
