use super::{Cell, Layer, LayerMut};
use std::ops::{Index, IndexMut};

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

impl_layer!(View<T> [view, x, y] {
    Layer <T: Layer,> { view.width } { view.height } {
        view.layer.get_unchecked(view.x + x, view.y + y)
    }
    Index <T: Layer Index<(u16, u16), Output = Cell>,> { &view.layer[(x, y)] }
    LayerMut <T: LayerMut,> {
        view.layer.get_mut_unchecked(view.x + x, view.y + y)
    }
    IndexMut <T: Layer IndexMut<(u16, u16), Output = Cell>,> { &mut view.layer[(x, y)] }
});

impl_layer_mut_ops!(View<T: LayerMut,>);
