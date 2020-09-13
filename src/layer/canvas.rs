use super::{Cell, Layer, LayerMut};

#[derive(Eq, PartialEq, Default, Debug)]
pub struct Canvas {
    cells:  Vec<Cell>,
    width:  u16,
    height: u16,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        let size = width * height;
        let mut cells = Vec::with_capacity(usize::from(size));
        cells.resize(usize::from(size), ' '.into());

        Self {
            cells,
            width,
            height,
        }
    }
}

impl Layer for Canvas {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn get(&self, x: u16, y: u16) -> Option<Cell> {
        self.cells
            .get(usize::from(x) + usize::from(y) * usize::from(self.width))
            .copied()
    }
}

impl LayerMut for Canvas {
    fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        self.cells
            .get_mut(usize::from(x) + usize::from(y) * usize::from(self.width))
    }
}
