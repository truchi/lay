macro_rules! impl_layer_mut_ops {
    ($Type:ident) => {
        impl_layer_mut_ops!(impl $Type Shl(shl) ShlAssign(shl_assign) below);
        impl_layer_mut_ops!(impl $Type Shr(shr) ShrAssign(shr_assign) above);
    };
    (impl $Type:ident $Op:ident($op:ident) $OpAssign:ident($op_assign:ident) $fn:ident) => {
        impl<T: $crate::Layer> ::std::ops::$Op<(T, u16, u16)> for $Type {
            type Output = Self;

            fn $op(mut self, (layer, x, y): (T, u16, u16)) -> Self {
                <$Type as $crate::LayerMut>::$fn(&mut self, &layer, x, y);
                self
            }
        }

        impl<T: $crate::Layer> ::std::ops::$OpAssign<(T, u16, u16)> for $Type {
            fn $op_assign(&mut self, (layer, x, y): (T, u16, u16)) {
                <$Type as $crate::LayerMut>::$fn(self, &layer, x, y);
            }
        }
    };
}
