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

    /// Writes this [`Layer`](crate::Layer) into `w` at cursor position `point`.
    ///
    /// Does not create new lines, make sure there is enough room available on
    /// the screen to display properly.
    ///
    /// Leaves cursor at last row, last column. Minimal CSIs.
    fn fmt(&self, w: &mut impl Write, point: impl Into<Point>) -> io::Result<()> {
        let (x, y) = point.into().into();
        let (width, height) = self.size().into();

        if width != 0 {
            let mut carry = Style::NONE;
            for row in 0..height {
                write!(w, "{}", To(x, y + row))?;
                carry = fmt_row(self, w, width, row, carry)?;
            }
        }

        Ok(())
    }

    /// Writes this [`Layer`](crate::Layer) into `w` at current cursor position.
    ///
    /// Creates new lines, make sure the induced scrolling is under control.
    ///
    /// Leaves cursor at last row, last column. Minimal CSIs.
    fn fmt_at_cursor(&self, w: &mut impl Write) -> io::Result<()> {
        let (width, height) = self.size().into();

        if width != 0 && height != 0 {
            let mut carry = Style::NONE;
            for row in 0..height - 1 {
                carry = fmt_row(self, w, width, row, carry)?;
                write!(w, "\n")?;
            }
            fmt_row(self, w, width, height - 1, carry)?;
        }

        Ok(())
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

        if point.lt(&self.size().into()) {
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
        merge(self, point.into(), above, Cell::above)
    }

    /// Superimposes `below` below `self`.
    fn below(self, point: impl Into<Point>, below: &impl LayerIndex) -> Self {
        merge(self, point.into(), below, Cell::below)
    }
}

/// [`Cell`](crate::Cell) setter, mutably.
pub trait LayerMut: LayerIndex {
    /// Sets the [`Cell`](crate::Cell) at `point`, mutably.
    fn set_mut(&mut self, point: impl Into<Point>, cell: impl Into<Cell>);

    /// Superimposes `above` above `self`, mutably.
    fn above_mut(&mut self, point: impl Into<Point>, above: &impl LayerIndex) {
        merge_mut(self, point.into(), above, Cell::above)
    }

    /// Superimposes `below` below `self`, mutably.
    fn below_mut(&mut self, point: impl Into<Point>, below: &impl LayerIndex) {
        merge_mut(self, point.into(), below, Cell::below)
    }
}

// ======= //
// Helpers //
// ======= //

/// Writes this [`Layer`](crate::Layer)'s `row` into `w`.
///
/// Carries on the printed [`Style`](crate::Style).
fn fmt_row<T>(
    layer: &T,
    w: &mut impl Write,
    width: u16,
    row: u16,
    mut carry: Style,
) -> io::Result<Style>
where
    T: LayerIndex + ?Sized,
{
    for col in 0..width {
        match layer.get_unchecked((col, row)) {
            Cell(Some(Styled { content, style })) => {
                carry = style.dedup(&carry);
                write!(w, "{}{}", carry, content)?;
            }
            _ => write!(w, "{}", Right(1))?,
        }
    }

    Ok(carry)
}

/// Merges `layer` and `other` according to `merge`.
fn merge<T, U, V>(mut layer: T, point: Point, other: &U, merge: V) -> T
where
    T: Layer,
    U: LayerIndex,
    V: Fn(Cell, Cell) -> Cell,
{
    let (x, y) = point.into();
    let (layer_width, layer_height) = layer.size().into();
    let (other_width, other_height) = other.size().into();
    let width = layer_width.min(x + other_width);
    let height = layer_height.min(y + other_height);

    for col in x..width {
        for row in y..height {
            let layer_cell = layer.get_unchecked((col, row));
            let other_cell = other.get_unchecked((col - width, row - height));
            layer = layer.set((col, row), merge(layer_cell, other_cell));
        }
    }

    layer
}

/// Merges `layer` and `other` according to `merge`, mutably.
fn merge_mut<T, U, V>(layer: &mut T, point: Point, other: &U, merge: V)
where
    T: LayerMut + ?Sized,
    U: LayerIndex,
    V: Fn(Cell, Cell) -> Cell,
{
    let (x, y) = point.into();
    let (layer_width, layer_height) = layer.size().into();
    let (other_width, other_height) = other.size().into();
    let width = layer_width.min(x + other_width);
    let height = layer_height.min(y + other_height);

    for col in x..width {
        for row in y..height {
            let layer_cell = layer.get_unchecked((col, row));
            let other_cell = other.get_unchecked((col - width, row - height));
            layer.set_mut((col, row), merge(layer_cell, other_cell));
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
            fn get_unchecked(&self, point: impl Into<Point>) -> Cell {
                <T as LayerIndex>::get_unchecked(self, point)
            }
        })*
    };
    (mut $T:ty) => {
        impl<T: LayerIndexMut> LayerIndexMut for $T {
            fn get_unchecked_mut(&mut self, point: impl Into<Point>) -> &mut Cell {
                <T as LayerIndexMut>::get_unchecked_mut(self, point)
            }
        }
        impl<T: LayerMut> LayerMut for $T {
            fn set_mut(&mut self, point: impl Into<Point>, cell: impl Into<Cell>) {
                <T as LayerMut>::set_mut(self, point, cell)
            }
        }
    };
}

refs!(&T, &mut T);

impl LayerIndex for str {
    fn size(&self) -> Size {
        Size {
            width:  Width(self.len() as u16),
            height: Height(1),
        }
    }

    fn get_unchecked(&self, point: impl Into<Point>) -> Cell {
        self.chars().nth(point.into().x.0 as usize).unwrap().into()
    }
}
