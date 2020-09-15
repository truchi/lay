macro_rules! impl_styler_ops {
    ($Type:ident $(< $($G:ident $(: $B:tt)?,)+ >)?) => {
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Mul(mul) MulAssign(mul_assign) Color(foreground_mut) NoColor rhs { $crate::Foreground(rhs) });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Div(div) DivAssign(div_assign) Color(background_mut) NoColor rhs { $crate::Background(rhs) });

        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Foreground(foreground_mut) NoForeground rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Background(background_mut) NoBackground rhs { rhs });

        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Weighted(weighted_mut) NoWeight rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Slanted(slanted_mut) NoSlant rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Blinking(blinking_mut) NoBlink rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Inverted(inverted_mut) NoInvert rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Striked(striked_mut) NoStrike rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Underlined(underlined_mut) NoUnderline rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Overlined(overlined_mut) NoOverline rhs { rhs });
        impl_styler_ops!(impl $Type $(<$($G $(: $B)?,)+>)?
            Add(add) AddAssign(add_assign) Bordered(bordered_mut) NoBorder rhs { rhs });
    };
    (
        impl
            $Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?
            $Ops:ident($ops:ident) $OpsAssign:ident($ops_assign:ident)
            $Rhs:ident($set_mut:ident) $NoRhs:ident $rhs:ident $body:expr
    ) => {
        impl $(<$($G $(: $B)?,)+>)? ::std::ops::$Ops<$crate::$Rhs> for $Type $(<$($G,)+>)? {
            type Output = Self;

            fn $ops(mut self, $rhs: $crate::$Rhs) -> Self {
                <Self as $crate::Styler>::$set_mut(&mut self, Some($body));
                self
            }
        }

        impl $(<$($G $(: $B)?,)+>)? ::std::ops::$OpsAssign<$crate::$Rhs> for $Type $(<$($G,)+>)? {
            fn $ops_assign(&mut self, $rhs: $crate::$Rhs) {
                <Self as $crate::Styler>::$set_mut(self, Some($body));
            }
        }

        impl $(<$($G $(: $B)?,)+>)? ::std::ops::$Ops<$crate::$NoRhs> for $Type $(<$($G,)+>)? {
            type Output = Self;

            fn $ops(mut self, _: $crate::$NoRhs) -> Self {
                <Self as $crate::Styler>::$set_mut(&mut self, None);
                self
            }
        }

        impl $(<$($G $(: $B)?,)+>)? ::std::ops::$OpsAssign<$crate::$NoRhs> for $Type $(<$($G,)+>)? {
            fn $ops_assign(&mut self, _: $crate::$NoRhs) {
                <Self as $crate::Styler>::$set_mut(self, None);
            }
        }
    };
}
