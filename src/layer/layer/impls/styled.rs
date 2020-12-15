use crate::*;
use std::iter::{repeat, Map, Repeat, Zip};

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
