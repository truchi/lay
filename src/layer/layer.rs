use crate::*;

macro_rules! write_row {
    ($w:ident, $layer:ident, $row:expr, $width:expr, $carry:ident) => {{
        let row = $row;
        let width = $width;
        for col in 0..width {
            match $layer.get_unchecked((col, row)) {
                Cell(Some(Styled { content, style })) => {
                    $carry = style.dedup(&$carry);
                    write!($w, "{}{}", $carry, content)?;
                }
                _ => write!($w, "{}", Right(1))?,
            }
        }
        $carry
    }};
}

/// [`Cell`](crate::Cell) getter.
pub trait LayerIndex {
    /// Returns the size of the `layer`.
    fn size(&self) -> Coord;

    /// Gets the [`Cell`](crate::Cell) at `point`.
    fn get_unchecked(&self, point: impl AsCoord) -> Cell;

    /// Gets the [`Cell`](crate::Cell) at point `(x, y)`,
    /// or `None` if out of bounds.
    fn get(&self, point: impl AsCoord) -> Option<Cell> {
        if point.lt(self.size()) {
            Some(self.get_unchecked(point))
        } else {
            None
        }
    }

    /// Writes this [`Layer`](crate::Layer) into `w` at cursor position `point`.
    ///
    /// Does not create new lines, make sure there is enough room available on
    /// the screen to display properly.
    ///
    /// Leaves cursor at last row, last column. Minimal CSIs.
    fn fmt(&self, w: &mut impl Write, point: impl AsCoord) -> io::Result<()> {
        let (x, y) = point.as_coord();
        let (width, height) = self.size();

        if width != 0 {
            let mut carry = Style::NONE;
            for row in 0..height {
                write!(w, "{}", To(x, y + row))?;
                carry = write_row!(w, self, row, width, carry);
            }
        }

        Ok(())
    }

    /// Writes this [`Layer`](crate::Layer) into `w` at current cursor position.
    ///
    /// Creates new lines, make sure the induced scrolling is under control.
    ///
    /// Leaves cursor at last row, last column. Minimal CSIs.
    fn fmt_at_cursor(&self, f: &mut Formatter) -> fmt::Result {
        let (width, height) = self.size();

        if width != 0 && height != 0 {
            let mut carry = Style::NONE;
            for row in 0..height - 1 {
                carry = write_row!(f, self, row, width, carry);
                write!(f, "\n")?;
            }
            carry = write_row!(f, self, height - 1, width, carry);
            write!(f, "{}", carry.reset())?;
        }

        Ok(())
    }
}

/// [`Cell`](crate::Cell) getter, mutably.
pub trait LayerIndexMut: LayerIndex {
    /// Gets the [`Cell`](crate::Cell) at `point`, mutably.
    fn get_unchecked_mut(&mut self, point: impl AsCoord) -> &mut Cell;

    /// Gets the [`Cell`](crate::Cell) at point `(x, y)`,
    /// or [`Cell::NONE`](crate::Cell::NONE) if out of bounds, mutably.
    fn get_mut(&mut self, point: impl AsCoord) -> Option<&mut Cell> {
        if point.lt(self.size()) {
            Some(self.get_unchecked_mut(point))
        } else {
            None
        }
    }
}

/// [`Cell`](crate::Cell) setter.
pub trait Layer: LayerIndex + Sized {
    /// Sets the [`Cell`](crate::Cell) at `point`.
    fn set(mut self, point: impl AsCoord, cell: impl Into<Cell>) -> Self {
        self.set_mut(point, cell);
        self
    }

    /// Merges `self` and `other` with the `merge` function.
    ///
    /// Redirects to [`Layer::merge_mut`](crate::Layer::merge_mut).
    fn merge(mut self, other: &impl LayerIndex, merge: impl Fn(Cell, Cell) -> Cell) -> Self {
        self.merge_mut(other, merge);
        self
    }

    /// Superimposes `above` above `self`.
    ///
    /// Calls [`Layer::merge`](crate::Layer::merge) with
    /// [`Cell::above`](crate::Cell::above).
    fn above(self, above: &impl LayerIndex) -> Self {
        self.merge(above, Cell::above)
    }

    /// Superimposes `below` below `self`.
    ///
    /// Calls [`Layer::merge`](crate::Layer::merge) with
    /// [`Cell::below`](crate::Cell::below).
    fn below(self, below: &impl LayerIndex) -> Self {
        self.merge(below, Cell::below)
    }

    /// Fills this [`Layer`](crate::Layer) with `cell`.
    ///
    /// Redirects to [`Layer::fill_mut`](crate::Layer::fill_mut).
    fn fill(mut self, cell: impl Into<Cell>) -> Self {
        self.fill_mut(cell.into());
        self
    }

    /// Clears this [`Layer`](crate::Layer).
    ///
    /// Calls [`Layer::fill`](crate::Layer::fill) with
    /// [`Cell::NONE`](crate::Cell::NONE).
    fn clear(self) -> Self {
        self.fill(Cell::NONE)
    }

    // -------
    // Mutable
    // -------

    /// Sets the [`Cell`](crate::Cell) at `point`, mutably.
    fn set_mut(&mut self, point: impl AsCoord, cell: impl Into<Cell>);

    /// Merges `self` and `other` with the `merge` function, mutably.
    fn merge_mut(&mut self, other: &impl LayerIndex, merge: impl Fn(Cell, Cell) -> Cell) {
        merge_mut(self, other, merge);
    }

    /// Superimposes `above` above `self`, mutably.
    ///
    /// Calls [`Layer::merge_mut`](crate::Layer::merge_mut) with
    /// [`Cell::above`](crate::Cell::above).
    fn above_mut(&mut self, above: &impl LayerIndex) {
        self.merge_mut(above, Cell::above);
    }

    /// Superimposes `below` below `self`, mutably.
    ///
    /// Calls [`Layer::merge_mut`](crate::Layer::merge_mut) with
    /// [`Cell::below`](crate::Cell::below).
    fn below_mut(&mut self, below: &impl LayerIndex) {
        self.merge_mut(below, Cell::below);
    }

    /// Fills this [`Layer`](crate::Layer) with `cell`, mutably.
    fn fill_mut(&mut self, cell: impl Into<Cell>) {
        fill_mut(self, cell.into());
    }

    /// Clears this [`Layer`](crate::Layer), mutably.
    ///
    /// Calls [`Layer::fill_mut`](crate::Layer::fill_mut) with
    /// [`Cell::NONE`](crate::Cell::NONE).
    fn clear_mut(&mut self) {
        self.fill_mut(Cell::NONE);
    }
}

// ======= //
// Helpers //
// ======= //

/// Merges `layer` and `other` with the `merge` function, mutably.
fn merge_mut(layer: &mut impl Layer, other: &impl LayerIndex, merge: impl Fn(Cell, Cell) -> Cell) {
    let (width, height) = layer.size();
    let (other_width, other_height) = other.size();
    let height = height.min(other_height);

    if height != 0 {
        let width = width.min(other_width);

        for x in 0..width {
            for y in 0..height {
                let layer_cell = layer.get_unchecked((x, y));
                let other_cell = other.get_unchecked((x, y));
                let cell = merge(layer_cell, other_cell);
                layer.set_mut((x, y), cell);
            }
        }
    }
}

/// Fills this [`Layer`](crate::Layer) with `cell`, mutably.
fn fill_mut(layer: &mut impl Layer, cell: Cell) {
    let (width, height) = layer.size();

    if width != 0 {
        for row in 0..height {
            for col in 0..width {
                layer.set_mut((col, row), cell);
            }
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
            fn size(&self) -> Coord { <T as LayerIndex>::size(self) }
            fn get_unchecked(&self, point: impl AsCoord) -> Cell {
                <T as LayerIndex>::get_unchecked(self, point)
            }
        })*
    };
    (mut $T:ty) => {
        impl<T: LayerIndexMut> LayerIndexMut for $T {
            fn get_unchecked_mut(&mut self, point: impl AsCoord) -> &mut Cell {
                <T as LayerIndexMut>::get_unchecked_mut(self, point)
            }
        }
        impl<T: Layer> Layer for $T {
            fn set_mut(&mut self, point: impl AsCoord, cell: impl Into<Cell>) {
                <T as Layer>::set_mut(self, point, cell)
            }
        }
    };
}

// TODO is this really doing something?
refs!(&T, &mut T);

impl LayerIndex for str {
    fn size(&self) -> Coord {
        (self.len() as u16, 1)
    }

    fn get_unchecked(&self, point: impl AsCoord) -> Cell {
        match self.chars().nth(point.x() as usize) {
            Some(char) => char.into(),
            _ => Cell::NONE,
        }
    }
}
