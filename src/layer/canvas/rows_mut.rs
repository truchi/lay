use crate::*;

pub struct CanvasRowsMut<'a> {
    cells:   &'a mut [Cell],
    width:   u16,
    col:     u16,
    len:     u16,
    start:   u16,
    current: u16,
    end:     u16,
}

impl<'a> CanvasRowsMut<'a> {
    pub fn new(canvas: &'a mut Canvas, col: u16, row: u16, width: u16, height: u16) -> Self {
        let (canvas_width, canvas_height) = canvas.size();

        if canvas_width == 0
            || canvas_height == 0
            || width == 0
            || height == 0
            || col >= canvas_width
            || row >= canvas_height
        {
            return Self {
                cells:   &mut [],
                width:   canvas_width,
                col:     0,
                len:     0,
                start:   0,
                current: 0,
                end:     0,
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
            cells: &mut canvas.cells,
            width: canvas_width,
            col,
            len: width,
            start: row,
            current: row,
            end: row + height,
        };
    }
}

impl<'a> Iterator for CanvasRowsMut<'a> {
    type Item = CanvasRowMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let cells = std::mem::take(&mut self.cells);

            // Advance the slice to the start of the row to be returned
            let offset = if self.current == self.start {
                // At first, the slice points to (0, 0)
                // Move to the requested row, then to the column
                self.start as usize * self.width as usize + self.col as usize
            } else {
                // On subsequent calls, the slice points to (col + len, some_row)
                // Move to the start of the next row
                self.width as usize - self.len as usize
            };

            let (_, cells) = cells.split_at_mut(offset);
            let (row, cells) = cells.split_at_mut(self.len as usize);
            self.cells = cells;
            self.current += 1;

            Some(row.iter_mut())
        } else {
            None
        }
    }
}
