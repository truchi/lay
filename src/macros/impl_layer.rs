macro_rules! impl_layer {
    (
        $Type:ty [$self:ident, $x:ident, $y:ident] {
            $(Layer    $(<$($G1:ident $(: $($B1:path)+)?,)+>)? $width:block $height:block $layer:block)?
            $(Index    $(<$($G2:ident $(: $($B2:path)+)?,)+>)? $index:block)?
            $(LayerMut $(<$($G3:ident $(: $($B3:path)+)?,)+>)? $layer_mut:block)?
            $(IndexMut $(<$($G4:ident $(: $($B4:path)+)?,)+>)? $index_mut:block)?
        }
    ) => {
        $(impl $(<$($G1 $(: $($B1 +)+ )?,)+>)? $crate::Layer for $Type {
            #[allow(unused)]
            fn width(&self) -> u16 {
                let $self = self;
                $width
            }

            #[allow(unused)]
            fn height(&self) -> u16 {
                let $self = self;
                $height
            }

            #[allow(unused)]
            fn get_unchecked(&self, $x: u16, $y: u16) -> $crate::Cell {
                let $self = self;
                $layer
            }
        })?

        $(impl $(<$($G2 $(: $($B2 +)+)?,)+>)? ::std::ops::Index<(u16, u16)> for $Type {
            type Output = $crate::Cell;

            #[allow(unused)]
            fn index(&self, ($x, $y): (u16, u16)) -> &$crate::Cell {
                let $self = self;
                $index
            }
        })?

        $(impl $(<$($G3 $(: $($B3 +)+ )?,)+>)? $crate::LayerMut for $Type {
            #[allow(unused)]
            fn get_mut_unchecked(&mut self, $x: u16, $y: u16) -> &mut $crate::Cell {
                let $self = self;
                $layer_mut
            }
        })?

        $(impl $(<$($G4 $(: $($B4 +)+ )?,)+>)? ::std::ops::IndexMut<(u16, u16)> for $Type {
            #[allow(unused)]
            fn index_mut(&mut self, ($x, $y): (u16, u16)) -> &mut $crate::Cell {
                let $self = self;
                $index_mut
            }
        })?
    };
}
