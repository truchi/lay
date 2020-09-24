use super::Cell;
use crate::Styler;

/// A rectangle filled by a unique `Cell`.
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

impl_layer!(Fill [fill, x, y] {
    Layer { fill.width } { fill.height } { fill.cell }
    Index { &fill.cell }
});
