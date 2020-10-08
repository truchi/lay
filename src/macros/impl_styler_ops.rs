/// Impl `Styler`/`StylerMut` ops.
#[macro_export]
macro_rules! impl_styler_ops {
    (
        mut
        // Generics and corresponding bounds
        $(<($($bounds:tt)+)>)?
        // Type
        $Self:path
    ) => {
        $crate::priv_impl_styler_ops!(mut $(<($($bounds)+)>)? $Self);
    };
    (
        // Generics and corresponding bounds
        $(<($($bounds:tt)+)>)?
        // Type
        $Self:path
    ) => {
        $crate::priv_impl_styler_ops!($(<($($bounds)+)>)? (s: $Self) -> $Self { s });

        $crate::priv_impl_styler_ops!(trait unary (s: $Self) -> $Self {
            "Resets (sets to reset value) fields which are `Some`.",
            $(<($($bounds)+)>)?
            Not(not) {
                $crate::Styler::reset(s)
            }
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! priv_impl_styler_ops {
    (trait unary ($self:tt: $Self:path) -> $Output:path { $(
        $($doc:expr)*,
        $(<($($bounds:tt)+)>)?
        $Op:ident($op:ident) $body:expr
    )* }) => {
        $($crate::doc!($($doc)*,
        impl $(<$($bounds)+>)? ::std::ops::$Op for $Self {
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
        $(<($($bounds:tt)+)>)?
        $Op:ident($op:ident) $body:expr
    )* }) => {
        $($crate::doc!($($doc)*,
        impl $(<$($bounds)+>)? ::std::ops::$Op for &mut $Self {
            $crate::doc!("`()`.",
            type Output = (););

            $crate::doc!($($doc)*,
            fn $op(self) {
                let $self = self;
                $body
            });
        });)*
    };

    (trait binary ($self:tt: $Self:path) -> $Output:path { $(
        $($doc:expr)*,
        $(<($($bounds:tt)+)>)? $Op:ident($op:ident)
        ($rhs:tt: $($Rhs:tt)*) $body:expr
    )* }) => {
        $($crate::doc!($($doc)*,
        impl $(<$($bounds)+>)? ::std::ops::$Op<$($Rhs)*> for $Self {
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
        $(<($($bounds:tt)+)>)? $OpAssign:ident($op_assign:ident)
        ($rhs:tt: $($Rhs:tt)*) $body:expr
    )* }) => {
        $($crate::doc!($($doc)*,
        impl $(<$($bounds)+>)? ::std::ops::$OpAssign<$($Rhs)*> for $Self {
            $crate::doc!($($doc)*,
            fn $op_assign(&mut self, $rhs: $($Rhs)*) {
                let $self = self;
                $body
            });
        });)*
    };

    ($(<($($bounds:tt)+)>)? ($self:tt: $Self:path) -> $Output:path $block:block) => {
        $crate::priv_impl_styler_ops!(trait binary ($self: $Self) -> $Output {
            "Sets `Option<Foreground>`.",
            <($($($bounds)+,)?
                C: std::convert::Into<std::option::Option<$crate::Foreground>>)>
            Mul(mul) (foreground: C) {
                $crate::Styler::foreground($block, foreground)
            }
            "Sets `Option<Background>`.",
            <($($($bounds)+,)?
                C: std::convert::Into<std::option::Option<$crate::Background>>)>
            Div(div) (background: C) {
                $crate::Styler::background($block, background)
            }

            "Sets `Option<Foreground>`.",
            $(<($($bounds)+)>)?
            Add(add) (foreground: $crate::Foreground) {
                $crate::Styler::foreground($block, Some(foreground))
            }
            "Sets foreground to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoForeground) {
                $crate::Styler::foreground($block, None)
            }

            "Sets `Option<Background>`.",
            $(<($($bounds)+)>)?
            Add(add) (background: $crate::Background) {
                $crate::Styler::background($block, Some(background))
            }
            "Sets background to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoBackground) {
                $crate::Styler::background($block, None)
            }

            "Sets `Option<Weight>`.",
            $(<($($bounds)+)>)?
            Add(add) (weight: $crate::Weight) {
                $crate::Styler::weight($block, Some(weight))
            }
            "Sets weight to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoWeight) {
                $crate::Styler::weight($block, None)
            }

            "Sets `Option<Slant>`.",
            $(<($($bounds)+)>)?
            Add(add) (slant: $crate::Slant) {
                $crate::Styler::slant($block, Some(slant))
            }
            "Sets slant to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoSlant) {
                $crate::Styler::slant($block, None)
            }

            "Sets `Option<Blink>`.",
            $(<($($bounds)+)>)?
            Add(add) (blink: $crate::Blink) {
                $crate::Styler::blink($block, Some(blink))
            }
            "Sets blink to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoBlink) {
                $crate::Styler::blink($block, None)
            }

            "Sets `Option<Invert>`.",
            $(<($($bounds)+)>)?
            Add(add) (invert: $crate::Invert) {
                $crate::Styler::invert($block, Some(invert))
            }
            "Sets invert to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoInvert) {
                $crate::Styler::invert($block, None)
            }

            "Sets `Option<Strike>`.",
            $(<($($bounds)+)>)?
            Add(add) (strike: $crate::Strike) {
                $crate::Styler::strike($block, Some(strike))
            }
            "Sets strike to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoStrike) {
                $crate::Styler::strike($block, None)
            }

            "Sets `Option<Underline>`.",
            $(<($($bounds)+)>)?
            Add(add) (underline: $crate::Underline) {
                $crate::Styler::underline($block, Some(underline))
            }
            "Sets underline to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoUnderline) {
                $crate::Styler::underline($block, None)
            }

            "Sets `Option<Overline>`.",
            $(<($($bounds)+)>)?
            Add(add) (overline: $crate::Overline) {
                $crate::Styler::overline($block, Some(overline))
            }
            "Sets overline to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoOverline) {
                $crate::Styler::overline($block, None)
            }

            "Sets `Option<Border>`.",
            $(<($($bounds)+)>)?
            Add(add) (border: $crate::Border) {
                $crate::Styler::border($block, Some(border))
            }
            "Sets border to `None`.",
            $(<($($bounds)+)>)?
            Add(add) (_: $crate::NoBorder) {
                $crate::Styler::border($block, None)
            }

            "`Option::and` fields.",
            <($($($bounds)+,)? Styler: $crate::Styler)>
            BitAnd(bitand) (styler: &Styler) {
                $crate::Styler::and($block, styler)
            }
            "`Option::or` fields.",
            <($($($bounds)+,)? Styler: $crate::Styler)>
            BitOr(bitor) (styler: &Styler) {
                $crate::Styler::or($block, styler)
            }
            "`Option::xor` fields.",
            <($($($bounds)+,)? Styler: $crate::Styler)>
            BitXor(bitxor) (styler: &Styler) {
                $crate::Styler::xor($block, styler)
            }

            "Dedups (`None`s if identicals) fields.",
            <($($($bounds)+,)? Styler: $crate::Styler)>
            Rem(rem) (styler: &Styler) {
                $crate::Styler::dedup($block, styler)
            }
        });
    };
    (mut $(<($($bounds:tt)+)>)? $Self:path) => {
        $crate::priv_impl_styler_ops!(mut trait binary (s: $Self) {
            "Sets `Option<Foreground>` mutably.",
            <($($($bounds)+,)?
                C: std::convert::Into<std::option::Option<$crate::Foreground>>)>
            MulAssign(mul_assign) (foreground: C) {
                $crate::StylerMut::foreground_mut(s, foreground)
            }
            "Sets `Option<Background>` mutably.",
            <($($($bounds)+,)?
                C: std::convert::Into<std::option::Option<$crate::Background>>)>
            DivAssign(div_assign) (background: C) {
                $crate::StylerMut::background_mut(s, background)
            }

            "Sets `Option<Foreground>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (foreground: $crate::Foreground) {
                $crate::StylerMut::foreground_mut(s, Some(foreground))
            }
            "Sets foreground to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoForeground) {
                $crate::StylerMut::foreground_mut(s, None)
            }

            "Sets `Option<Background>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (background: $crate::Background) {
                $crate::StylerMut::background_mut(s, Some(background))
            }
            "Sets background to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoBackground) {
                $crate::StylerMut::background_mut(s, None)
            }

            "Sets `Option<Weight>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (weight: $crate::Weight) {
                $crate::StylerMut::weight_mut(s, Some(weight))
            }
            "Sets weight to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoWeight) {
                $crate::StylerMut::weight_mut(s, None)
            }

            "Sets `Option<Slant>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (slant: $crate::Slant) {
                $crate::StylerMut::slant_mut(s, Some(slant))
            }
            "Sets slant to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoSlant) {
                $crate::StylerMut::slant_mut(s, None)
            }

            "Sets `Option<Blink>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (blink: $crate::Blink) {
                $crate::StylerMut::blink_mut(s, Some(blink))
            }
            "Sets blink to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoBlink) {
                $crate::StylerMut::blink_mut(s, None)
            }

            "Sets `Option<Invert>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (invert: $crate::Invert) {
                $crate::StylerMut::invert_mut(s, Some(invert))
            }
            "Sets invert to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoInvert) {
                $crate::StylerMut::invert_mut(s, None)
            }

            "Sets `Option<Strike>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (strike: $crate::Strike) {
                $crate::StylerMut::strike_mut(s, Some(strike))
            }
            "Sets strike to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoStrike) {
                $crate::StylerMut::strike_mut(s, None)
            }

            "Sets `Option<Underline>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (underline: $crate::Underline) {
                $crate::StylerMut::underline_mut(s, Some(underline))
            }
            "Sets underline to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoUnderline) {
                $crate::StylerMut::underline_mut(s, None)
            }

            "Sets `Option<Overline>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (overline: $crate::Overline) {
                $crate::StylerMut::overline_mut(s, Some(overline))
            }
            "Sets overline to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoOverline) {
                $crate::StylerMut::overline_mut(s, None)
            }

            "Sets `Option<Border>` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (border: $crate::Border) {
                $crate::StylerMut::border_mut(s, Some(border))
            }
            "Sets border to `None` mutably.",
            $(<($($bounds)+)>)?
            AddAssign(add_assign) (_: $crate::NoBorder) {
                $crate::StylerMut::border_mut(s, None)
            }

            "`Option::and` fields mutably.",
            <($($($bounds)+,)? Styler: $crate::Styler)>
            BitAndAssign(bitand_assign) (styler: &Styler) {
                $crate::StylerMut::and_mut(s, styler)
            }
            "`Option::or` fields mutably.",
            <($($($bounds)+,)? Styler: $crate::Styler)>
            BitOrAssign(bitor_assign) (styler: &Styler) {
                $crate::StylerMut::or_mut(s, styler)
            }
            "`Option::xor` fields mutably.",
            <($($($bounds)+,)? Styler: $crate::Styler)>
            BitXorAssign(bitxor_assign) (styler: &Styler) {
                $crate::StylerMut::xor_mut(s, styler)
            }

            "Dedups (`None`s if identicals) fields mutably.",
            <($($($bounds)+,)? Styler: $crate::Styler)>
            RemAssign(rem_assign) (styler: &Styler) {
                $crate::StylerMut::dedup_mut(s, styler)
            }
        });

        $crate::priv_impl_styler_ops!(mut trait unary (s: $Self) {
            "Resets (sets to reset value) fields which are `Some` mutably.",
            $(<($($bounds)+)>)?
            Not(not) {
                $crate::StylerMut::reset_mut(s)
            }
        });
    };
}
