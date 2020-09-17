macro_rules! impl_layer_mut_ops {
    ($Type:ident) => {
        impl<T: $crate::Layer> ::std::ops::Shl<(&T, u16, u16)> for $Type {
            type Output = Self;

            fn shl(mut self, (below, x, y): (&T, u16, u16)) -> Self {
                <$Type as $crate::LayerMut>::below(&mut self, below, x, y);
                self
            }
        }

        impl<T: $crate::Layer> ::std::ops::ShlAssign<(&T, u16, u16)> for $Type {
            fn shl_assign(&mut self, (below, x, y): (&T, u16, u16)) {
                <$Type as $crate::LayerMut>::below(self, below, x, y);
            }
        }

        impl<T: $crate::Layer> ::std::ops::Shr<(&T, u16, u16)> for $Type {
            type Output = Self;

            fn shr(mut self, (above, x, y): (&T, u16, u16)) -> Self {
                <$Type as $crate::LayerMut>::above(&mut self, above, x, y);
                self
            }
        }

        impl<T: $crate::Layer> ::std::ops::ShrAssign<(&T, u16, u16)> for $Type {
            fn shr_assign(&mut self, (above, x, y): (&T, u16, u16)) {
                <$Type as $crate::LayerMut>::above(self, above, x, y);
            }
        }
    };
}
