use crate::*;
use std::{
    iter::{repeat, Flatten, Map, Repeat, Skip, Take, Zip},
    str::Chars,
};

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

// ================ //
// Iterator helpers //
// ================ //

pub struct Rows<'a, T: ?Sized> {
    layer: &'a T,
    col:   u16,
    width: u16,
    row:   u16,
    end:   u16,
}

impl<'a, T: LayerSize + ?Sized> Rows<'a, T> {
    pub fn new(layer: &'a T, col: u16, row: u16, width: u16, height: u16) -> Self {
        // layer.cropped_row will handle X-axis bound checks
        // we handle Y-axis bound checks here to avoid empty rows
        let (_, layer_height) = layer.size();
        let row = row.min(layer_height);
        let height = height.min(layer_height.saturating_sub(row));

        Self {
            layer,
            col,
            width,
            row,
            end: row + height,
        }
    }
}

impl<'a, T: Layer<'a> + ?Sized> Iterator for Rows<'a, T> {
    type Item = T::Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.end {
            let item = self.layer.cropped_row(self.row, self.col, self.width);
            self.row += 1;
            Some(item)
        } else {
            None
        }
    }
}

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

// =============== //
// Implementations //
// =============== //

impl LayerSize for str {
    fn size(&self) -> Coord {
        (self.len() as u16, 1)
    }
}

impl<'a> Layer<'a> for str {
    type Cells = Self::Row;
    type Row = Map<Take<Skip<Chars<'a>>>, fn(char) -> Cell>;
    type Rows = Rows<'a, Self>;

    fn cropped_row(&'a self, row: u16, col: u16, len: u16) -> Self::Row {
        // Row should be empty when row != 0
        let str = if row == 0 { self } else { "" };

        str.chars()
            .skip(col as usize)
            .take(len as usize)
            .map(|char| char.into())
    }

    fn cropped_rows(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Rows {
        // Let's use the Rows helper to handle Y-axis gracefully
        Rows::new(self, col, row, width, height)
    }

    fn cropped_cells(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Cells {
        // Cells should be empty when requested height is 0
        Layer::cropped_row(self, row, col, if height == 0 { 0 } else { width })
    }
}

impl<T: AsRef<str>> LayerSize for Styled<T> {
    fn size(&self) -> Coord {
        LayerSize::size(self.content.as_ref())
    }
}

impl<'a, T: 'a + AsRef<str>> Layer<'a> for Styled<T> {
    type Cells = Self::Row;
    type Row = Map<Zip<<str as Layer<'a>>::Row, Repeat<Style>>, fn((Cell, Style)) -> Cell>;
    type Rows = Rows<'a, Self>;

    fn cropped_row(&'a self, row: u16, col: u16, len: u16) -> Self::Row {
        // Reusing Layer for str
        Layer::cropped_row(self.content.as_ref(), row, col, len)
            .zip(repeat(self.style))
            .map(|(cell, style)| cell.style(&style))
    }

    fn cropped_rows(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Rows {
        // Let's use the Rows helper to handle Y-axis gracefully
        Rows::new(self, col, row, width, height)
    }

    fn cropped_cells(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Cells {
        // Cells should be empty when requested height is 0
        Layer::cropped_row(self, row, col, if height == 0 { 0 } else { width })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn str() {
        let str = "0123456789";

        fn to_string(it: impl Iterator<Item = Cell>) -> String {
            it.map(|cell| {
                let styled = cell.0.expect("Cell to be Some");
                assert_eq!(styled.style, Style::NONE);
                styled.content
            })
            .collect::<String>()
        }

        assert_eq!(LayerSize::size(str), (10, 1), "Size is (len, 1)");
        assert_eq!(
            Layer::cropped_row(str, 1, 0, 10).count(),
            0,
            "Nothing in row 1+"
        );
        assert_eq!(to_string(Layer::cropped_row(str, 0, 0, 10)), str);
        assert_eq!(to_string(Layer::cropped_row(str, 0, 3, 2)), &str[3..5]);
        assert_eq!(to_string(Layer::cropped_row(str, 0, 3, 20)), &str[3..]);
    }

    #[test]
    fn styled() {
        let str = "0123456789";
        let style = Style::NONE.blue().on_red().fast().reset_underline();
        let styled = Styled {
            content: str,
            style,
        };

        fn to_string(it: impl Iterator<Item = Cell>, style: Style) -> String {
            it.map(|cell| {
                let styled = cell.0.expect("Cell to be Some");
                assert_eq!(styled.style, style);
                styled.content
            })
            .collect::<String>()
        }

        assert_eq!(LayerSize::size(&styled), (10, 1), "Size is (len, 1)");
        assert_eq!(
            Layer::cropped_row(&styled, 1, 0, 10).count(),
            0,
            "Nothing in row 1+"
        );
        assert_eq!(to_string(Layer::cropped_row(&styled, 0, 0, 10), style), str);
        assert_eq!(
            to_string(Layer::cropped_row(&styled, 0, 3, 2), style),
            &str[3..5]
        );
        assert_eq!(
            to_string(Layer::cropped_row(&styled, 0, 3, 20), style),
            &str[3..]
        );
    }
}
