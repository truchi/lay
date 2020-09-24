use super::{Layer, LayerMut};
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

impl Layer for Cell {
    fn width(&self) -> u16 {
        1
    }

    fn height(&self) -> u16 {
        1
    }

    fn get_unchecked(&self, _: u16, _: u16) -> Cell {
        *self
    }
}

impl LayerMut for Cell {
    fn get_mut_unchecked(&mut self, _: u16, _: u16) -> &mut Cell {
        self
    }
}

impl_layer_mut_ops!(Cell);
