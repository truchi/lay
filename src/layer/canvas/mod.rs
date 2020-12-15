mod layer;
mod rows;
mod rows_mut;

pub use layer::*;
pub use rows::*;
pub use rows_mut::*;

use crate::*;
use std::{
    iter::{Copied, Flatten},
    slice::{Iter, IterMut},
};

pub type CanvasRow<'a> = Copied<Iter<'a, Cell>>;
pub type CanvasRowMut<'a> = IterMut<'a, Cell>;
pub type CanvasCells<'a> = Flatten<CanvasRows<'a>>;
pub type CanvasCellsMut<'a> = Flatten<CanvasRowsMut<'a>>;

/// A [`Canvas`](crate::Canvas).
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Canvas {
    size:  Coord,
    cells: Vec<Cell>,
}

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
    pub fn size(&self) -> Coord {
        self.size
    }

    pub fn row(&self, row: u16, col: u16, len: u16) -> CanvasRow {
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

    pub fn row_mut(&mut self, row: u16, col: u16, len: u16) -> CanvasRowMut {
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

    pub fn rows(&self, col: u16, row: u16, width: u16, height: u16) -> CanvasRows {
        CanvasRows::new(self, col, row, width, height)
    }

    pub fn rows_mut(&mut self, col: u16, row: u16, width: u16, height: u16) -> CanvasRowsMut {
        CanvasRowsMut::new(self, col, row, width, height)
    }

    pub fn cells(&self, col: u16, row: u16, width: u16, height: u16) -> CanvasCells {
        self.rows(col, row, width, height).flatten()
    }

    pub fn cells_mut(&mut self, col: u16, row: u16, width: u16, height: u16) -> CanvasCellsMut {
        self.rows_mut(col, row, width, height).flatten()
    }
}

/// ### Private methods
impl Canvas {
    unsafe fn row_unchecked(&self, row: u16, col: u16, len: u16) -> CanvasRow {
        self.get_row_unchecked(row, col, len).iter().copied()
    }

    unsafe fn row_unchecked_mut(&mut self, row: u16, col: u16, len: u16) -> CanvasRowMut {
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
