/// Impl `Styler` ops.
#[macro_export]
macro_rules! impl_styler_mut_ops {
    ($(<$($G1:ident $(: $($B1:path)+)?,)+>)? $Self:path) => {
        $crate::__impl_styler_mut_ops!(trait binary (s: $Self) {
            "Sets `Option<Foreground>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?
                Color: std::convert::Into<std::option::Option<$crate::Foreground>>,>
            MulAssign(mul_assign) (foreground: Color) {
                $crate::StylerMut::foreground_mut(s, foreground)
            }
            "Sets `Option<Background>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?
                Color: std::convert::Into<std::option::Option<$crate::Background>>,>
            DivAssign(div_assign) (background: Color) {
                $crate::StylerMut::background_mut(s, background)
            }

            "Sets `Option<Foreground>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (foreground: $crate::Foreground) {
                $crate::StylerMut::foreground_mut(s, Some(foreground))
            }
            "Sets foreground to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoForeground) {
                $crate::StylerMut::foreground_mut(s, None)
            }

            "Sets `Option<Background>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (background: $crate::Background) {
                $crate::StylerMut::background_mut(s, Some(background))
            }
            "Sets background to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoBackground) {
                $crate::StylerMut::background_mut(s, None)
            }

            "Sets `Option<Weight>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (weight: $crate::Weight) {
                $crate::StylerMut::weight_mut(s, Some(weight))
            }
            "Sets weight to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoWeight) {
                $crate::StylerMut::weight_mut(s, None)
            }

            "Sets `Option<Slant>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (slant: $crate::Slant) {
                $crate::StylerMut::slant_mut(s, Some(slant))
            }
            "Sets slant to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoSlant) {
                $crate::StylerMut::slant_mut(s, None)
            }

            "Sets `Option<Blink>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (blink: $crate::Blink) {
                $crate::StylerMut::blink_mut(s, Some(blink))
            }
            "Sets blink to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoBlink) {
                $crate::StylerMut::blink_mut(s, None)
            }

            "Sets `Option<Invert>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (invert: $crate::Invert) {
                $crate::StylerMut::invert_mut(s, Some(invert))
            }
            "Sets invert to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoInvert) {
                $crate::StylerMut::invert_mut(s, None)
            }

            "Sets `Option<Strike>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (strike: $crate::Strike) {
                $crate::StylerMut::strike_mut(s, Some(strike))
            }
            "Sets strike to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoStrike) {
                $crate::StylerMut::strike_mut(s, None)
            }

            "Sets `Option<Underline>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (underline: $crate::Underline) {
                $crate::StylerMut::underline_mut(s, Some(underline))
            }
            "Sets underline to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoUnderline) {
                $crate::StylerMut::underline_mut(s, None)
            }

            "Sets `Option<Overline>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (overline: $crate::Overline) {
                $crate::StylerMut::overline_mut(s, Some(overline))
            }
            "Sets overline to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoOverline) {
                $crate::StylerMut::overline_mut(s, None)
            }

            "Sets `Option<Border>` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (border: $crate::Border) {
                $crate::StylerMut::border_mut(s, Some(border))
            }
            "Sets border to `None` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            AddAssign(add_assign) (_: $crate::NoBorder) {
                $crate::StylerMut::border_mut(s, None)
            }

            "`Option::and` fields mutably.",
            <$($($G1 $(: $($B1)+)?,)+)? Styler: $crate::Styler,>
            BitAndAssign(bitand_assign) (styler: &Styler) {
                $crate::StylerMut::and_mut(s, styler)
            }
            "`Option::or` fields mutably.",
            <$($($G1 $(: $($B1)+)?,)+)? Styler: $crate::Styler,>
            BitOrAssign(bitor_assign) (styler: &Styler) {
                $crate::StylerMut::or_mut(s, styler)
            }
            "`Option::xor` fields mutably.",
            <$($($G1 $(: $($B1)+)?,)+)? Styler: $crate::Styler,>
            BitXorAssign(bitxor_assign) (styler: &Styler) {
                $crate::StylerMut::xor_mut(s, styler)
            }

            "Dedups (`None`s if identicals) fields mutably.",
            <$($($G1 $(: $($B1)+)?,)+)? Styler: $crate::Styler,>
            RemAssign(rem_assign) (styler: &Styler) {
                $crate::StylerMut::dedup_mut(s, styler)
            }
        });

        $crate::__impl_styler_mut_ops!(trait unary (s: $Self) {
            "Resets (sets to reset value) fields which are `Some` mutably.",
            <$($($G1 $(: $($B1)+)?,)+)?>
            Not(not) {
                $crate::StylerMut::reset_mut(s)
            }
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler_mut_ops {
    (trait binary ($self:ident: $Self:path) { $(
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
    (trait unary ($self:ident: $Self:path) { $(
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
