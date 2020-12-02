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
    pub fn new(size: impl Into<Size>, cell: impl Into<Cell>) -> Self {
        Self {
            size: size.into(),
            cell: cell.into(),
        }
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Fill`](crate::Fill).
impl<T: Into<Size>, U: Into<Cell>> From<(T, U)> for Fill {
    /// Returns a new [`Fill`](crate::Fill).
    fn from((size, cell): (T, U)) -> Self {
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

    fn get_unchecked(&self, _: impl Into<Point>) -> Cell {
        self.cell
    }
}
