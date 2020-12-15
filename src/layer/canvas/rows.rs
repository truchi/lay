use crate::*;

pub struct CanvasRows<'a> {
    pub canvas: &'a Canvas,
    col:        u16,
    width:      u16,
    row:        u16,
    end:        u16,
}

impl<'a> CanvasRows<'a> {
    pub fn new(canvas: &'a Canvas, col: u16, row: u16, width: u16, height: u16) -> Self {
        let (canvas_width, canvas_height) = canvas.size();

        if canvas_width == 0
            || canvas_height == 0
            || width == 0
            || height == 0
            || col >= canvas_width
            || row >= canvas_height
        {
            return Self {
                canvas,
                col: 0,
                width: 0,
                row: 0,
                end: 0,
            };
        }

        let (width, height) = (
            width.min(canvas_width - col),
            height.min(canvas_height - row),
        );

        debug_assert!(col < canvas_width);
        debug_assert!(row < canvas_height);
        debug_assert!(col as usize + width as usize <= canvas_width as usize);
        debug_assert!(row as usize + height as usize <= canvas_height as usize);

        return Self {
            canvas,
            col,
            width,
            row,
            end: row + height,
        };
    }
}

impl<'a> Iterator for CanvasRows<'a> {
    type Item = CanvasRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.end {
            // SAFETY:
            // Constructor makes sure we are in bounds
            let item = unsafe { self.canvas.row_unchecked(self.row, self.col, self.width) };
            self.row += 1;
            Some(item)
        } else {
            None
        }
    }
}
