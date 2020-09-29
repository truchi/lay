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
        impl_styler!(impl field $(<$($G $(: $($B)+)?,)+>)? ($self: $Type) {
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
        impl_styler!(impl method $(<$($G $(: $($B)+)?,)+>)? ($self: $Type) {
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
    (impl $mode:ident $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
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
            impl_styler!(impl $mode Foreground get_foreground foreground_mut $self $foreground);
            impl_styler!(impl $mode Background get_background background_mut $self $background);
            impl_styler!(impl $mode Weighted get_weighted weighted_mut $self $weight);
            impl_styler!(impl $mode Slanted get_slanted slanted_mut $self $slant);
            impl_styler!(impl $mode Blinking get_blinking blinking_mut $self $blink);
            impl_styler!(impl $mode Inverted get_inverted inverted_mut $self $invert);
            impl_styler!(impl $mode Striked get_striked striked_mut $self $strike);
            impl_styler!(impl $mode Underlined get_underlined underlined_mut $self $underline);
            impl_styler!(impl $mode Overlined get_overlined overlined_mut $self $overline);
            impl_styler!(impl $mode Bordered get_bordered bordered_mut $self $border);
        }

        impl_styler!(impl ops $Type: $(<$($G $(: $($B)+)?,)+>)?);
    };
    (impl field $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $field
        }

        fn $set_mut(&mut self, arg: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $field = arg.into();
        }
    };
    (impl method $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
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
    (impl ops $Type:ty: $(<$($G:ident $(: $($B:path)+)?,)+>)?) => {
        impl_styler!(impl ops $Type {
            <$($($G $(: $($B)+)?,)+)?> [*] foreground_mut (foreground: Color      NoColor     ) Some($crate::Foreground(foreground));
            <$($($G $(: $($B)+)?,)+)?> [/] background_mut (background: Color      NoColor     ) Some($crate::Background(background));
            <$($($G $(: $($B)+)?,)+)?> [+] foreground_mut (foreground: Foreground NoForeground) Some(foreground);
            <$($($G $(: $($B)+)?,)+)?> [+] background_mut (background: Background NoBackground) Some(background);
            <$($($G $(: $($B)+)?,)+)?> [+] weighted_mut   (weighted  : Weighted   NoWeight    ) Some(weighted);
            <$($($G $(: $($B)+)?,)+)?> [+] slanted_mut    (slanted   : Slanted    NoSlant     ) Some(slanted);
            <$($($G $(: $($B)+)?,)+)?> [+] blinking_mut   (blinking  : Blinking   NoBlink     ) Some(blinking);
            <$($($G $(: $($B)+)?,)+)?> [+] inverted_mut   (inverted  : Inverted   NoInvert    ) Some(inverted);
            <$($($G $(: $($B)+)?,)+)?> [+] striked_mut    (striked   : Striked    NoStrike    ) Some(striked);
            <$($($G $(: $($B)+)?,)+)?> [+] underlined_mut (underlined: Underlined NoUnderline ) Some(underlined);
            <$($($G $(: $($B)+)?,)+)?> [+] overlined_mut  (overlined : Overlined  NoOverline  ) Some(overlined);
            <$($($G $(: $($B)+)?,)+)?> [+] bordered_mut   (bordered  : Bordered   NoBorder    ) Some(bordered);
        } {
            <$($($G $(: $($B)+)?,)+)?> [&] and_mut   (style: <Styler: Styler>) &style;
            <$($($G $(: $($B)+)?,)+)?> [|] or_mut    (style: <Styler: Styler>) &style;
            <$($($G $(: $($B)+)?,)+)?> [^] xor_mut   (style: <Styler: Styler>) &style;
            <$($($G $(: $($B)+)?,)+)?> [%] dedup_mut (style: <Styler: Styler>) &style;
            <$($($G $(: $($B)+)?,)+)?> [!] reset_mut;
        });
    };

    (impl trait $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        $Ops:ident($ops:ident) $OpsAssign:ident($ops_assign:ident)
        ($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr
    ) => {
        impl<$($G $(: $($B+)+,)?)* $($GRhs: $crate::$BRhs)?>
            ::std::ops::$Ops<$($crate::$Rhs)? $($GRhs)?> for $Type {
            type Output = Self;

            fn $ops(mut self, $rhs: $($crate::$Rhs)? $($GRhs)?) -> Self {
                <Self as $crate::Styler>::$fn(&mut self, $body);
                self
            }
        }

        impl<$($G $(: $($B+)+,)?)* $($GRhs: $crate::$BRhs)?>
            ::std::ops::$OpsAssign<$($crate::$Rhs)? $($GRhs)?> for $Type {
            fn $ops_assign(&mut self, $rhs: $($crate::$Rhs)? $($GRhs)?) {
                <Self as $crate::Styler>::$fn(self, $body);
            }
        }
    };
    (impl trait unary $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        $Ops:ident($ops:ident)
    ) => {
        impl<$($G $(: $($B+)+,)?)*> ::std::ops::$Ops for $Type {
            type Output = Self;

            fn $ops(mut self) -> Self {
                <Self as $crate::Styler>::$fn(&mut self);
                self
            }
        }
    };

    (impl ops $Type:ty { $(
        <$($G1:ident $(: $($B1:path)+)?,)*> [$op1:tt] $fn1:ident
        ($rhs1:tt: $Rhs:ident $NoRhs:ident) $body1:expr;
    )* } { $(
        <$($G2:ident $(: $($B2:path)+)?,)*> [$op2:tt] $fn2:ident
        $(($rhs2:tt: <$GRhs:ident: $BRhs:ident>) $body2:expr)?;
    )* }) => {
        $(impl_styler!(impl op [$op1] $Type: <$($G1 $(: $($B1)+ )?,)*> $fn1 ($rhs1:   $Rhs) $body1);)*
        $(impl_styler!(impl op [$op1] $Type: <$($G1 $(: $($B1)+ )?,)*> $fn1 (_    : $NoRhs) None);)*
        $(impl_styler!(impl op [$op2] $Type: <$($G2 $(: $($B2)+ )?,)*> $fn2 $(($rhs2: <$GRhs: $BRhs>) $body2)?);)*
    };

    (impl op [+] $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr
    ) => {
        impl_styler!(impl trait $Type: <$($G $(: $($B)+)?,)*> $fn
            Add(add) AddAssign(add_assign)
            ($rhs: $($Rhs)? $(<$GRhs: $BRhs>)?) $body);
    };
    (impl op [*] $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr
    ) => {
        impl_styler!(impl trait $Type: <$($G $(: $($B)+)?,)*> $fn
            Mul(mul) MulAssign(mul_assign)
            ($rhs: $($Rhs)? $(<$GRhs: $BRhs>)?) $body);
    };
    (impl op [/] $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr
    ) => {
        impl_styler!(impl trait $Type: <$($G $(: $($B)+)?,)*> $fn
            Div(div) DivAssign(div_assign)
            ($rhs: $($Rhs)? $(<$GRhs: $BRhs>)?) $body);
    };
    (impl op [&] $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr
    ) => {
        impl_styler!(impl trait $Type: <$($G $(: $($B)+)?,)*> $fn
            BitAnd(bitand) BitAndAssign(bitand_assign)
            ($rhs: $($Rhs)? $(<$GRhs: $BRhs>)?) $body);
    };
    (impl op [|] $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr
    ) => {
        impl_styler!(impl trait $Type: <$($G $(: $($B)+)?,)*> $fn
            BitOr(bitor) BitOrAssign(bitor_assign)
            ($rhs: $($Rhs)? $(<$GRhs: $BRhs>)?) $body);
    };
    (impl op [^] $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr
    ) => {
        impl_styler!(impl trait $Type: <$($G $(: $($B)+)?,)*> $fn
            BitXor(bitxor) BitXorAssign(bitxor_assign)
            ($rhs: $($Rhs)? $(<$GRhs: $BRhs>)?) $body);
    };
    (impl op [%] $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr
    ) => {
        impl_styler!(impl trait $Type: <$($G $(: $($B)+)?,)*> $fn
            Rem(rem) RemAssign(rem_assign)
            ($rhs: $($Rhs)? $(<$GRhs: $BRhs>)?) $body);
    };
    (impl op [!] $Type:ty: <$($G:ident $(: $($B:path)+)?,)*> $fn:ident) => {
        impl_styler!(impl trait unary $Type: <$($G $(: $($B)+)?,)*> $fn Not(not));
    };
}
