use crate::*;
use std::{
    iter::{Copied, Flatten},
    slice::{Iter, IterMut},
};

/// A [`Canvas`](crate::Canvas).
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Canvas {
    size:  Coord,
    cells: Vec<Cell>,
}

pub type CanvasRowIter<'a> = Copied<Iter<'a, Cell>>;
pub type CanvasRowIterMut<'a> = IterMut<'a, Cell>;
pub type CanvasCellsIter<'a> = Flatten<CanvasRowsIter<'a>>;
pub type CanvasCellsIterMut<'a> = Flatten<CanvasRowsIterMut<'a>>;

/// ### Constructors
impl Canvas {
    /// Returns a new [`Canvas`](crate::Canvas) of `cell`s.
    pub fn new(size: impl AsCoord, cell: impl Into<Cell>) -> Self {
        let size = size.as_coord();
        let (width, height) = (size.0 as usize, size.1 as usize);

        let mut cells = Vec::with_capacity(width * height);
        cells.resize(width * height, cell.into());

        Self { size, cells }
    }

    /// Returns a new [`Canvas`](crate::Canvas) of
    /// [`Cell::NONE`](crate::Cell::NONE).
    pub fn with_default(size: impl AsCoord) -> Self {
        Self::new(size, Cell::NONE)
    }

    pub fn with_cells(size: impl AsCoord, cells: Vec<Cell>) -> Option<Self> {
        let size = size.as_coord();
        let (width, height) = (size.0 as usize, size.1 as usize);

        if width * height != cells.len() {
            return None;
        }

        Some(Self { size, cells })
    }
}

/// ### Methods
impl Canvas {
    pub fn row(&self, row: u16, col: u16, len: u16) -> CanvasRowIter {
        let (width, height) = self.size;

        if width == 0 || height == 0 || row >= height || col >= width || len == 0 {
            // SAFETY:
            // Self::get_row_unchecked is safe for 0, 0, 0
            unsafe { self.row_unchecked(0, 0, 0) }
        } else {
            let len = len.min(width - col);

            // SAFETY:
            // - (col, row) < size
            // - col + len <= width
            unsafe { self.row_unchecked(row, col, len) }
        }
    }

    pub fn row_mut(&mut self, row: u16, col: u16, len: u16) -> CanvasRowIterMut {
        let (width, height) = self.size;

        if width == 0 || height == 0 || row >= height || col >= width || len == 0 {
            // SAFETY:
            // Self::get_row_unchecked is safe for 0, 0, 0
            unsafe { self.row_unchecked_mut(0, 0, 0) }
        } else {
            let len = len.min(width - col);

            // SAFETY:
            // - (col, row) < size
            // - col + len <= width
            unsafe { self.row_unchecked_mut(row, col, len) }
        }
    }

    pub fn rows(&self, col: u16, row: u16, width: u16, height: u16) -> CanvasRowsIter {
        CanvasRowsIter::new(self, col, row, width, height)
    }

    pub fn rows_mut(&mut self, col: u16, row: u16, width: u16, height: u16) -> CanvasRowsIterMut {
        CanvasRowsIterMut::new(self, col, row, width, height)
    }

    pub fn cells(&self, col: u16, row: u16, width: u16, height: u16) -> CanvasCellsIter {
        self.rows(col, row, width, height).flatten()
    }

    pub fn cells_mut(&mut self, col: u16, row: u16, width: u16, height: u16) -> CanvasCellsIterMut {
        self.rows_mut(col, row, width, height).flatten()
    }
}

/// ### Private methods
impl Canvas {
    unsafe fn row_unchecked(&self, row: u16, col: u16, len: u16) -> CanvasRowIter {
        self.get_row_unchecked(row, col, len).iter().copied()
    }

    unsafe fn row_unchecked_mut(&mut self, row: u16, col: u16, len: u16) -> CanvasRowIterMut {
        self.get_row_unchecked_mut(row, col, len).iter_mut()
    }

    /// Returns a slice of the [`Cell`](crate::Cell)s of row `row` from
    /// column `col` with length `len`.
    ///
    /// ### Safety
    ///
    /// Callers MUST ensure:
    /// - `row < height`
    /// - `col < width`
    /// - `col + len <= width`
    ///
    /// where `(width, height)` being the `size` of the
    /// [`Canvas`](crate::Canvas).
    unsafe fn get_row_unchecked(&self, row: u16, col: u16, len: u16) -> &[Cell] {
        let (row, col, len) = (row as usize, col as usize, len as usize);
        let (width, height) = AsCoord::as_usize(&self.size);

        debug_assert!(row < height);
        debug_assert!(col < width);
        debug_assert!(col + len <= width);

        let rows = row * width;
        let start = rows + col;
        let end = start + len;

        let size = self.cells.len();
        debug_assert!(start < size);
        debug_assert!(end <= size);

        self.cells.get_unchecked(start..end)
    }

    unsafe fn get_row_unchecked_mut(&mut self, row: u16, col: u16, len: u16) -> &mut [Cell] {
        let (row, col, len) = (row as usize, col as usize, len as usize);
        let (width, height) = AsCoord::as_usize(&self.size);

        debug_assert!(row < height);
        debug_assert!(col < width);
        debug_assert!(col + len <= width);

        let rows = row * width;
        let start = rows + col;
        let end = start + len;

        let size = self.cells.len();
        debug_assert!(start < size);
        debug_assert!(end <= size);

        self.cells.get_unchecked_mut(start..end)
    }
}

impl LayerSize for Canvas {
    fn size(&self) -> Coord {
        self.size
    }
}

pub struct CanvasRowsIter<'a> {
    pub canvas: &'a Canvas,
    col:        u16,
    width:      u16,
    row:        u16,
    end:        u16,
}

impl<'a> CanvasRowsIter<'a> {
    fn new(canvas: &'a Canvas, col: u16, row: u16, width: u16, height: u16) -> Self {
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

impl<'a> Iterator for CanvasRowsIter<'a> {
    type Item = CanvasRowIter<'a>;

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

pub struct CanvasRowsIterMut<'a> {
    cells:   &'a mut [Cell],
    width:   u16,
    col:     u16,
    len:     u16,
    start:   u16,
    current: u16,
    end:     u16,
}

impl<'a> CanvasRowsIterMut<'a> {
    fn new(canvas: &'a mut Canvas, col: u16, row: u16, width: u16, height: u16) -> Self {
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

impl<'a> Iterator for CanvasRowsIterMut<'a> {
    type Item = CanvasRowIterMut<'a>;

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

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Canvas`](crate::Canvas) of `cell`s.
impl<T: AsCoord, U: Into<Cell>> From<(T, U)> for Canvas {
    /// Returns a new [`Canvas`](crate::Canvas) of `cell`s.
    fn from((size, cell): (T, U)) -> Self {
        Self::new(size, cell)
    }
}
