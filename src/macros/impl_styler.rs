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

        fn $set_mut(&mut self, arg: ::std::option::Option<$crate::$Attr>) {
            let $self = self;
            $field = arg;
        }
    };
    (impl method $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $field.$get()
        }

        fn $set_mut(&mut self, arg: ::std::option::Option<$crate::$Attr>) {
            let $self = self;
            $field.$set_mut(arg);
        }
    };

    // ===
    // Ops
    // ===
    (impl ops $Type:ty: $(<$($G:ident $(: $($B:path)+)?,)+>)?) => {
        impl_styler!(impl $Type:
            <$($($G $(: $($B)+)?,)+)?> [*] foreground_mut (foreground: Color) Some($crate::Foreground(foreground));
            <$($($G $(: $($B)+)?,)+)?> [/] background_mut (background: Color) Some($crate::Background(background));
            <$($($G $(: $($B)+)?,)+)?> [+] foreground_mut (foreground: Foreground) Some(foreground);
            <$($($G $(: $($B)+)?,)+)?> [+] background_mut (background: Background) Some(background);
            <$($($G $(: $($B)+)?,)+)?> [+] weighted_mut   (weighted  : Weighted  ) Some(weighted);
            <$($($G $(: $($B)+)?,)+)?> [+] slanted_mut    (slanted   : Slanted   ) Some(slanted);
            <$($($G $(: $($B)+)?,)+)?> [+] blinking_mut   (blinking  : Blinking  ) Some(blinking);
            <$($($G $(: $($B)+)?,)+)?> [+] inverted_mut   (inverted  : Inverted  ) Some(inverted);
            <$($($G $(: $($B)+)?,)+)?> [+] striked_mut    (striked   : Striked   ) Some(striked);
            <$($($G $(: $($B)+)?,)+)?> [+] underlined_mut (underlined: Underlined) Some(underlined);
            <$($($G $(: $($B)+)?,)+)?> [+] overlined_mut  (overlined : Overlined ) Some(overlined);
            <$($($G $(: $($B)+)?,)+)?> [+] bordered_mut   (bordered  : Bordered  ) Some(bordered);

            <$($($G $(: $($B)+)?,)+)?> [*] foreground_mut (_: NoColor) None;
            <$($($G $(: $($B)+)?,)+)?> [/] background_mut (_: NoColor) None;
            <$($($G $(: $($B)+)?,)+)?> [+] foreground_mut (_: NoForeground) None;
            <$($($G $(: $($B)+)?,)+)?> [+] background_mut (_: NoBackground) None;
            <$($($G $(: $($B)+)?,)+)?> [+] weighted_mut   (_: NoWeight    ) None;
            <$($($G $(: $($B)+)?,)+)?> [+] slanted_mut    (_: NoSlant     ) None;
            <$($($G $(: $($B)+)?,)+)?> [+] blinking_mut   (_: NoBlink     ) None;
            <$($($G $(: $($B)+)?,)+)?> [+] inverted_mut   (_: NoInvert    ) None;
            <$($($G $(: $($B)+)?,)+)?> [+] striked_mut    (_: NoStrike    ) None;
            <$($($G $(: $($B)+)?,)+)?> [+] underlined_mut (_: NoUnderline ) None;
            <$($($G $(: $($B)+)?,)+)?> [+] overlined_mut  (_: NoOverline  ) None;
            <$($($G $(: $($B)+)?,)+)?> [+] bordered_mut   (_: NoBorder    ) None;

            <$($($G $(: $($B)+)?,)+)?> [&] or_mut    (style: <Styler: Styler>) &style;
            <$($($G $(: $($B)+)?,)+)?> [|] or_mut    (style: <Styler: Styler>) &style;
            <$($($G $(: $($B)+)?,)+)?> [^] xor_mut   (style: <Styler: Styler>) &style;
            <$($($G $(: $($B)+)?,)+)?> [%] dedup_mut (style: <Styler: Styler>) &style;
            <$($($G $(: $($B)+)?,)+)?> [!] reset_mut;
        );
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

    (impl $Type:ty: $(
        <$($G:ident $(: $($B:path)+)?,)*> [$op:tt] $fn:ident
        $(($rhs:tt: $($Rhs:ident)? $(<$GRhs:ident: $BRhs:ident>)?) $body:expr)?;
    )*) => {
        $(impl_styler!(impl op [$op] $Type: <$($G $(: $($B)+ )?,)*> $fn
            $(($rhs: $($Rhs)? $(<$GRhs: $BRhs>)?) $body)?
        );)*
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
