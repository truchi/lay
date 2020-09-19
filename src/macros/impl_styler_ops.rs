macro_rules! impl_styler_ops {
    ($Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?) => {
        impl_styler_ops!(impl $Type as Styler
            <$($($G $(: $B)?,)+)?> [*] foreground_mut (foreground: Color) $crate::Foreground(foreground);
            <$($($G $(: $B)?,)+)?> [/] background_mut (background: Color) $crate::Background(background);
            <$($($G $(: $B)?,)+)?> [+] foreground_mut (foreground: Foreground) foreground;
            <$($($G $(: $B)?,)+)?> [+] background_mut (background: Background) background;
            <$($($G $(: $B)?,)+)?> [+] weighted_mut   (weighted  : Weighted  ) weighted;
            <$($($G $(: $B)?,)+)?> [+] slanted_mut    (slanted   : Slanted   ) slanted;
            <$($($G $(: $B)?,)+)?> [+] blinking_mut   (blinking  : Blinking  ) blinking;
            <$($($G $(: $B)?,)+)?> [+] inverted_mut   (inverted  : Inverted  ) inverted;
            <$($($G $(: $B)?,)+)?> [+] striked_mut    (striked   : Striked   ) striked;
            <$($($G $(: $B)?,)+)?> [+] underlined_mut (underlined: Underlined) underlined;
            <$($($G $(: $B)?,)+)?> [+] overlined_mut  (overlined : Overlined ) overlined;
            <$($($G $(: $B)?,)+)?> [+] bordered_mut   (bordered  : Bordered  ) bordered;

            <$($($G $(: $B)?,)+)?> [!] reset_mut;
        );
    };
    ([Option] $Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?) => {
        impl_styler_ops!(impl $Type as OptionalStyler
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

            <$($($G $(: $B)?,)+)?> [!] reset_mut;

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

            <$($($G $(: $B)?,)+)?> [&] or_mut    (style: <Rhs: OptionalStyler>) &style;
            <$($($G $(: $B)?,)+)?> [|] or_mut    (style: <Rhs: OptionalStyler>) &style;
            <$($($G $(: $B)?,)+)?> [^] xor_mut   (style: <Rhs: OptionalStyler>) &style;
            <$($($G $(: $B)?,)+)?> [%] dedup_mut (style: <Rhs: OptionalStyler>) &style;
        );
    };
    (impl $Type:ident as $As:ident
        $(
            <$($G:ident $(: $B:tt)?,)*> [$op:tt] $fn:ident
            $(($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr)?;
        )*
    ) => {
        $(impl_styler_ops!(impl op [$op] $Type as $As <$($G $(: $B)?,)*> $fn
            $(($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body)?
        );)*
    };
    (impl trait $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        $Ops:ident($ops:ident) $OpsAssign:ident($ops_assign:ident)
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl<$($GenericRhs: $crate::$RhsBound,)? $($G $(: $B)?,)*>
            ::std::ops::$Ops<$($crate::$Rhs)? $($GenericRhs)?> for $Type<$($G,)*> {
            type Output = Self;

            fn $ops(mut self, $rhs: $($crate::$Rhs)? $($GenericRhs)?) -> Self {
                <Self as $crate::$As>::$fn(&mut self, $body);
                self
            }
        }

        impl<$($GenericRhs: $crate::$RhsBound,)? $($G $(: $B)?,)*>
            ::std::ops::$OpsAssign<$($crate::$Rhs)? $($GenericRhs)?> for $Type<$($G,)*> {
            fn $ops_assign(&mut self, $rhs: $($crate::$Rhs)? $($GenericRhs)?) {
                <Self as $crate::$As>::$fn(self, $body);
            }
        }
    };
    (impl trait unary $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        $Ops:ident($ops:ident)
    ) => {
        impl<$($G $(: $B)?,)*> ::std::ops::$Ops for $Type<$($G,)*> {
            type Output = Self;

            fn $ops(mut self) -> Self {
                <Self as $crate::$As>::$fn(&mut self);
                self
            }
        }
    };
    (impl op [+] $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type as $As <$($G $(: $B)?,)*> $fn
            Add(add) AddAssign(add_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [*] $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type as $As <$($G $(: $B)?,)*> $fn
            Mul(mul) MulAssign(mul_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [/] $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type as $As <$($G $(: $B)?,)*> $fn
            Div(div) DivAssign(div_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [&] $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type as $As <$($G $(: $B)?,)*> $fn
            BitAnd(bitand) BitAndAssign(bitand_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [|] $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type as $As <$($G $(: $B)?,)*> $fn
            BitOr(bitor) BitOrAssign(bitor_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [^] $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type as $As <$($G $(: $B)?,)*> $fn
            BitXor(bitxor) BitXorAssign(bitxor_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [%] $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident
        ($rhs:tt: $($Rhs:ident)? $(<$GenericRhs:ident: $RhsBound:ident>)?) $body:expr
    ) => {
        impl_styler_ops!(impl trait $Type as $As <$($G $(: $B)?,)*> $fn
            Rem(rem) RemAssign(rem_assign)
            ($rhs: $($Rhs)? $(<$GenericRhs: $RhsBound>)?) $body);
    };
    (impl op [!] $Type:ident as $As:ident <$($G:ident $(: $B:tt)?,)*> $fn:ident) => {
        impl_styler_ops!(impl trait unary $Type as $As <$($G $(: $B)?,)*> $fn Not(not));
    };
}
