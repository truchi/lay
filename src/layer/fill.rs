use crate::*;

/// A plain [`Fill`](crate::Fill).
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Fill {
    pub width:  usize,
    pub height: usize,
    pub cell:   Cell,
}

/// ### Constructors
impl Fill {
    /// Returns a new [`Fill`](crate::Fill).
    pub fn new<T: Into<Cell>>(width: usize, height: usize, cell: T) -> Self {
        Self {
            width,
            height,
            cell: cell.into(),
        }
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Fill`](crate::Fill).
impl From<(usize, usize, Cell)> for Fill {
    /// Returns a new [`Fill`](crate::Fill).
    fn from((width, height, cell): (usize, usize, Cell)) -> Self {
        Self::new(width, height, cell)
    }
}

// ============ //
// Layer traits //
// ============ //

impl LayerIndex for Fill {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn get_unchecked(&self, _: usize, _: usize) -> Cell {
        self.cell
    }
}
