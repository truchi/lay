use crate::*;

/// A [`Rect`](crate::Rect) [`View`](crate::View) into a
/// [`Layer`](crate::Layer).
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Hash, Default, Debug)]
pub struct View<T> {
    layer: T,
    rect:  Rect,
}

/// ### Constructors
impl<T: LayerSize> View<T> {
    /// Returns a new [`View`](crate::View).
    pub fn new(layer: T, rect: impl AsRect) -> Self {
        Self {
            layer,
            rect: RECT_ZERO,
        }
        .rect(rect)
    }
}

/// ### Methods
impl<T: LayerSize> View<T> {
    /// Returns this [`View`](crate::View)'s `layer`.
    pub fn into_layer(self) -> T {
        self.layer
    }

    /// Returns this [`View`](crate::View)'s `&layer`.
    pub fn get_layer(&self) -> &T {
        &self.layer
    }

    /// Returns this [`View`](crate::View)'s [`Rect`](crate::Rect).
    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    /// Sets [`Rect`](crate::Rect).
    pub fn rect(mut self, rect: impl AsRect) -> Self {
        self.rect_mut(rect);
        self
    }

    /// Sets [`Rect`](crate::Rect)'s `origin`.
    pub fn origin(mut self, origin: impl AsCoord) -> Self {
        self.origin_mut(origin);
        self
    }

    /// Sets [`Rect`](crate::Rect)'s `size`.
    pub fn size(mut self, size: impl AsCoord) -> Self {
        self.size_mut(size);
        self
    }

    /// Sets [`Rect`](crate::Rect), mutably.
    /// Returns the actual, bound checked [`Rect`](crate::Rect).
    pub fn rect_mut(&mut self, rect: impl AsRect) -> Rect {
        self.rect = rect.crop(self.layer.size());
        self.rect
    }

    /// Sets [`Rect`](crate::Rect)'s `origin`, mutably.
    /// Returns the actual, bound checked [`Rect`](crate::Rect).
    pub fn origin_mut(&mut self, origin: impl AsCoord) -> Rect {
        self.rect_mut((origin, self.rect.size()))
    }

    /// Sets [`Rect`](crate::Rect)'s `size`, mutably.
    /// Returns the actual, bound checked [`Rect`](crate::Rect).
    pub fn size_mut(&mut self, size: impl AsCoord) -> Rect {
        self.rect_mut((self.rect.point(), size))
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`View`](crate::View) at full size.
impl<T: LayerSize> From<T> for View<T> {
    /// Returns a new [`View`](crate::View) at full size.
    fn from(layer: T) -> Self {
        let size = layer.size();
        Self::new(layer, size)
    }
}

/// Returns a new [`View`](crate::View).
impl<T: LayerSize, U: AsRect> From<(T, U)> for View<T> {
    /// Returns a new [`View`](crate::View).
    fn from((layer, rect): (T, U)) -> Self {
        Self::new(layer, rect)
    }
}
