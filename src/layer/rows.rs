use crate::*;

pub struct Rows<'a, T: ?Sized> {
    pub layer: &'a T,
    col:       u16,
    width:     u16,
    row:       u16,
    // it:        Option<<T as LayerIter<'a>>::Row>,
    end:       u16,
}

impl<'a, T: LayerIter<'a> + ?Sized> Rows<'a, T> {
    pub fn new(layer: &'a T, col: u16, row: u16, width: u16, height: u16) -> Self {
        let (layer_width, layer_height) = layer.size();

        if layer_width == 0
            || layer_height == 0
            || width == 0
            || height == 0
            || col >= layer_width
            || row >= layer_height
        {
            return Self {
                layer,
                col: 0,
                width: 0,
                row: 0,
                // it: None,
                end: 0,
            };
        }

        let (width, height) = (width.min(layer_width - col), height.min(layer_height - row));

        debug_assert!(col < layer_width);
        debug_assert!(row < layer_height);
        debug_assert!(col as usize + width as usize <= layer_width as usize);
        debug_assert!(row as usize + height as usize <= layer_height as usize);
        // let it = Some(unsafe { layer.row_unchecked(row, col, width) });

        Self {
            layer,
            col,
            width,
            row,
            end: row + height,
        }
    }
}

impl<'a, T: LayerIter<'a> + ?Sized> Iterator for Rows<'a, T> {
    type Item = <T as LayerIter<'a>>::Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.end {
            self.row += 1;

            // SAFETY:
            // Self::new guaranties we are in layer's bounds
            Some(unsafe { self.layer.row_unchecked(self.row - 1, self.col, self.width) })
        } else {
            None
        }
    }
}

/*
impl<'a, T: LayerIter<'a> + ?Sized> Iterator for Rows<&'a T> {
    type Item = <T as LayerIter<'a>>::Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.end {
            self.row += 1;

            // SAFETY:
            // Self::new guaranties we are in layer's bounds
            Some(unsafe { self.layer.row_unchecked(self.row - 1, self.col, self.width) })
        } else {
            None
        }
    }
}

impl<'a, T: LayerIterMut<'a> + ?Sized> Iterator for Rows<&'a mut T> {
    type Item = <T as LayerIterMut<'a>>::RowMut;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.end {
            self.row += 1;

            // SAFETY:
            // Self::new guaranties we are in layer's bounds
            Some(unsafe {
                self.layer
                    .row_unchecked_mut(self.row - 1, self.col, self.width)
            })
        } else {
            None
        }
    }
}
*/

// pub struct RowsMut<'a, T: LayerIterMut<'a> + ?Sized> {
// pub layer: &'a mut T,
// col:       u16,
// width:     u16,
// row:       u16,
// it:        Option<<T as LayerIterMut<'a>>::RowMut>,
// end:       u16,
// }
//
// impl<'a, T: LayerIterMut<'a> + ?Sized> RowsMut<'a, T> {
// pub fn new(layer: &'a mut T, col: u16, row: u16, width: u16, height: u16) ->
// Self { let (layer_width, layer_height) = layer.size();
//
// if layer_width == 0
// || layer_height == 0
// || width == 0
// || height == 0
// || col >= layer_width
// || row >= layer_height
// {
// return Self {
// layer,
// col: 0,
// width: 0,
// row: 0,
// it: None,
// end: 0,
// };
// }
//
// let (width, height) = (width.min(layer_width - col), height.min(layer_height
// - row));
//
// debug_assert!(col < layer_width);
// debug_assert!(row < layer_height);
// debug_assert!(col as usize + width as usize <= layer_width as usize);
// debug_assert!(row as usize + height as usize <= layer_height as usize);
// let it = Some(unsafe { layer.row_unchecked_mut(row, col, width) });
// let it = None;
//
// Self {
// layer,
// col,
// width,
// row,
// it,
// end: row + height,
// }
// }
// }
//
// impl<'a, T: LayerIterMut<'a> + ?Sized> Iterator for RowsMut<'a, T> {
// type Item = <T as LayerIterMut<'a>>::RowMut;
//
// fn next(&mut self) -> Option<Self::Item> {
// if self.row < self.end {
// self.row += 1;
// self.it.take()
// } else {
// None
// }
// }
// }
