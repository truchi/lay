use crate::*;
use std::{
    iter::{Map, Skip, Take},
    str::Chars,
};

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
}
