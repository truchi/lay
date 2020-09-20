use super::{Cell, Layer, LayerMut};

#[derive(Eq, PartialEq, Default, Debug)]
pub struct Canvas {
    cells:  Vec<Cell>,
    width:  u16,
    height: u16,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self::with_cell(Cell::EMPTY, width, height)
    }

    pub fn with_cell(cell: Cell, width: u16, height: u16) -> Self {
        let size = width * height;
        let mut cells = Vec::with_capacity(usize::from(size));
        cells.resize(usize::from(size), cell);

        Self {
            cells,
            width,
            height,
        }
    }
}

impl From<(u16, u16)> for Canvas {
    fn from((width, height): (u16, u16)) -> Self {
        Self::new(width, height)
    }
}

impl Layer for Canvas {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn get_unchecked(&self, x: u16, y: u16) -> Cell {
        self.cells
            .get(usize::from(x) + usize::from(y) * usize::from(self.width))
            .copied()
            .unwrap()
    }
}

impl LayerMut for Canvas {
    fn get_mut_unchecked(&mut self, x: u16, y: u16) -> &mut Cell {
        self.cells
            .get_mut(usize::from(x) + usize::from(y) * usize::from(self.width))
            .unwrap()
    }
}

impl_layer_mut_ops!(Canvas);
