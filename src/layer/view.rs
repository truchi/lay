use crate::*;

/// A [`View`](crate::View) into a [`Layer`](crate::Layer).
#[derive(Debug)]
pub struct View<T: Layer> {
    pub layer: T,
    rect:      Rect,
}

/// ### Constructors
impl<T: Layer> View<T> {
    // TODO
    //  /// Returns a new [`View`](crate::View).
    //  pub fn new(layer: T, origin: Position, size: Size) -> Self {
    //      let (x, y) = origin;
    //      let (width, height) = size;
    //      let (layer_width, layer_height) = layer.size();
    //      let x = x.min(layer_width);
    //      let y = y.min(layer_height);
    //      let width = (width + x).min(layer_width) - x;
    //      let height = (height + y).min(layer_height) - y;
    //
    //      Self {
    //          layer,
    //          origin: (width, height),
    //          size: (x, y),
    //      }
    //  }
}

/// ### Methods
impl<T: Layer> View<T> {
    /// Returns this [`View`](crate::View)'s [`Rect`](crate::Rect).
    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    /// Returns this [`View`](crate::View)'s [`Rect`](crate::Rect)'s `origin`.
    pub fn get_origin(&self) -> Position {
        self.rect.0
    }

    /// Returns this [`View`](crate::View)'s [`Rect`](crate::Rect)'s `size`.
    pub fn get_size(&self) -> Size {
        self.rect.1
    }

    /// Sets this [`View`](crate::View)'s [`Rect`](crate::Rect).
    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect_mut(rect);
        self
    }

    /// Sets this [`View`](crate::View)'s [`Rect`](crate::Rect)'s `origin`.
    pub fn origin(mut self, origin: Position) -> Self {
        self.origin_mut(origin);
        self
    }

    /// Sets this [`View`](crate::View)'s [`Rect`](crate::Rect)'s `size`.
    pub fn size(mut self, size: Size) -> Self {
        self.size_mut(size);
        self
    }

    /// Sets this [`View`](crate::View)'s [`Rect`](crate::Rect), mutably.
    /// Returns the actual, bound checked [`Rect`](crate::Rect).
    pub fn rect_mut(&mut self, rect: Rect) -> Rect {
        let ((x, y), (width, height)) = rect;
        let (layer_width, layer_height) = self.layer.size();
        let x = x.min(layer_width);
        let y = y.min(layer_height);
        let width = (width + x).min(layer_width) - x;
        let height = (height + y).min(layer_height) - y;
        let rect = ((x, y), (width, height));

        self.rect = rect;
        rect
    }

    /// Sets this [`View`](crate::View)'s [`Rect`](crate::Rect)'s `origin`,
    /// mutably. Returns the actual, bound checked [`Rect`](crate::Rect).
    pub fn origin_mut(&mut self, origin: Position) -> Rect {
        self.rect_mut((origin, self.rect.1))
    }

    /// Sets this [`View`](crate::View)'s [`Rect`](crate::Rect)'s `size`,
    /// mutably. Returns the actual, bound checked [`Rect`](crate::Rect).
    pub fn size_mut(&mut self, size: Size) -> Rect {
        self.rect_mut((self.rect.0, size))
    }
}
