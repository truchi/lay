use super::{Cell, Layer, LayerMut};

/// A rectangle `View` into a `Layer`.
#[derive(Debug)]
pub struct View<T: Layer> {
    layer:  T,
    x:      u16,
    y:      u16,
    width:  u16,
    height: u16,
}

impl<T: Layer> View<T> {
    pub fn new(layer: T, x: u16, y: u16, width: u16, height: u16) -> Self {
        let layer_width = layer.width();
        let layer_height = layer.height();
        let x = layer_width.min(x);
        let y = layer_height.min(y);
        let width = layer_width.min(x + width) - x;
        let height = layer_height.min(y + height) - y;

        Self {
            layer,
            x,
            y,
            width,
            height,
        }
    }
}

impl<T: Layer> From<(T, u16, u16, u16, u16)> for View<T> {
    fn from((layer, x, y, width, height): (T, u16, u16, u16, u16)) -> Self {
        Self::new(layer, x, y, width, height)
    }
}

impl<T: Layer> From<T> for View<T> {
    fn from(layer: T) -> Self {
        let width = layer.width();
        let height = layer.height();

        Self::new(layer, 0, 0, width, height)
    }
}

impl<T: Layer> Layer for View<T> {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn get_unchecked(&self, x: u16, y: u16) -> Cell {
        self.layer.get_unchecked(self.x + x, self.y + y)
    }
}

impl<T: LayerMut> LayerMut for View<T> {
    fn get_mut_unchecked(&mut self, x: u16, y: u16) -> &mut Cell {
        self.layer.get_mut_unchecked(self.x + x, self.y + y)
    }
}

impl_layer_mut_ops!(View<T: LayerMut,>);
