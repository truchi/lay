use crate::*;

pub struct Rows<'a, T: ?Sized> {
    pub layer: &'a T,
    col:       u16,
    width:     u16,
    row:       u16,
    end:       u16,
}

impl<'a, T: ?Sized> Rows<'a, T> {
    pub fn new_empty(layer: &'a T) -> Self {
        Self {
            layer,
            col: 0,
            width: 0,
            row: 0,
            end: 0,
        }
    }
}

impl<'a, T: LayerIndex + ?Sized> Rows<'a, T> {
    pub fn new(layer: &'a T, col: u16, row: u16, width: u16, height: u16) -> Self {
        let (canvas_width, canvas_height) = layer.size();

        if canvas_width == 0
            || canvas_height == 0
            || width == 0
            || height == 0
            || col >= canvas_width
            || row >= canvas_height
        {
            return Self::new_empty(layer);
        }

        let (width, height) = (
            width.min(canvas_width - col),
            height.min(canvas_height - row),
        );

        debug_assert!(col < canvas_width);
        debug_assert!(row < canvas_height);
        debug_assert!(col as usize + width as usize <= canvas_width as usize);
        debug_assert!(row as usize + height as usize <= canvas_height as usize);

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
