macro_rules! impl_styler_ops {
    ($Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?) => {
        impl_styler_ops!(impl $Type
            <$($($G $(: $B)?,)+)?> [*] foreground_mut (foreground: Color) Some($crate::Foreground(foreground));
            <$($($G $(: $B)?,)+)?> [/] background_mut (background: Color) Some($crate::Background(background));
            <$($($G $(: $B)?,)+)?> [+] foreground_mut (foreground: Foreground) Some(foreground);
            <$($($G $(: $B)?,)+)?> [+] background_mut (background: Background) Some(background);
            <$($($G $(: $B)?,)+)?> [+] weighted_mut   (weighted  : Weighted  ) Some(weighted);
            <$($($G $(: $B)?,)+)?> [+] slanted_mut    (slanted   : Slanted   ) Some(slanted);
            <$($($G $(: $B)?,)+)?> [+] blinking_mut   (blinking  : Blinking  ) Some(blinking);
            <$($($G $(: $B)?,)+)?> [+] inverted_mut   (inverted  : Inverted  ) Some(inverted);
            <$($($G $(: $B)?,)+)?> [+] striked_mut    (striked   : Striked   ) Some(striked);
            <$($($G $(: $B)?,)+)?> [+] underlined_mut (underlined: Underlined) Some(underlined);
            <$($($G $(: $B)?,)+)?> [+] overlined_mut  (overlined : Overlined ) Some(overlined);
            <$($($G $(: $B)?,)+)?> [+] bordered_mut   (bordered  : Bordered  ) Some(bordered);

            <$($($G $(: $B)?,)+)?> [*] foreground_mut (_: NoColor) None;
            <$($($G $(: $B)?,)+)?> [/] background_mut (_: NoColor) None;
            <$($($G $(: $B)?,)+)?> [+] foreground_mut (_: NoForeground) None;
            <$($($G $(: $B)?,)+)?> [+] background_mut (_: NoBackground) None;
            <$($($G $(: $B)?,)+)?> [+] weighted_mut   (_: NoWeight    ) None;
            <$($($G $(: $B)?,)+)?> [+] slanted_mut    (_: NoSlant     ) None;
            <$($($G $(: $B)?,)+)?> [+] blinking_mut   (_: NoBlink     ) None;
            <$($($G $(: $B)?,)+)?> [+] inverted_mut   (_: NoInvert    ) None;
            <$($($G $(: $B)?,)+)?> [+] striked_mut    (_: NoStrike    ) None;
            <$($($G $(: $B)?,)+)?> [+] underlined_mut (_: NoUnderline ) None;
            <$($($G $(: $B)?,)+)?> [+] overlined_mut  (_: NoOverline  ) None;
            <$($($G $(: $B)?,)+)?> [+] bordered_mut   (_: NoBorder    ) None;

            <$($($G $(: $B)?,)+)?> [&] or_mut    (style: <Rhs: Styler>) &style;
            <$($($G $(: $B)?,)+)?> [|] or_mut    (style: <Rhs: Styler>) &style;
            <$($($G $(: $B)?,)+)?> [^] xor_mut   (style: <Rhs: Styler>) &style;
            <$($($G $(: $B)?,)+)?> [%] dedup_mut (style: <Rhs: Styler>) &style;
            <$($($G $(: $B)?,)+)?> [!] reset_mut;
        );
    };
    (impl $Type:ident
        $(
            <$($G:ident $(: $B:tt)?,)*> [$op:tt] $fn:ident
            $(($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr)?;
        )*
    ) => {
        $(impl_styler_ops!(impl op [$op] $Type <$($G $(: $B)?,)*> $fn
            $(($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body)?
        );)*
    };
    (impl trait $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        $Ops:ident($ops:ident) $OpsAssign:ident($ops_assign:ident)
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl<$($GenericRhs: $crate::$RhsBound,)? $($G $(: $B,)?)*>
            ::std::ops::$Ops<$($crate::$Rhs)? $($GenericRhs)?> for $Type<$($G,)*> {
            type Output = Self;

            fn $ops(mut self, $rhs: $($crate::$Rhs)? $($GenericRhs)?) -> Self {
                <Self as $crate::Styler>::$fn(&mut self, $body);
                self
            }
        }

        impl<$($GenericRhs: $crate::$RhsBound,)? $($G $(: $B,)?)*>
            ::std::ops::$OpsAssign<$($crate::$Rhs)? $($GenericRhs)?> for $Type<$($G,)*> {
            fn $ops_assign(&mut self, $rhs: $($crate::$Rhs)? $($GenericRhs)?) {
                <Self as $crate::Styler>::$fn(self, $body);
            }
        }
    };
    (impl trait unary $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        $Ops:ident($ops:ident)
    ) => {
        impl<$($G $(: $B,)?)*> ::std::ops::$Ops for $Type<$($G,)*> {
            type Output = Self;

            fn $ops(mut self) -> Self {
                <Self as $crate::Styler>::$fn(&mut self);
                self
            }
        }
    };
    (impl op [+] $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type <$($G $(: $B)?,)*> $fn
            Add(add) AddAssign(add_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [*] $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type <$($G $(: $B)?,)*> $fn
            Mul(mul) MulAssign(mul_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [/] $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type <$($G $(: $B)?,)*> $fn
            Div(div) DivAssign(div_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [&] $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type <$($G $(: $B)?,)*> $fn
            BitAnd(bitand) BitAndAssign(bitand_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [|] $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type <$($G $(: $B)?,)*> $fn
            BitOr(bitor) BitOrAssign(bitor_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [^] $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type <$($G $(: $B)?,)*> $fn
            BitXor(bitxor) BitXorAssign(bitxor_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [%] $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type <$($G $(: $B)?,)*> $fn
            Rem(rem) RemAssign(rem_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [!] $Type:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident) => {
        impl_styler_ops!(impl trait unary $Type <$($G $(: $B)?,)*> $fn Not(not));
    };
}
