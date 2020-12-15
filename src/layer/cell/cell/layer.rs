use crate::*;
use std::iter::{once, Map, Once, Skip, Take};

impl LayerSize for Cell {
    fn size(&self) -> Coord {
        (1, 1)
    }
}

impl<'a> Layer<'a> for Cell {
    type Cells = Take<Skip<Once<Cell>>>;
    type Row = Take<Skip<Once<Cell>>>;
    type Rows = Map<Take<Skip<Once<(Cell, u16, u16)>>>, fn((Cell, u16, u16)) -> Self::Row>;

    fn cropped_row(&'a self, row: u16, col: u16, len: u16) -> Self::Row {
        once(*self)
            .skip(row as usize + col as usize)
            .take(len as usize)
    }

    fn cropped_rows(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Rows {
        once((*self, col, width))
            .skip(row as usize)
            .take(height as usize)
            .map(|(cell, col, width)| Layer::cropped_row(&cell, 0, col, width))
    }

    fn cropped_cells(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Cells {
        once(*self)
            .skip(row as usize + col as usize)
            .take(width as usize * height as usize)
    }
}

impl<'a> LayerMut<'a> for Cell {
    type CellsMut = Take<Skip<Once<&'a mut Cell>>>;
    type RowMut = Take<Skip<Once<&'a mut Cell>>>;
    type RowsMut = Map<
        Take<Skip<Once<(&'a mut Cell, u16, u16)>>>,
        fn((&'a mut Cell, u16, u16)) -> Self::RowMut,
    >;

    fn cropped_row_mut(&'a mut self, row: u16, col: u16, len: u16) -> Self::RowMut {
        once(self)
            .skip(row as usize + col as usize)
            .take(len as usize)
    }

    fn cropped_rows_mut(
        &'a mut self,
        col: u16,
        row: u16,
        width: u16,
        height: u16,
    ) -> Self::RowsMut {
        once((self, col, width))
            .skip(row as usize)
            .take(height as usize)
            .map(|(cell, col, width)| LayerMut::cropped_row_mut(cell, 0, col, width))
    }

    fn cropped_cells_mut(
        &'a mut self,
        col: u16,
        row: u16,
        width: u16,
        height: u16,
    ) -> Self::CellsMut {
        once(self)
            .skip(row as usize + col as usize)
            .take(width as usize * height as usize)
    }
}
