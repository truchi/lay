use super::{Cell, Layer};
use crossterm::cursor::MoveTo;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Render<T: Layer> {
    pub layer:  T,
    pub width:  u16,
    pub height: u16,
    pub x:      u16,
    pub y:      u16,
}

impl<T: Layer> Layer for Render<T> {
    fn width(&self) -> u16 {
        self.layer.width()
    }

    fn height(&self) -> u16 {
        self.layer.height()
    }

    fn get_unchecked(&self, x: u16, y: u16) -> Cell {
        self.layer.get_unchecked(x, y)
    }
}

impl<T: Layer> Display for Render<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for line in 0..self.height.min(self.height()) {
            write!(f, "{}", MoveTo(self.x, self.y + line))?;

            for row in 0..self.width.min(self.width()) {
                write!(f, "{}", self.get(row, line).unwrap())?;
            }
        }

        Ok(())
    }
}
