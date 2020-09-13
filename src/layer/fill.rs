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

// impl From<(Cell)>

impl Layer for Fill {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn get(&self, _: u16, _: u16) -> Option<Cell> {
        Some(self.cell)
    }
}
