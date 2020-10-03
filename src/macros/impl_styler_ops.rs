/// Impl `Styler` ops.
#[macro_export]
macro_rules! impl_styler_ops {
    ($(<$($G1:ident $(: $($B1:path)+)?,)+>)? $Self:path) => {
        $crate::__impl_styler_ops!(trait binary (s: $Self) -> $Self {
            "Sets `Option<Foreground>`.",
            <$($($G1 $(: $($B1)+)?,)+)?
                Color: std::convert::Into<std::option::Option<$crate::Foreground>>,>
            Mul(mul) (foreground: Color) {
                $crate::Styler::foreground(s, foreground)
            }
            "Sets `Option<Background>`.",
            <$($($G1 $(: $($B1)+)?,)+)?
                Color: std::convert::Into<std::option::Option<$crate::Background>>,>
            Div(div) (background: Color) {
                $crate::Styler::background(s, background)
            }

            "Sets `Option<Foreground>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (foreground: $crate::Foreground) {
                $crate::Styler::foreground(s, Some(foreground))
            }
            "Sets foreground to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoForeground) {
                $crate::Styler::foreground(s, None)
            }

            "Sets `Option<Background>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (background: $crate::Background) {
                $crate::Styler::background(s, Some(background))
            }
            "Sets background to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoBackground) {
                $crate::Styler::background(s, None)
            }

            "Sets `Option<Weight>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (weight: $crate::Weight) {
                $crate::Styler::weight(s, Some(weight))
            }
            "Sets weight to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoWeight) {
                $crate::Styler::weight(s, None)
            }

            "Sets `Option<Slant>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (slant: $crate::Slant) {
                $crate::Styler::slant(s, Some(slant))
            }
            "Sets slant to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoSlant) {
                $crate::Styler::slant(s, None)
            }

            "Sets `Option<Blink>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (blink: $crate::Blink) {
                $crate::Styler::blink(s, Some(blink))
            }
            "Sets blink to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoBlink) {
                $crate::Styler::blink(s, None)
            }

            "Sets `Option<Invert>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (invert: $crate::Invert) {
                $crate::Styler::invert(s, Some(invert))
            }
            "Sets invert to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoInvert) {
                $crate::Styler::invert(s, None)
            }

            "Sets `Option<Strike>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (strike: $crate::Strike) {
                $crate::Styler::strike(s, Some(strike))
            }
            "Sets strike to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoStrike) {
                $crate::Styler::strike(s, None)
            }

            "Sets `Option<Underline>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (underline: $crate::Underline) {
                $crate::Styler::underline(s, Some(underline))
            }
            "Sets underline to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoUnderline) {
                $crate::Styler::underline(s, None)
            }

            "Sets `Option<Overline>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (overline: $crate::Overline) {
                $crate::Styler::overline(s, Some(overline))
            }
            "Sets overline to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoOverline) {
                $crate::Styler::overline(s, None)
            }

            "Sets `Option<Border>`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (border: $crate::Border) {
                $crate::Styler::border(s, Some(border))
            }
            "Sets border to `None`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Add(add) (_: $crate::NoBorder) {
                $crate::Styler::border(s, None)
            }

            "`Option::and` fields.",
            <$($($G1 $(: $($B1)+)?,)+)? Styler: $crate::Styler,>
            BitAnd(bitand) (styler: &Styler) {
                $crate::Styler::and(s, styler)
            }
            "`Option::or` fields.",
            <$($($G1 $(: $($B1)+)?,)+)? Styler: $crate::Styler,>
            BitOr(bitor) (styler: &Styler) {
                $crate::Styler::or(s, styler)
            }
            "`Option::xor` fields.",
            <$($($G1 $(: $($B1)+)?,)+)? Styler: $crate::Styler,>
            BitXor(bitxor) (styler: &Styler) {
                $crate::Styler::xor(s, styler)
            }

            "Dedups (`None`s if identicals) fields.",
            <$($($G1 $(: $($B1)+)?,)+)? Styler: $crate::Styler,>
            Rem(rem) (styler: &Styler) {
                $crate::Styler::dedup(s, styler)
            }
        });

        $crate::__impl_styler_ops!(trait unary (s: $Self) -> $Self {
            "Resets (sets to reset value) fields which are `Some`.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Not(not) {
                $crate::Styler::reset(s)
            }
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler_ops {
    (trait binary ($self:ident: $Self:path) -> $Output:path { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*> $Op:ident($op:ident)
        ($rhs:tt: $($Rhs:tt)*) $body:expr
    )* }) => {
        $(
            $crate::doc!($($doc)*,
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op<$($Rhs)*> for $Self {
                $crate::doc!("`" stringify!($Output) "`.",
                type Output = $Output;);

                $crate::doc!($($doc)*,
                fn $op(self, $rhs: $($Rhs)*) -> $Output {
                    let $self = self;
                    $body
                });
            });
        )*
    };
    (trait unary ($self:ident: $Self:path) -> $Output:path { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $body:expr
    )* }) => {
        $(
            $crate::doc!($($doc)*,
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for $Self {
                $crate::doc!("`" stringify!($Output) "`.",
                type Output = $Output;);

                $crate::doc!($($doc)*,
                fn $op(self) -> $Output {
                    let $self = self;
                    $body

                });
            });
        )*
    };
}
