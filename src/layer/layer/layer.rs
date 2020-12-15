use crate::*;

pub trait LayerSize {
    fn size(&self) -> Coord;
}

pub trait Layer<'a>: LayerSize {
    type Row: Iterator<Item = Cell>;
    type Rows: Iterator<Item = Self::Row>;
    type Cells: Iterator<Item = Cell>;

    fn cropped_row(&'a self, row: u16, col: u16, len: u16) -> Self::Row;
    fn cropped_rows(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Rows;
    fn cropped_cells(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Cells;

    fn row(&'a self, row: u16) -> Self::Row {
        self.cropped_row(row, 0, self.size().0)
    }

    fn rows(&'a self) -> Self::Rows {
        let (width, height) = self.size();
        self.cropped_rows(0, 0, width, height)
    }

    fn cells(&'a self) -> Self::Cells {
        let (width, height) = self.size();
        self.cropped_cells(0, 0, width, height)
    }

    /// Writes this [`Layer`](crate::Layer) into `w` at cursor position `point`.
    ///
    /// Does not create new lines, make sure there is enough room available on
    /// the screen to display properly.
    ///
    /// Leaves cursor at last row, last column. Minimal CSIs.
    fn fmt(&'a self, w: &mut impl Write, point: impl AsCoord) -> io::Result<()> {
        let (x, y) = point.as_coord();
        let mut carry = Style::NONE;

        for (row, cells) in self.rows().enumerate() {
            write!(w, "{}", To(x, y + row as u16))?;

            for cell in cells {
                match cell {
                    Cell(Some(Styled { content, style })) => {
                        carry = style.dedup(&carry);
                        write!(w, "{}{}", carry, content)?;
                    }
                    _ => write!(w, "{}", Right(1))?,
                }
            }
        }

        Ok(())
    }
}

pub trait LayerMut<'a>: LayerSize {
    type RowMut: Iterator<Item = &'a mut Cell>;
    type RowsMut: Iterator<Item = Self::RowMut>;
    type CellsMut: Iterator<Item = &'a mut Cell>;

    fn cropped_row_mut(&'a mut self, row: u16, col: u16, len: u16) -> Self::RowMut;
    fn cropped_rows_mut(&'a mut self, col: u16, row: u16, width: u16, height: u16)
        -> Self::RowsMut;
    fn cropped_cells_mut(
        &'a mut self,
        col: u16,
        row: u16,
        width: u16,
        height: u16,
    ) -> Self::CellsMut;

    fn row_mut(&'a mut self, row: u16) -> Self::RowMut {
        self.cropped_row_mut(row, 0, self.size().0)
    }

    fn rows_mut(&'a mut self) -> Self::RowsMut {
        let (width, height) = self.size();
        self.cropped_rows_mut(0, 0, width, height)
    }

    fn cells_mut(&'a mut self) -> Self::CellsMut {
        let (width, height) = self.size();
        self.cropped_cells_mut(0, 0, width, height)
    }

    fn merge_mut<'b>(
        &'a mut self,
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

    fn fill_mut(&'a mut self, cell: Cell) {
        for self_cell in self.cells_mut() {
            *self_cell = cell
        }
    }
}
