use crate::*;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct DamagedCell {
    pub current: Cell,
    previous:    Cell,
}

impl DamagedCell {
    pub fn new(current: impl Into<Cell>) -> Self {
        Self {
            current:  current.into(),
            previous: Cell::NONE,
        }
    }
}

impl<T: Into<Cell>> From<T> for DamagedCell {
    fn from(current: T) -> Self {
        Self::new(current)
    }
}

impl AsDamagedCell for DamagedCell {
    fn get_cell(&self) -> Cell {
        self.current
    }

    fn get_damage(&self) -> Cell {
        self.current.get_damage(self.previous)
    }

    fn save_mut(&mut self) {
        self.previous = self.current;
    }
}
