macro_rules! impl_layer_mut_ops {
    ($Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?) => {
        impl_layer_mut_ops!(impl $Type $(<$($G $(: $B)?,)+>)? Shl(shl) ShlAssign(shl_assign) below);
        impl_layer_mut_ops!(impl $Type $(<$($G $(: $B)?,)+>)? Shr(shr) ShrAssign(shr_assign) above);
    };
    (impl $Type:ident $(<$($G:ident $(: $B:tt)?,)+>)?
        $Op:ident($op:ident) $OpAssign:ident($op_assign:ident) $fn:ident
    ) => {
        impl<__Rhs: $crate::Layer $(, $($G $(: $B)?)+)?> ::std::ops::$Op<(__Rhs, u16, u16)> for $Type $(<$($G,)+>)? {
            type Output = Self;

            fn $op(mut self, (layer, x, y): (__Rhs, u16, u16)) -> Self {
                <$Type $(<$($G,)+>)? as $crate::LayerMut>::$fn(&mut self, &layer, x, y);
                self
            }
        }

        impl<__Rhs: $crate::Layer $(, $($G $(: $B)?)+)?> ::std::ops::$OpAssign<(__Rhs, u16, u16)> for $Type $(<$($G,)+>)? {
            fn $op_assign(&mut self, (layer, x, y): (__Rhs, u16, u16)) {
                <$Type $(<$($G,)+>)? as $crate::LayerMut>::$fn(self, &layer, x, y);
            }
        }
    };
}
