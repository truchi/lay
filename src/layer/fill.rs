use crate::*;
use std::iter::{repeat, Repeat, Take};

/// A plain [`Fill`](crate::Fill).
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Fill {
    pub size: Coord,
    pub cell: Cell,
}

pub type FillRowIter = Take<Repeat<Cell>>;
pub type FillCellsIter = FillRowIter;

/// ### Constructors
impl Fill {
    /// Returns a new [`Fill`](crate::Fill).
    pub fn new(size: impl AsCoord, cell: impl Into<Cell>) -> Self {
        Self {
            size: size.as_coord(),
            cell: cell.into(),
        }
    }
}

/// ### Constructors
impl Fill {
    pub fn row(&self, row: u16, col: u16, len: u16) -> FillRowIter {
        let (width, height) = self.size;

        if width == 0 || height == 0 || row >= height || col >= width || len == 0 {
            repeat(Cell::NONE).take(0)
        } else {
            let len = len.min(width - col);
            repeat(self.cell).take(len as usize)
        }
    }

    pub fn rows(&self, col: u16, row: u16, width: u16, height: u16) -> FillRowsIter {
        FillRowsIter::new(self, col, row, width, height)
    }

    pub fn cells(&self, col: u16, row: u16, width: u16, height: u16) -> FillCellsIter {
        let point = (col, row).min(self.size);
        let (width, height) = (width, height).min(self.size.sub(point));

        repeat(self.cell).take(width as usize * height as usize)
    }
}

impl LayerSize for Fill {
    fn size(&self) -> Coord {
        self.size
    }
}

pub struct FillRowsIter {
    cell:    Cell,
    width:   u16,
    height:  u16,
    current: u16,
}

impl FillRowsIter {
    pub fn new(fill: &Fill, col: u16, row: u16, width: u16, height: u16) -> Self {
        let (fill_width, fill_height) = fill.size();

        if fill_width == 0
            || fill_height == 0
            || width == 0
            || height == 0
            || col >= fill_width
            || row >= fill_height
        {
            return Self {
                cell:    Cell::NONE,
                width:   0,
                height:  0,
                current: 0,
            };
        }

        let (width, height) = (width.min(fill_width - col), height.min(fill_height - row));

        debug_assert!(col < fill_width);
        debug_assert!(row < fill_height);
        debug_assert!(col as usize + width as usize <= fill_width as usize);
        debug_assert!(row as usize + height as usize <= fill_height as usize);

        return Self {
            cell: fill.cell,
            width,
            height,
            current: 0,
        };
    }
}

impl Iterator for FillRowsIter {
    type Item = FillRowIter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.height {
            Some(repeat(self.cell).take(self.width as usize))
        } else {
            None
        }
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Fill`](crate::Fill).
impl<T: AsCoord, U: Into<Cell>> From<(T, U)> for Fill {
    /// Returns a new [`Fill`](crate::Fill).
    fn from((size, cell): (T, U)) -> Self {
        Self::new(size, cell)
    }
}
