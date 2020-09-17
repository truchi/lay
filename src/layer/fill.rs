use super::{Cell, Layer};
use crate::Styler;

#[derive(Debug)]
pub struct Fill {
    pub cell:   Cell,
    pub width:  u16,
    pub height: u16,
}

impl Fill {
    pub fn new<T: Into<Cell>>(cell: T, width: u16, height: u16) -> Self {
        let cell = cell.into();

        Self {
            cell,
            width,
            height,
        }
    }
}

impl Styler for Fill {
    impl_styler!(fill => fill.cell);
}

impl_styler_ops!(Fill);

impl<T: Into<Cell>> From<(T, u16, u16)> for Fill {
    fn from((cell, width, height): (T, u16, u16)) -> Self {
        Fill::new(cell, width, height)
    }
}

impl Layer for Fill {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn get_unchecked(&self, _: u16, _: u16) -> Cell {
        self.cell
    }
}
