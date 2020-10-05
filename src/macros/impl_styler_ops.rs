/// Impl `Styler`/`StylerMut` ops.
#[macro_export]
macro_rules! impl_styler_ops {
    (
        mut
        // Generics and corresponding bounds
        $(<$($G:ident $(: $($B:path)+)?,)+>)?
        // Type
        $Self:path
    ) => {
        $crate::priv_impl_styler_ops!(mut <$($($G $(: $($B)+)?,)+)?> $Self);
    };
    (
        // Generics and corresponding bounds
        $(<$($G:ident $(: $($B:path)+)?,)+>)?
        // Type
        $Self:path
    ) => {
        $crate::priv_impl_styler_ops!(<$($($G $(: $($B)+)?,)+)?> $Self);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! priv_impl_styler_ops {
    (<$($G1:ident $(: $($B1:path)+)?,)*> $Self:path) => {
        $crate::priv_impl_styler_ops!(trait binary (s: $Self) -> $Self {
            "Sets `Option<Foreground>`.",
            <$($G1 $(: $($B1)+)?,)*
                Color: std::convert::Into<std::option::Option<$crate::Foreground>>,>
            Mul(mul) (foreground: Color) {
                $crate::Styler::foreground(s, foreground)
            }
            "Sets `Option<Background>`.",
            <$($G1 $(: $($B1)+)?,)*
                Color: std::convert::Into<std::option::Option<$crate::Background>>,>
            Div(div) (background: Color) {
                $crate::Styler::background(s, background)
            }

            "Sets `Option<Foreground>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (foreground: $crate::Foreground) {
                $crate::Styler::foreground(s, Some(foreground))
            }
            "Sets foreground to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoForeground) {
                $crate::Styler::foreground(s, None)
            }

            "Sets `Option<Background>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (background: $crate::Background) {
                $crate::Styler::background(s, Some(background))
            }
            "Sets background to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoBackground) {
                $crate::Styler::background(s, None)
            }

            "Sets `Option<Weight>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (weight: $crate::Weight) {
                $crate::Styler::weight(s, Some(weight))
            }
            "Sets weight to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoWeight) {
                $crate::Styler::weight(s, None)
            }

            "Sets `Option<Slant>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (slant: $crate::Slant) {
                $crate::Styler::slant(s, Some(slant))
            }
            "Sets slant to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoSlant) {
                $crate::Styler::slant(s, None)
            }

            "Sets `Option<Blink>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (blink: $crate::Blink) {
                $crate::Styler::blink(s, Some(blink))
            }
            "Sets blink to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoBlink) {
                $crate::Styler::blink(s, None)
            }

            "Sets `Option<Invert>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (invert: $crate::Invert) {
                $crate::Styler::invert(s, Some(invert))
            }
            "Sets invert to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoInvert) {
                $crate::Styler::invert(s, None)
            }

            "Sets `Option<Strike>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (strike: $crate::Strike) {
                $crate::Styler::strike(s, Some(strike))
            }
            "Sets strike to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoStrike) {
                $crate::Styler::strike(s, None)
            }

            "Sets `Option<Underline>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (underline: $crate::Underline) {
                $crate::Styler::underline(s, Some(underline))
            }
            "Sets underline to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoUnderline) {
                $crate::Styler::underline(s, None)
            }

            "Sets `Option<Overline>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (overline: $crate::Overline) {
                $crate::Styler::overline(s, Some(overline))
            }
            "Sets overline to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoOverline) {
                $crate::Styler::overline(s, None)
            }

            "Sets `Option<Border>`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (border: $crate::Border) {
                $crate::Styler::border(s, Some(border))
            }
            "Sets border to `None`.",
            <$($G1 $(: $($B1)+)?,)*>
            Add(add) (_: $crate::NoBorder) {
                $crate::Styler::border(s, None)
            }

            "`Option::and` fields.",
            <$($G1 $(: $($B1)+)?,)* Styler: $crate::Styler,>
            BitAnd(bitand) (styler: &Styler) {
                $crate::Styler::and(s, styler)
            }
            "`Option::or` fields.",
            <$($G1 $(: $($B1)+)?,)* Styler: $crate::Styler,>
            BitOr(bitor) (styler: &Styler) {
                $crate::Styler::or(s, styler)
            }
            "`Option::xor` fields.",
            <$($G1 $(: $($B1)+)?,)* Styler: $crate::Styler,>
            BitXor(bitxor) (styler: &Styler) {
                $crate::Styler::xor(s, styler)
            }

            "Dedups (`None`s if identicals) fields.",
            <$($G1 $(: $($B1)+)?,)* Styler: $crate::Styler,>
            Rem(rem) (styler: &Styler) {
                $crate::Styler::dedup(s, styler)
            }
        });

        $crate::priv_impl_styler_ops!(trait unary (s: $Self) -> $Self {
            "Resets (sets to reset value) fields which are `Some`.",
            <$($G1 $(: $($B1)+)?,)*>
            Not(not) {
                $crate::Styler::reset(s)
            }
        });
    };
    (mut <$($G1:ident $(: $($B1:path)+)?,)*> $Self:path) => {
        $crate::priv_impl_styler_ops!(mut trait binary (s: $Self) {
            "Sets `Option<Foreground>` mutably.",
            <$($G1 $(: $($B1)+)?,)*
                Color: std::convert::Into<std::option::Option<$crate::Foreground>>,>
            MulAssign(mul_assign) (foreground: Color) {
                $crate::StylerMut::foreground_mut(s, foreground)
            }
            "Sets `Option<Background>` mutably.",
            <$($G1 $(: $($B1)+)?,)*
                Color: std::convert::Into<std::option::Option<$crate::Background>>,>
            DivAssign(div_assign) (background: Color) {
                $crate::StylerMut::background_mut(s, background)
            }

            "Sets `Option<Foreground>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (foreground: $crate::Foreground) {
                $crate::StylerMut::foreground_mut(s, Some(foreground))
            }
            "Sets foreground to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoForeground) {
                $crate::StylerMut::foreground_mut(s, None)
            }

            "Sets `Option<Background>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (background: $crate::Background) {
                $crate::StylerMut::background_mut(s, Some(background))
            }
            "Sets background to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoBackground) {
                $crate::StylerMut::background_mut(s, None)
            }

            "Sets `Option<Weight>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (weight: $crate::Weight) {
                $crate::StylerMut::weight_mut(s, Some(weight))
            }
            "Sets weight to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoWeight) {
                $crate::StylerMut::weight_mut(s, None)
            }

            "Sets `Option<Slant>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (slant: $crate::Slant) {
                $crate::StylerMut::slant_mut(s, Some(slant))
            }
            "Sets slant to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoSlant) {
                $crate::StylerMut::slant_mut(s, None)
            }

            "Sets `Option<Blink>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (blink: $crate::Blink) {
                $crate::StylerMut::blink_mut(s, Some(blink))
            }
            "Sets blink to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoBlink) {
                $crate::StylerMut::blink_mut(s, None)
            }

            "Sets `Option<Invert>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (invert: $crate::Invert) {
                $crate::StylerMut::invert_mut(s, Some(invert))
            }
            "Sets invert to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoInvert) {
                $crate::StylerMut::invert_mut(s, None)
            }

            "Sets `Option<Strike>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (strike: $crate::Strike) {
                $crate::StylerMut::strike_mut(s, Some(strike))
            }
            "Sets strike to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoStrike) {
                $crate::StylerMut::strike_mut(s, None)
            }

            "Sets `Option<Underline>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (underline: $crate::Underline) {
                $crate::StylerMut::underline_mut(s, Some(underline))
            }
            "Sets underline to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoUnderline) {
                $crate::StylerMut::underline_mut(s, None)
            }

            "Sets `Option<Overline>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (overline: $crate::Overline) {
                $crate::StylerMut::overline_mut(s, Some(overline))
            }
            "Sets overline to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoOverline) {
                $crate::StylerMut::overline_mut(s, None)
            }

            "Sets `Option<Border>` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (border: $crate::Border) {
                $crate::StylerMut::border_mut(s, Some(border))
            }
            "Sets border to `None` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoBorder) {
                $crate::StylerMut::border_mut(s, None)
            }

            "`Option::and` fields mutably.",
            <$($G1 $(: $($B1)+)?,)* Styler: $crate::Styler,>
            BitAndAssign(bitand_assign) (styler: &Styler) {
                $crate::StylerMut::and_mut(s, styler)
            }
            "`Option::or` fields mutably.",
            <$($G1 $(: $($B1)+)?,)* Styler: $crate::Styler,>
            BitOrAssign(bitor_assign) (styler: &Styler) {
                $crate::StylerMut::or_mut(s, styler)
            }
            "`Option::xor` fields mutably.",
            <$($G1 $(: $($B1)+)?,)* Styler: $crate::Styler,>
            BitXorAssign(bitxor_assign) (styler: &Styler) {
                $crate::StylerMut::xor_mut(s, styler)
            }

            "Dedups (`None`s if identicals) fields mutably.",
            <$($G1 $(: $($B1)+)?,)* Styler: $crate::Styler,>
            RemAssign(rem_assign) (styler: &Styler) {
                $crate::StylerMut::dedup_mut(s, styler)
            }
        });

        $crate::priv_impl_styler_ops!(mut trait unary (s: $Self) {
            "Resets (sets to reset value) fields which are `Some` mutably.",
            <$($G1 $(: $($B1)+)?,)*>
            Not(not) {
                $crate::StylerMut::reset_mut(s)
            }
        });
    };

    (trait binary ($self:tt: $Self:path) -> $Output:path { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*> $Op:ident($op:ident)
        ($rhs:tt: $($Rhs:tt)*) $body:expr
    )* }) => {
        $($crate::doc!($($doc)*,
        impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op<$($Rhs)*> for $Self {
            $crate::doc!("`" stringify!($Output) "`.",
            type Output = $Output;);

            $crate::doc!($($doc)*,
            fn $op(self, $rhs: $($Rhs)*) -> $Output {
                let $self = self;
                $body
            });
        });)*
    };
    (mut trait binary ($self:tt: $Self:path) { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*>
        $OpAssign:ident($op_assign:ident)
        ($rhs:tt: $($Rhs:tt)*) $body:expr
    )* }) => {
        $(
            $crate::doc!($($doc)*,
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$OpAssign<$($Rhs)*> for $Self {
                $crate::doc!($($doc)*,
                fn $op_assign(&mut self, $rhs: $($Rhs)*) {
                    let $self = self;
                    $body
                });
            });
        )*
    };

    (trait unary ($self:tt: $Self:path) -> $Output:path { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $body:expr
    )* }) => {
        $($crate::doc!($($doc)*,
        impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for $Self {
            $crate::doc!("`" stringify!($Output) "`.",
            type Output = $Output;);

            $crate::doc!($($doc)*,
            fn $op(self) -> $Output {
                let $self = self;
                $body

            });
        });)*
    };
    (mut trait unary ($self:tt: $Self:path) { $(
        $($doc:expr)*,
        <$($G:ident $(: $($B:path)+)?,)*>
        $Op:ident($op:ident) $body:expr
    )* }) => {
        $(
            $crate::doc!($($doc)*,
            impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for &mut $Self {
                $crate::doc!("`()`.",
                type Output = (););

                $crate::doc!($($doc)*,
                fn $op(self) {
                    let $self = self;
                    $body
                });
            });
        )*
    };
}
