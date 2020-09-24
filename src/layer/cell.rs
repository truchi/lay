use crate::{Styled, Styler};

/// `Styled<char>`.
///
/// When merging, `char::default()` (`'\u{0}'`, NUL char) denotes transparency,
/// though it will render as `' '` in layers.
pub type Cell = Styled<char>;

/// See [`Cell`](type.Cell.html).
impl Cell {
    /// Superimposes `above` above `self`.
    pub fn above(mut self, above: Self) -> Self {
        self.above_mut(above);
        self
    }

    /// Superimposes `above` above `self`.
    pub fn above_mut(&mut self, above: Self) {
        if above.content != char::default() {
            *self = above.or(self);
        }
    }

    /// Superimposes `below` below `self`.
    pub fn below(self, mut below: Self) -> Self {
        below.above_mut(self);
        below
    }

    /// Superimposes `below` below `self`.
    pub fn below_mut(&mut self, mut below: Self) {
        below.above_mut(*self);
        *self = below;
    }
}

impl_layer!(Cell [cell, x, y] {
    Layer { 1 } { 1 } { *cell }
    Index { cell }
    LayerMut { cell }
    IndexMut { cell }
    + Ops
});
