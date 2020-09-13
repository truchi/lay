use super::{Cell, Layer};
use crossterm::cursor::MoveTo;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Render<'a, T: Layer> {
    pub layer:  &'a T,
    pub width:  u16,
    pub height: u16,
    pub x:      u16,
    pub y:      u16,
}

impl<'a, T: Layer> Layer for Render<'a, T> {
    fn width(&self) -> u16 {
        self.layer.width()
    }

    fn height(&self) -> u16 {
        self.layer.height()
    }

    fn get(&self, x: u16, y: u16) -> Option<Cell> {
        self.layer.get(x, y)
    }
}

impl<'a, T: Layer> Display for Render<'a, T> {
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
