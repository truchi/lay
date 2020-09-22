use super::{Layer, LayerMut};
use crate::{Style, Styled};

/// `Styled<char>`.
///
/// `Cell`s without `Foreground` have transparent foreground.  
/// `Cell`s without `Background` have transparent background.
pub type Cell = Styled<char>;

/// See [`Cell`](type.Cell.html).
impl Cell {
    /// Superimposes `above` above `self`.
    pub fn above(&self, above: &Self) -> Self {
        if above.style.background.is_some() {
            *above
        } else if above.style.foreground.is_some() {
            let mut above = *above;
            above.style.background = self.style.background;

            above
        } else {
            *self
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
