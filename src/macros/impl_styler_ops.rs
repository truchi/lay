macro_rules! impl_styler_ops {
    ($Type:ident $(< $($G:ident $(: $B:tt)?,)+ >)?) => {
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Mul(mul) MulAssign(mul_assign) Color(foreground_mut) rhs { $crate::Foreground(rhs) });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Div(div) DivAssign(div_assign) Color(background_mut) rhs { $crate::Background(rhs) });

        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Foreground(foreground_mut) rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Background(background_mut) rhs { rhs });

        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Weighted(weighted_mut) rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Slanted(slanted_mut) rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Blinking(blinking_mut) rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Inverted(inverted_mut) rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Striked(striked_mut) rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Underlined(underlined_mut) rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Overlined(overlined_mut) rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Bordered(bordered_mut) rhs { rhs });
    };
    (
        impl
            $Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?
            $Ops:ident($ops:ident) $OpsAssign:ident($ops_assign:ident)
            $Rhs:ident($set_mut:ident) $rhs:ident $body:expr
    ) => {
        impl $(<$($G $(: $B)?,)+>)? ::std::ops::$Ops<$crate::$Rhs> for $Type $(<$($G,)+>)? {
            type Output = Self;

            fn $ops(mut self, rhs: $crate::$Rhs) -> Self {
                <Self as ::std::ops::$OpsAssign<$crate::$Rhs>>::$ops_assign(&mut self, rhs);
                self
            }
        }

        impl $(<$($G $(: $B)?,)+>)? ::std::ops::$OpsAssign<$crate::$Rhs> for $Type $(<$($G,)+>)? {
            fn $ops_assign(&mut self, $rhs: $crate::$Rhs) {
                <Self as $crate::Styler>::$set_mut(self, $body);
            }
        }
    };
}
