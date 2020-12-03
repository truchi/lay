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

    /// Merges `self` and `other` with the `merge` function.
    ///
    /// Assumes `other` is contained in `self`. May panic otherwise.
    fn merge_unchecked(self, other: &impl LayerIndex, merge: impl Fn(Cell, Cell) -> Cell) -> Self {
        merger_unchecked(self, other, merge)
    }

    /// Merges `self` and `other` with the `merge` function.
    fn merge(self, other: &impl LayerIndex, merge: impl Fn(Cell, Cell) -> Cell) -> Self {
        merger(self, other, merge)
    }

    /// Superimposes `above` above `self`.
    ///
    /// Assumes `above` is contained in `self`. May panic otherwise.
    fn above_unchecked(self, above: &impl LayerIndex) -> Self {
        merger_unchecked(self, above, Cell::above)
    }

    /// Superimposes `above` above `self`.
    fn above(self, above: &impl LayerIndex) -> Self {
        merger(self, above, Cell::above)
    }

    /// Superimposes `below` below `self`.
    ///
    /// Assumes `below` is contained in `self`. May panic otherwise.
    fn below_unchecked(self, below: &impl LayerIndex) -> Self {
        merger_unchecked(self, below, Cell::below)
    }

    /// Superimposes `below` below `self`.
    fn below(self, below: &impl LayerIndex) -> Self {
        merger(self, below, Cell::below)
    }

    /// Fills this [`Layer`](crate::Layer) with `cell`.
    fn fill(self, cell: impl Into<Cell>) -> Self {
        fill(self, cell.into())
    }

    /// Clears this [`Layer`](crate::Layer) with
    /// [`Cell::NONE`](crate::Cell::NONE).
    fn clear(self) -> Self {
        self.fill(Cell::NONE)
    }
}

/// [`Cell`](crate::Cell) setter, mutably.
pub trait LayerMut: LayerIndex {
    /// Sets the [`Cell`](crate::Cell) at `point`, mutably.
    fn set_mut(&mut self, point: impl Into<Point>, cell: impl Into<Cell>);

    /// Merges `self` and `other` with the `merge` function, mutably.
    ///
    /// Assumes `other` is contained in `self`. May panic otherwise.
    fn merge_unchecked_mut(&mut self, other: &impl LayerIndex, merge: impl Fn(Cell, Cell) -> Cell) {
        merger_unchecked_mut(self, other, merge)
    }

    /// Merges `self` and `other` with the `merge` function, mutably.
    fn merge_mut(&mut self, other: &impl LayerIndex, merge: impl Fn(Cell, Cell) -> Cell) {
        merger_mut(self, other, merge)
    }

    /// Superimposes `above` above `self`, mutably.
    ///
    /// Assumes `above` is contained in `self`. May panic otherwise.
    fn above_unchecked_mut(&mut self, above: &impl LayerIndex) {
        merger_unchecked_mut(self, above, Cell::above)
    }

    /// Superimposes `above` above `self`, mutably.
    fn above_mut(&mut self, above: &impl LayerIndex) {
        merger_mut(self, above, Cell::above)
    }

    /// Superimposes `below` below `self`, mutably.
    ///
    /// Assumes `below` is contained in `self`. May panic otherwise.
    fn below_unchecked_mut(&mut self, below: &impl LayerIndex) {
        merger_unchecked_mut(self, below, Cell::below)
    }

    /// Superimposes `below` below `self`, mutably.
    fn below_mut(&mut self, below: &impl LayerIndex) {
        merger_mut(self, below, Cell::below)
    }

    /// Fills this [`Layer`](crate::Layer) with `cell`, mutably.
    fn fill_mut(&mut self, cell: impl Into<Cell>) {
        fill_mut(self, cell.into())
    }

    /// Clears this [`Layer`](crate::Layer) with
    /// [`Cell::NONE`](crate::Cell::NONE), mutably.
    fn clear_mut(&mut self) {
        self.fill_mut(Cell::NONE)
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

macro_rules! merge {
    ($fn:ident $fn_mut:ident $fn_un:ident $fn_un_mut:ident) => {
        merge!(impl l o
            fn $fn       (l:     (Layer))        -> T { (if let Some(o)) get () set }
            fn $fn_mut   ( : mut (LayerMut + ?Sized)) { (if let Some(o)) get () set_mut }
            fn $fn_un    (l:     (Layer))        -> T { (let o) get_unchecked (;) set }
            fn $fn_un_mut( : mut (LayerMut + ?Sized)) { (let o) get_unchecked (;) set_mut }
        );
    };
    (impl $layer:ident $other_cell:ident $(
        fn $fn:ident($($layer2:ident)?: $($mut:ident)? ($($T:tt)*))
        $(-> $ret:ident)? {
            ($($get_left_side:tt)*)
            $get:ident
            ($($get_right_side:tt)*)
            $set:ident
        }
    )* ) => { $(
        /// Merges `layer` and `other` with the `merge` function.
        fn $fn<T, U>($layer: $(&$mut)? T, other: &impl LayerIndex, merge: U) $(-> $ret)?
        where
            T: $($T)*,
            U: Fn(Cell, Cell) -> Cell
        {
            $(let mut $layer2 = $layer;)?
            let (width, height) = $layer.size().into();

            for x in 0..width {
                for y in 0..height {
                    let layer_cell = $layer.get_unchecked((x, y));

                    $($get_left_side)* = other.$get((x, y)) $($get_right_side)*
                    {
                        let cell = merge(layer_cell, $other_cell);
                        $($layer2 =)? $layer.$set((x, y), cell);
                    }
                }
            }

            $($layer2)?
        }
    )* };
}

merge!(merger merger_mut merger_unchecked merger_unchecked_mut);

/// Fills this [`Layer`](crate::Layer) with `cell`.
fn fill<T>(mut layer: T, cell: Cell) -> T
where
    T: Layer,
{
    let (width, height) = layer.size().into();

    if width != 0 {
        for row in 0..height {
            for col in 0..width {
                layer = layer.set((col, row), cell);
            }
        }
    }

    layer
}

/// Fills this [`Layer`](crate::Layer) with `cell`, matably.
fn fill_mut<T>(layer: &mut T, cell: Cell)
where
    T: LayerMut + ?Sized,
{
    let (width, height) = layer.size().into();

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
