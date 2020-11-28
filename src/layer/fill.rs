use crate::*;

/// A plain [`Fill`](crate::Fill).
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Fill {
    pub size: Size,
    pub cell: Cell,
}

/// ### Constructors
impl Fill {
    /// Returns a new [`Fill`](crate::Fill).
    pub fn new<T: Into<Cell>>(size: Size, cell: T) -> Self {
        Self {
            size,
            cell: cell.into(),
        }
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Fill`](crate::Fill).
impl From<(Size, Cell)> for Fill {
    /// Returns a new [`Fill`](crate::Fill).
    fn from((size, cell): (Size, Cell)) -> Self {
        Self::new(size, cell)
    }
}

// ============ //
// Layer traits //
// ============ //

impl LayerIndex for Fill {
    fn size(&self) -> Size {
        self.size
    }

    fn get_unchecked(&self, _: Position) -> Cell {
        self.cell
    }
}
