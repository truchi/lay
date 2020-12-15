use crate::*;
use std::iter::Flatten;

pub struct Cells<'a, T: Layer<'a>> {
    cells: Flatten<T::Rows>,
}

impl<'a, T: Layer<'a>> Cells<'a, T> {
    pub fn new(layer: &'a T, col: u16, row: u16, width: u16, height: u16) -> Self {
        Self {
            cells: layer.cropped_rows(col, row, width, height).flatten(),
        }
    }
}

impl<'a, T: Layer<'a>> Iterator for Cells<'a, T> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.cells.next()
    }
}
