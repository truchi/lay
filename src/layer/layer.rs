use crate::*;

/// [`Cell`](crate::Cell) getter.
pub trait LayerIndex {
    /// Returns the [`Size`](crate::Size) of the `layer`.
    fn size(&self) -> Size;

    /// Gets the [`Cell`](crate::Cell) at `position`.
    fn get_unchecked(&self, position: Position) -> Cell;

    /// Gets the [`Cell`](crate::Cell) at position `(x, y)`,
    /// or `None` if out of bounds.
    fn get(&self, (x, y): Position) -> Option<Cell> {
        let (width, height) = self.size();

        if x < width && y < height {
            Some(self.get_unchecked((x, y)))
        } else {
            None
        }
    }
}

/// [`Cell`](crate::Cell) getter, mutably.
pub trait LayerIndexMut: LayerIndex {
    /// Gets the [`Cell`](crate::Cell) at `position`, mutably.
    fn get_unchecked_mut(&mut self, position: Position) -> &mut Cell;

    /// Gets the [`Cell`](crate::Cell) at position `(x, y)`,
    /// or [`Cell::NONE`](crate::Cell::NONE) if out of bounds, mutably.
    fn get_mut(&mut self, (x, y): Position) -> Option<&mut Cell> {
        let (width, height) = self.size();

        if x < width && y < height {
            Some(self.get_unchecked_mut((x, y)))
        } else {
            None
        }
    }
}

/// [`Cell`](crate::Cell) setter.
pub trait Layer: LayerIndex + Sized {
    /// Sets the [`Cell`](crate::Cell) at `position`.
    fn set(self, position: Position, cell: Cell) -> Self;

    /// Superimposes `above` above `self`.
    fn above(self, position: Position, above: &impl LayerIndex) -> Self {
        merge(self, position, above, Cell::above)
    }

    /// Superimposes `below` below `self`.
    fn below(self, positon: Position, below: &impl LayerIndex) -> Self {
        merge(self, positon, below, Cell::below)
    }
}

/// [`Cell`](crate::Cell) setter, mutably.
pub trait LayerMut: LayerIndex {
    /// Sets the [`Cell`](crate::Cell) at `position`, mutably.
    fn set_mut(&mut self, position: Position, cell: Cell);

    /// Superimposes `above` above `self`, mutably.
    fn above_mut(&mut self, position: Position, above: &impl LayerIndex) {
        merge_mut(self, position, above, Cell::above)
    }

    /// Superimposes `below` below `self`, mutably.
    fn below_mut(&mut self, position: Position, below: &impl LayerIndex) {
        merge_mut(self, position, below, Cell::below)
    }
}

// ======= //
// Helpers //
// ======= //

/// Merges `layer` and `other` according to `merge`.
fn merge<T, U, V>(mut layer: T, (x, y): Position, other: &U, merge: V) -> T
where
    T: Layer,
    U: LayerIndex,
    V: Fn(Cell, Cell) -> Cell,
{
    let (layer_width, layer_height) = layer.size();
    let (other_width, other_height) = other.size();
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
fn merge_mut<T, U, V>(layer: &mut T, (x, y): Position, other: &U, merge: V)
where
    T: LayerMut + ?Sized,
    U: LayerIndex,
    V: Fn(Cell, Cell) -> Cell,
{
    let (layer_width, layer_height) = layer.size();
    let (other_width, other_height) = other.size();
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
            fn size(&self) -> Size { <T as LayerIndex>::size(self) }
            fn get_unchecked(&self, position: Position) -> Cell {
                <T as LayerIndex>::get_unchecked(self, position)
            }
        })*
    };
    (mut $T:ty) => {
        impl<T: LayerIndexMut> LayerIndexMut for $T {
            fn get_unchecked_mut(&mut self, position: Position) -> &mut Cell {
                <T as LayerIndexMut>::get_unchecked_mut(self, position)
            }
        }
        impl<T: LayerMut> LayerMut for $T {
            fn set_mut(&mut self, position: Position, cell: Cell) {
                <T as LayerMut>::set_mut(self, position, cell)
            }
        }
    };
}

refs!(&T, &mut T);

impl LayerIndex for str {
    fn size(&self) -> Size {
        (self.len(), 1)
    }

    fn get_unchecked(&self, (x, _): Position) -> Cell {
        self.chars().nth(x).unwrap().into()
    }
}
