use super::{Cell, Layer, LayerMut};
use crossterm::cursor::MoveTo;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Render<T: Layer> {
    layer: T,
    x:     u16,
    y:     u16,
}

impl<T: Layer> Render<T> {
    pub fn new(layer: T, x: u16, y: u16) -> Self {
        Self { layer, x, y }
    }
}

impl<T: Layer> From<(T, u16, u16)> for Render<T> {
    fn from((layer, x, y): (T, u16, u16)) -> Self {
        Self::new(layer, x, y)
    }
}

impl<T: Layer> From<T> for Render<T> {
    fn from(layer: T) -> Self {
        Self::new(layer, 0, 0)
    }
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

impl<T: LayerMut> LayerMut for Render<T> {
    fn get_mut_unchecked(&mut self, x: u16, y: u16) -> &mut Cell {
        self.layer.get_mut_unchecked(x, y)
    }
}

impl_layer_mut_ops!(Render<T: LayerMut,>);

impl<T: Layer> Display for Render<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for line in 0..self.height() {
            write!(f, "{}", MoveTo(self.x, self.y + line))?;

            for row in 0..self.width() {
                write!(f, "{}", self.get(row, line).unwrap())?;
            }
        }

        Ok(())
    }
}
