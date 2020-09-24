use crate::{Styled, Styler};

/// `Styled<char>`.
///
/// When merging, `char::default()` (`'\u{0}'`, NUL char) denotes transparency,
/// though it will render as `' '` in layers.
pub type Cell = Styled<char>;

/// See [`Cell`](type.Cell.html).
impl Cell {
    /// Superimposes `above` above `self`.
    pub fn above(&self, above: &Self) -> Self {
        if above.content == char::default() {
            *self
        } else {
            above.or(self)
        }
    }

    /// Superimposes `below` below `self`.
    pub fn below(&self, below: &Self) -> Self {
        below.above(self)
    }
}

impl_layer!(Cell [cell, x, y] {
    Layer { 1 } { 1 } { *cell }
    Index { cell }
    LayerMut { cell }
    IndexMut { cell }
});

impl_layer_mut_ops!(Cell);
