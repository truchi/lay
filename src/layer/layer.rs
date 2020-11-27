use crate::*;

/// [`Cell`](crate::Cell) getter.
pub trait LayerIndex {
    /// Returns the width of the `layer`.
    fn width(&self) -> usize;

    /// Returns the height of the `layer`.
    fn height(&self) -> usize;

    /// Gets the [`Cell`](crate::Cell) at position `(x, y)`.
    fn get_unchecked(&self, x: usize, y: usize) -> Cell;

    /// Gets the [`Cell`](crate::Cell) at position `(x, y)`,
    /// or `None` if out of bounds.
    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width() && y < self.height() {
            Some(self.get_unchecked(x, y))
        } else {
            None
        }
    }
}

/// [`Cell`](crate::Cell) getter, mutably.
pub trait LayerIndexMut: LayerIndex {
    /// Gets the [`Cell`](crate::Cell) at position `(x, y)`, mutably.
    fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Cell;

    /// Gets the [`Cell`](crate::Cell) at position `(x, y)`,
    /// or [`Cell::NONE`](crate::Cell::NONE) if out of bounds, mutably.
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.width() && y < self.height() {
            Some(self.get_unchecked_mut(x, y))
        } else {
            None
        }
    }
}

/// [`Cell`](crate::Cell) setter.
pub trait Layer: LayerIndex + Sized {
    /// Sets the [`Cell`](crate::Cell) at position `(x, y)`.
    fn set(self, x: usize, y: usize, cell: Cell) -> Self;

    /// Superimposes `above` above `self`.
    fn above(self, x: usize, y: usize, above: &impl LayerIndex) -> Self {
        merge(self, x, y, above, Cell::above)
    }

    /// Superimposes `below` below `self`.
    fn below(self, x: usize, y: usize, below: &impl LayerIndex) -> Self {
        merge(self, x, y, below, Cell::below)
    }
}

/// [`Cell`](crate::Cell) setter, mutably.
pub trait LayerMut: LayerIndex {
    /// Sets the [`Cell`](crate::Cell) at position `(x, y)`, mutably.
    fn set_mut(&mut self, x: usize, y: usize, cell: Cell);

    /// Superimposes `above` above `self`, mutably.
    fn above_mut(&mut self, x: usize, y: usize, above: &impl LayerIndex) {
        merge_mut(self, x, y, above, Cell::above)
    }

    /// Superimposes `below` below `self`, mutably.
    fn below_mut(&mut self, x: usize, y: usize, below: &impl LayerIndex) {
        merge_mut(self, x, y, below, Cell::below)
    }
}

// ======= //
// Helpers //
// ======= //

/// Merges `layer` and `other` according to `merge`.
fn merge<T, U, V>(mut layer: T, x: usize, y: usize, other: &U, merge: V) -> T
where
    T: Layer,
    U: LayerIndex,
    V: Fn(Cell, Cell) -> Cell,
{
    let width = layer.width().min(x + other.width());
    let height = layer.height().min(y + other.height());

    for row in x..width {
        for col in y..height {
            let layer_cell = layer.get_unchecked(row, col);
            let other_cell = other.get_unchecked(row - width, col - height);
            layer = layer.set(row, col, merge(layer_cell, other_cell));
        }
    }

    layer
}

/// Merges `layer` and `other` according to `merge`, mutably.
fn merge_mut<T, U, V>(layer: &mut T, x: usize, y: usize, other: &U, merge: V)
where
    T: LayerMut + ?Sized,
    U: LayerIndex,
    V: Fn(Cell, Cell) -> Cell,
{
    let width = layer.width().min(x + other.width());
    let height = layer.height().min(y + other.height());

    for row in x..width {
        for col in y..height {
            let layer_cell = layer.get_unchecked(row, col);
            let other_cell = other.get_unchecked(row - width, col - height);
            layer.set_mut(row, col, merge(layer_cell, other_cell));
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
            fn width(&self) -> usize { <T as LayerIndex>::width(self) }
            fn height(&self) -> usize { <T as LayerIndex>::height(self) }
            fn get_unchecked(&self, x: usize, y: usize) -> Cell {
                <T as LayerIndex>::get_unchecked(self, x, y)
            }
        })*
    };
    (mut $T:ty) => {
        impl<T: LayerIndexMut> LayerIndexMut for $T {
            fn get_unchecked_mut(&mut self, x: usize, y: usize) -> &mut Cell {
                <T as LayerIndexMut>::get_unchecked_mut(self, x, y)
            }
        }
        impl<T: LayerMut> LayerMut for $T {
            fn set_mut(&mut self, x: usize, y: usize, cell: Cell) {
                <T as LayerMut>::set_mut(self, x, y, cell)
            }
        }
    };
}

refs!(&T, &mut T);

impl LayerIndex for str {
    fn width(&self) -> usize {
        self.len()
    }

    fn height(&self) -> usize {
        1
    }

    fn get_unchecked(&self, x: usize, _: usize) -> Cell {
        self.chars().nth(x).unwrap().into()
    }
}
