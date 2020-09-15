use super::{Cell, Layer};

#[derive(Debug)]
pub struct Fill {
    pub cell:   Cell,
    pub width:  u16,
    pub height: u16,
}

impl Fill {
    pub fn new(cell: Cell, width: u16, height: u16) -> Self {
        Self {
            cell,
            width,
            height,
        }
    }
}

impl From<(Cell, u16, u16)> for Fill {
    fn from((cell, width, height): (Cell, u16, u16)) -> Self {
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

    fn get(&self, x: u16, y: u16) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cell)
        } else {
            None
        }
    }
}
