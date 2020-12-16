mod layer;
mod rows;
mod rows_mut;

pub use layer::*;
pub use rows::*;
pub use rows_mut::*;

use crate::*;

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
    pub fn merge_mut<'b>(
        &mut self,
        self_col: u16,
        self_row: u16,
        self_width: u16,
        self_height: u16,
        other: &'b impl Layer<'b>,
        other_col: u16,
        other_row: u16,
        other_width: u16,
        other_height: u16,
        merge: impl Fn(Cell, Cell) -> Cell,
    ) {
        let (width, height) = (self_width, self_height).min((other_width, other_height));
        let self_cells = self.cropped_cells_mut(self_col, self_row, width, height);
        let other_cells = other.cropped_cells(other_col, other_row, width, height);

        for (self_cell, other_cell) in self_cells.zip(other_cells) {
            *self_cell = merge(*self_cell, other_cell);
        }
    }

    fn fill_mut(&mut self, cell: Cell) {
        for self_cell in self.cells_mut() {
            *self_cell = cell
        }
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

    unsafe fn get_cell_unchecked(&self, row: u16, col: u16) -> Cell {
        let (row, col) = (row as usize, col as usize);
        let (width, height) = AsCoord::as_usize(&self.size);

        debug_assert!(row < height);
        debug_assert!(col < width);

        let rows = row * width;
        let i = rows + col;

        debug_assert!(i < self.cells.len());

        *self.cells.get_unchecked(i)
    }

    unsafe fn get_cell_unchecked_mut(&mut self, row: u16, col: u16) -> &mut Cell {
        let (row, col) = (row as usize, col as usize);
        let (width, height) = AsCoord::as_usize(&self.size);

        debug_assert!(row < height);
        debug_assert!(col < width);

        let rows = row * width;
        let i = rows + col;

        debug_assert!(i < self.cells.len());

        self.cells.get_unchecked_mut(i)
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
