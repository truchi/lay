use super::{Cell, Layer, LayerMut};
use crossterm::cursor::MoveTo;
use std::{
    fmt::{Display, Error, Formatter},
    ops::{Index, IndexMut},
};

/// A `Display`able `Layer`.
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

impl_layer!(Render<T> [render, x, y] {
    Layer <T: Layer,> { render.layer.width() } { render.layer.height() } {
        render.layer.get_unchecked(x, y)
    }
    Index <T: Layer Index<(u16, u16), Output = Cell>,> { &render.layer[(x, y)] }
    LayerMut <T: LayerMut,> {
        render.layer.get_mut_unchecked(x, y)
    }
    IndexMut <T: Layer IndexMut<(u16, u16), Output = Cell>,> { &mut render.layer[(x, y)] }
    + Ops <T: LayerMut,>
});

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
