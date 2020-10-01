/// Impl `Styler` ops.
#[macro_export]
macro_rules! impl_styler_ops {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($Type:ty)) => {
        $crate::__impl_styler_ops!([ops] $Type {
            "Foreground",
            <$($($G $(: $($B)+)?,)+)?> Mul(mul) MulAssign(mul_assign)
                foreground_mut(foreground: Color NoColor) $crate::Foreground(foreground),
            "Background",
            <$($($G $(: $($B)+)?,)+)?> Div(div) DivAssign(div_assign)
                background_mut(background: Color NoColor) $crate::Background(background),
            "Foreground",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                foreground_mut(foreground: Foreground NoForeground) foreground,
            "Background",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                background_mut(background: Background NoBackground) background,
            "Weight",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                weight_mut(weight: Weight NoWeight) weight,
            "Slant",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                slant_mut(slant: Slant NoSlant) slant,
            "Blink",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                blink_mut(blink: Blink NoBlink) blink,
            "Invert",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                invert_mut(invert: Invert NoInvert) invert,
            "Strike",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                strike_mut(strike: Strike NoStrike) strike,
            "Underline",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                underline_mut(underline: Underline NoUnderline) underline,
            "Overline",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                overline_mut(overline: Overline NoOverline) overline,
            "Border",
            <$($($G $(: $($B)+)?,)+)?> Add(add) AddAssign(add_assign)
                border_mut(border: Border NoBorder) border,
        });

        $crate::__impl_styler_ops!([bit] $Type {
            "`Option::and` fields",
            <$($($G $(: $($B)+)?,)+)?> BitAnd(bitand) BitAndAssign(bitand_assign) and_mut,
            "`Option::or` fields",
            <$($($G $(: $($B)+)?,)+)?> BitOr(bitor) BitOrAssign(bitor_assign) or_mut,
            "`Option::xor` fields",
            <$($($G $(: $($B)+)?,)+)?> BitXor(bitxor) BitXorAssign(bitxor_assign) xor_mut,
            "Dedups (`None`s if identicals) fields",
            <$($($G $(: $($B)+)?,)+)?> Rem(rem) RemAssign(rem_assign) dedup_mut,
        });

        $crate::__impl_styler_ops!(trait unary $Type {
            "Resets (sets to reset value) fields which are `Some`",
            <$($($G $(: $($B)+)?,)+)?> Not(not) reset_mut
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler_ops {
    ([ops] $Type:ty { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident($rhs:tt: $Rhs:ident $NoRhs:ident) $body:expr,
    )* }) => {
        __impl_styler_ops!(trait binary $Type { $(
            "Sets `Option<" $($doc)* ">`",
            <$($G $(: $($B)+)?,)*> $Op($op) $OpAssign($op_assign)
            $fn($rhs: $crate::$Rhs) Some($body),
            "Sets " stringify!($rhs) " to `None`",
            <$($G $(: $($B)+)?,)*> $Op($op) $OpAssign($op_assign)
            $fn(_: $crate::$NoRhs) None,
        )* });
    };
   ([bit] $Type:ty { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident,
    )* }) => {
        __impl_styler_ops!(trait binary $Type { $(
            $($doc)*,
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            $Op($op) $OpAssign($op_assign) $fn(styler: &Styler) styler,
        )* });
    };
    (trait binary $Type:ty { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident)
        $fn:ident($rhs:tt: $($Rhs:tt)*) $body:expr,
    )* }) => {
        $(
            $crate::doc!($($doc)* ".",
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op<$($Rhs)*> for $Type {
                $crate::doc!("`" stringify!($Type) "`.",
                type Output = Self;);

                $crate::doc!($($doc)* ".",
                fn $op(mut self, $rhs: $($Rhs)*) -> Self {
                    $crate::Styler::$fn(&mut self, $body);
                    self
                });
            });

            $crate::doc!($($doc)* " mutably.",
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$OpAssign<$($Rhs)*> for $Type {
                $crate::doc!($($doc)* " mutably.",
                fn $op_assign(&mut self, $rhs: $($Rhs)*) {
                    $crate::Styler::$fn(self, $body);
                });
            });
        )*
    };
    (trait unary $Type:ty { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $fn:ident
    )* }) => {
        $(
            $crate::doc!($($doc)* ".",
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for $Type {
                $crate::doc!("`" stringify!($Type) "`.",
                type Output = Self;);

                $crate::doc!($($doc)* ".",
                fn $op(mut self) -> Self {
                    $crate::Styler::$fn(&mut self);
                    self
                });
            });

            $crate::doc!($($doc)* " mutably.",
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for &mut $Type {
                $crate::doc!("`()` (in place).",
                type Output = (););

                $crate::doc!($($doc)* " mutably.",
                fn $op(self) {
                    $crate::Styler::$fn(self);
                });
            });
        )*
    };
}
