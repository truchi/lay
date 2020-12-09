use crate::*;
use std::{iter::Copied, slice::Iter};

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
        let (width, height) = size;
        let (width, height): (usize, usize) = (width.into(), height.into());

        let mut cells = Vec::with_capacity(width * height);
        cells.resize(width * height, cell.into());

        Self { size, cells }
    }

    /// Returns a new [`Canvas`](crate::Canvas) of
    /// [`Cell::NONE`](crate::Cell::NONE).
    pub fn with_default(size: impl AsCoord) -> Self {
        Self::new(size, Cell::NONE)
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
}

/// `Display`s the [`Canvas`](crate::Canvas).
impl Display for Canvas {
    /// `Display`s the [`Canvas`](crate::Canvas).
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_at_cursor(f)
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

// ============ //
// Layer traits //
// ============ //

impl<'a> LayerIter<'a> for Canvas {
    type Row = Copied<Iter<'a, Cell>>;

    unsafe fn row_unchecked(&'a self, row: u16, col: u16, len: u16) -> Self::Row {
        self.get_row_unchecked(row, col, len).iter().copied()
    }
}

impl LayerIndex for Canvas {
    fn size(&self) -> Coord {
        self.size
    }

    fn get_unchecked(&self, point: impl AsCoord) -> Cell {
        let (x, y) = point.as_coord();
        let (width, _) = self.size;

        match self.cells.get(x as usize + y as usize * width as usize) {
            Some(cell) => *cell,
            _ => Cell::NONE,
        }
    }
}

impl LayerIndexMut for Canvas {
    fn get_unchecked_mut(&mut self, point: impl AsCoord) -> &mut Cell {
        let (x, y) = point.as_coord();
        let (width, _) = self.size;

        self.cells
            .get_mut(x as usize + y as usize * width as usize)
            .expect("Out of bounds access")
    }
}

impl Layer for Canvas {
    fn set_mut(&mut self, point: impl AsCoord, cell: impl Into<Cell>) {
        if let Some(c) = LayerIndexMut::get_mut(self, point) {
            *c = cell.into();
        }
    }
}
