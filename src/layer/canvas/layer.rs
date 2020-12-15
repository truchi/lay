use crate::*;
use std::{
    iter::{Copied, Flatten},
    slice::{Iter, IterMut},
};

pub type CanvasRow<'a> = Copied<Iter<'a, Cell>>;
pub type CanvasRowMut<'a> = IterMut<'a, Cell>;
pub type CanvasCells<'a> = Flatten<CanvasRows<'a>>;
pub type CanvasCellsMut<'a> = Flatten<CanvasRowsMut<'a>>;

impl LayerSize for Canvas {
    fn size(&self) -> Coord {
        self.size
    }
}

impl<'a> Layer<'a> for Canvas {
    type Cells = CanvasCells<'a>;
    type Row = CanvasRow<'a>;
    type Rows = CanvasRows<'a>;

    fn cropped_row(&self, row: u16, col: u16, len: u16) -> CanvasRow {
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

    fn cropped_rows(&self, col: u16, row: u16, width: u16, height: u16) -> CanvasRows {
        CanvasRows::new(self, col, row, width, height)
    }

    fn cropped_cells(&self, col: u16, row: u16, width: u16, height: u16) -> CanvasCells {
        self.cropped_rows(col, row, width, height).flatten()
    }
}

impl<'a> LayerMut<'a> for Canvas {
    type CellsMut = CanvasCellsMut<'a>;
    type RowMut = CanvasRowMut<'a>;
    type RowsMut = CanvasRowsMut<'a>;

    fn cropped_row_mut(&mut self, row: u16, col: u16, len: u16) -> CanvasRowMut {
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

    fn cropped_rows_mut(&mut self, col: u16, row: u16, width: u16, height: u16) -> CanvasRowsMut {
        CanvasRowsMut::new(self, col, row, width, height)
    }

    fn cropped_cells_mut(&mut self, col: u16, row: u16, width: u16, height: u16) -> CanvasCellsMut {
        self.rows_mut(col, row, width, height).flatten()
    }
}
