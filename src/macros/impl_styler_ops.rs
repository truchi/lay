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
        $crate::priv_impl_styler_ops!(<$($($G $(: $($B)+)?,)+)?> (s: $Self) -> $Self);

        $crate::priv_impl_styler_ops!(trait unary (s: $Self) -> $Self {
            "Resets (sets to reset value) fields which are `Some`.",
            <$($($G $(: $($B)+)?,)+)?>
            Not(not) {
                $crate::Styler::reset(s)
            }
        });
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! priv_impl_styler_ops {
    (<$($G:ident $(: $($B:path)+)?,)*> ($self:tt: $Self:path) -> $Output:path $(, $expr:expr)?) => {
        $crate::priv_impl_styler_ops!(trait binary ($self: $Self) -> $Output {
            "Sets `Option<Foreground>`.",
            <$($G $(: $($B)+)?,)*
                C: std::convert::Into<std::option::Option<$crate::Foreground>>,>
            Mul(mul) (foreground: C) {
                $(let $self = $expr;)?
                $crate::Styler::foreground($self, foreground)
            }
            "Sets `Option<Background>`.",
            <$($G $(: $($B)+)?,)*
                C: std::convert::Into<std::option::Option<$crate::Background>>,>
            Div(div) (background: C) {
                $(let $self = $expr;)?
                $crate::Styler::background($self, background)
            }

            "Sets `Option<Foreground>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (foreground: $crate::Foreground) {
                $(let $self = $expr;)?
                $crate::Styler::foreground($self, Some(foreground))
            }
            "Sets foreground to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoForeground) {
                $(let $self = $expr;)?
                $crate::Styler::foreground($self, None)
            }

            "Sets `Option<Background>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (background: $crate::Background) {
                $(let $self = $expr;)?
                $crate::Styler::background($self, Some(background))
            }
            "Sets background to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoBackground) {
                $(let $self = $expr;)?
                $crate::Styler::background($self, None)
            }

            "Sets `Option<Weight>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (weight: $crate::Weight) {
                $(let $self = $expr;)?
                $crate::Styler::weight($self, Some(weight))
            }
            "Sets weight to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoWeight) {
                $(let $self = $expr;)?
                $crate::Styler::weight($self, None)
            }

            "Sets `Option<Slant>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (slant: $crate::Slant) {
                $(let $self = $expr;)?
                $crate::Styler::slant($self, Some(slant))
            }
            "Sets slant to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoSlant) {
                $(let $self = $expr;)?
                $crate::Styler::slant($self, None)
            }

            "Sets `Option<Blink>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (blink: $crate::Blink) {
                $(let $self = $expr;)?
                $crate::Styler::blink($self, Some(blink))
            }
            "Sets blink to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoBlink) {
                $(let $self = $expr;)?
                $crate::Styler::blink($self, None)
            }

            "Sets `Option<Invert>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (invert: $crate::Invert) {
                $(let $self = $expr;)?
                $crate::Styler::invert($self, Some(invert))
            }
            "Sets invert to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoInvert) {
                $(let $self = $expr;)?
                $crate::Styler::invert($self, None)
            }

            "Sets `Option<Strike>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (strike: $crate::Strike) {
                $(let $self = $expr;)?
                $crate::Styler::strike($self, Some(strike))
            }
            "Sets strike to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoStrike) {
                $(let $self = $expr;)?
                $crate::Styler::strike($self, None)
            }

            "Sets `Option<Underline>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (underline: $crate::Underline) {
                $(let $self = $expr;)?
                $crate::Styler::underline($self, Some(underline))
            }
            "Sets underline to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoUnderline) {
                $(let $self = $expr;)?
                $crate::Styler::underline($self, None)
            }

            "Sets `Option<Overline>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (overline: $crate::Overline) {
                $(let $self = $expr;)?
                $crate::Styler::overline($self, Some(overline))
            }
            "Sets overline to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoOverline) {
                $(let $self = $expr;)?
                $crate::Styler::overline($self, None)
            }

            "Sets `Option<Border>`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (border: $crate::Border) {
                $(let $self = $expr;)?
                $crate::Styler::border($self, Some(border))
            }
            "Sets border to `None`.",
            <$($G $(: $($B)+)?,)*>
            Add(add) (_: $crate::NoBorder) {
                $(let $self = $expr;)?
                $crate::Styler::border($self, None)
            }

            "`Option::and` fields.",
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            BitAnd(bitand) (styler: &Styler) {
                $(let $self = $expr;)?
                $crate::Styler::and($self, styler)
            }
            "`Option::or` fields.",
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            BitOr(bitor) (styler: &Styler) {
                $(let $self = $expr;)?
                $crate::Styler::or($self, styler)
            }
            "`Option::xor` fields.",
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            BitXor(bitxor) (styler: &Styler) {
                $(let $self = $expr;)?
                $crate::Styler::xor($self, styler)
            }

            "Dedups (`None`s if identicals) fields.",
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            Rem(rem) (styler: &Styler) {
                $(let $self = $expr;)?
                $crate::Styler::dedup($self, styler)
            }
        });
    };
    (mut <$($G:ident $(: $($B:path)+)?,)*> $Self:path) => {
        $crate::priv_impl_styler_ops!(mut trait binary (s: $Self) {
            "Sets `Option<Foreground>` mutably.",
            <$($G $(: $($B)+)?,)*
                C: std::convert::Into<std::option::Option<$crate::Foreground>>,>
            MulAssign(mul_assign) (foreground: C) {
                $crate::StylerMut::foreground_mut(s, foreground)
            }
            "Sets `Option<Background>` mutably.",
            <$($G $(: $($B)+)?,)*
                C: std::convert::Into<std::option::Option<$crate::Background>>,>
            DivAssign(div_assign) (background: C) {
                $crate::StylerMut::background_mut(s, background)
            }

            "Sets `Option<Foreground>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (foreground: $crate::Foreground) {
                $crate::StylerMut::foreground_mut(s, Some(foreground))
            }
            "Sets foreground to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoForeground) {
                $crate::StylerMut::foreground_mut(s, None)
            }

            "Sets `Option<Background>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (background: $crate::Background) {
                $crate::StylerMut::background_mut(s, Some(background))
            }
            "Sets background to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoBackground) {
                $crate::StylerMut::background_mut(s, None)
            }

            "Sets `Option<Weight>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (weight: $crate::Weight) {
                $crate::StylerMut::weight_mut(s, Some(weight))
            }
            "Sets weight to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoWeight) {
                $crate::StylerMut::weight_mut(s, None)
            }

            "Sets `Option<Slant>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (slant: $crate::Slant) {
                $crate::StylerMut::slant_mut(s, Some(slant))
            }
            "Sets slant to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoSlant) {
                $crate::StylerMut::slant_mut(s, None)
            }

            "Sets `Option<Blink>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (blink: $crate::Blink) {
                $crate::StylerMut::blink_mut(s, Some(blink))
            }
            "Sets blink to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoBlink) {
                $crate::StylerMut::blink_mut(s, None)
            }

            "Sets `Option<Invert>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (invert: $crate::Invert) {
                $crate::StylerMut::invert_mut(s, Some(invert))
            }
            "Sets invert to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoInvert) {
                $crate::StylerMut::invert_mut(s, None)
            }

            "Sets `Option<Strike>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (strike: $crate::Strike) {
                $crate::StylerMut::strike_mut(s, Some(strike))
            }
            "Sets strike to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoStrike) {
                $crate::StylerMut::strike_mut(s, None)
            }

            "Sets `Option<Underline>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (underline: $crate::Underline) {
                $crate::StylerMut::underline_mut(s, Some(underline))
            }
            "Sets underline to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoUnderline) {
                $crate::StylerMut::underline_mut(s, None)
            }

            "Sets `Option<Overline>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (overline: $crate::Overline) {
                $crate::StylerMut::overline_mut(s, Some(overline))
            }
            "Sets overline to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoOverline) {
                $crate::StylerMut::overline_mut(s, None)
            }

            "Sets `Option<Border>` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (border: $crate::Border) {
                $crate::StylerMut::border_mut(s, Some(border))
            }
            "Sets border to `None` mutably.",
            <$($G $(: $($B)+)?,)*>
            AddAssign(add_assign) (_: $crate::NoBorder) {
                $crate::StylerMut::border_mut(s, None)
            }

            "`Option::and` fields mutably.",
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            BitAndAssign(bitand_assign) (styler: &Styler) {
                $crate::StylerMut::and_mut(s, styler)
            }
            "`Option::or` fields mutably.",
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            BitOrAssign(bitor_assign) (styler: &Styler) {
                $crate::StylerMut::or_mut(s, styler)
            }
            "`Option::xor` fields mutably.",
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            BitXorAssign(bitxor_assign) (styler: &Styler) {
                $crate::StylerMut::xor_mut(s, styler)
            }

            "Dedups (`None`s if identicals) fields mutably.",
            <$($G $(: $($B)+)?,)* Styler: $crate::Styler,>
            RemAssign(rem_assign) (styler: &Styler) {
                $crate::StylerMut::dedup_mut(s, styler)
            }
        });

        $crate::priv_impl_styler_ops!(mut trait unary (s: $Self) {
            "Resets (sets to reset value) fields which are `Some` mutably.",
            <$($G $(: $($B)+)?,)*>
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
        $($crate::doc!($($doc)*,
        impl<$($G $(: $($B+)+,)?)*> ::std::ops::$OpAssign<$($Rhs)*> for $Self {
            $crate::doc!($($doc)*,
            fn $op_assign(&mut self, $rhs: $($Rhs)*) {
                let $self = self;
                $body
            });
        });)*
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
        $($crate::doc!($($doc)*,
        impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Op for &mut $Self {
            $crate::doc!("`()`.",
            type Output = (););

            $crate::doc!($($doc)*,
            fn $op(self) {
                let $self = self;
                $body
            });
        });)*
    };
}
