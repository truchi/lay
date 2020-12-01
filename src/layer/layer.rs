use crate::*;

/// [`Cell`](crate::Cell) getter.
pub trait LayerIndex {
    /// Returns the [`Size`](crate::Size) of the `layer`.
    fn size(&self) -> Size;

    /// Gets the [`Cell`](crate::Cell) at `point`.
    fn get_unchecked(&self, point: impl Into<Point>) -> Cell;

    /// Gets the [`Cell`](crate::Cell) at point `(x, y)`,
    /// or `None` if out of bounds.
    fn get(&self, point: impl Into<Point>) -> Option<Cell> {
        let point = point.into();

        if point.lt(&Point::from(self.size())) {
            Some(self.get_unchecked(point))
        } else {
            None
        }
    }
}

/// [`Cell`](crate::Cell) getter, mutably.
pub trait LayerIndexMut: LayerIndex {
    /// Gets the [`Cell`](crate::Cell) at `point`, mutably.
    fn get_unchecked_mut(&mut self, point: impl Into<Point>) -> &mut Cell;

    /// Gets the [`Cell`](crate::Cell) at point `(x, y)`,
    /// or [`Cell::NONE`](crate::Cell::NONE) if out of bounds, mutably.
    fn get_mut(&mut self, point: impl Into<Point>) -> Option<&mut Cell> {
        let point = point.into();

        if point.lt(&Point::from(self.size())) {
            Some(self.get_unchecked_mut(point))
        } else {
            None
        }
    }
}

/// [`Cell`](crate::Cell) setter.
pub trait Layer: LayerIndex + Sized {
    /// Sets the [`Cell`](crate::Cell) at `point`.
    fn set(self, point: impl Into<Point>, cell: impl Into<Cell>) -> Self;

    /// Superimposes `above` above `self`.
    fn above(self, point: impl Into<Point>, above: &impl LayerIndex) -> Self {
        merge(self, point, above, Cell::above)
    }

    /// Superimposes `below` below `self`.
    fn below(self, positon: impl Into<Point>, below: &impl LayerIndex) -> Self {
        merge(self, positon, below, Cell::below)
    }
}

/// [`Cell`](crate::Cell) setter, mutably.
pub trait LayerMut: LayerIndex {
    /// Sets the [`Cell`](crate::Cell) at `point`, mutably.
    fn set_mut(&mut self, point: impl Into<Point>, cell: impl Into<Cell>);

    /// Superimposes `above` above `self`, mutably.
    fn above_mut(&mut self, point: impl Into<Point>, above: &impl LayerIndex) {
        merge_mut(self, point, above, Cell::above)
    }

    /// Superimposes `below` below `self`, mutably.
    fn below_mut(&mut self, point: impl Into<Point>, below: &impl LayerIndex) {
        merge_mut(self, point, below, Cell::below)
    }
}

// ======= //
// Helpers //
// ======= //

/// Merges `layer` and `other` according to `merge`.
fn merge<T, U, V>(mut layer: T, point: impl Into<Point>, other: &U, merge: V) -> T
where
    T: Layer,
    U: LayerIndex,
    V: Fn(Cell, Cell) -> Cell,
{
    let point = point.into();
    let (x, y) = point.into();
    let (layer_width, layer_height) = layer.size().into();
    let (other_width, other_height) = other.size().into();
    let width = layer_width.min(x + other_width);
    let height = layer_height.min(y + other_height);

    for row in x..width {
        for col in y..height {
            let layer_cell = layer.get_unchecked((row, col));
            let other_cell = other.get_unchecked((row - width, col - height));
            layer = layer.set((row, col), merge(layer_cell, other_cell));
        }
    }

    layer
}

/// Merges `layer` and `other` according to `merge`, mutably.
fn merge_mut<T, U, V>(layer: &mut T, point: impl Into<Point>, other: &U, merge: V)
where
    T: LayerMut + ?Sized,
    U: LayerIndex,
    V: Fn(Cell, Cell) -> Cell,
{
    let point = point.into();
    let (x, y) = point.into();
    let (layer_width, layer_height) = layer.size().into();
    let (other_width, other_height) = other.size().into();
    let width = layer_width.min(x + other_width);
    let height = layer_height.min(y + other_height);

    for row in x..width {
        for col in y..height {
            let layer_cell = layer.get_unchecked((row, col));
            let other_cell = other.get_unchecked((row - width, col - height));
            layer.set_mut((row, col), merge(layer_cell, other_cell));
        }
    }
}

// =============== //
// Implementations //
// =============== //

macro_rules! refs {
    ($Ref:ty, $Mut:ty) => { refs!(ref $Ref $Mut); refs!(mut $Mut); };
    (ref $($T:ty)*) => {
        $(impl<T: LayerIndex> LayerIndex for $T {
            fn size(&self) -> Size { LayerIndex::size(self) }
            fn get_unchecked(&self, point: impl Into<Point>) -> Cell {
                LayerIndex::get_unchecked(self, point)
            }
        })*
    };
    (mut $T:ty) => {
        impl<T: LayerIndexMut> LayerIndexMut for $T {
            fn get_unchecked_mut(&mut self, point: impl Into<Point>) -> &mut Cell {
                LayerIndexMut::get_unchecked_mut(self, point)
            }
        }
        impl<T: LayerMut> LayerMut for $T {
            fn set_mut(&mut self, point: impl Into<Point>, cell: impl Into<Cell>) {
                LayerMut::set_mut(self, point, cell)
            }
        }
    };
}

refs!(&T, &mut T);

impl LayerIndex for str {
    fn size(&self) -> Size {
        Size {
            width:  Width(self.len()),
            height: Height(1),
        }
    }

    fn get_unchecked(&self, point: impl Into<Point>) -> Cell {
        self.chars().nth(point.into().x.0).unwrap().into()
    }
}
