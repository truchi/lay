use crate::*;

/// A plain [`Fill`](crate::Fill).
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Fill {
    pub size: Coord,
    pub cell: Cell,
}

/// ### Constructors
impl Fill {
    /// Returns a new [`Fill`](crate::Fill).
    pub fn new(size: impl AsCoord, cell: impl Into<Cell>) -> Self {
        Self {
            size: size.as_coord(),
            cell: cell.into(),
        }
    }
}

/// `Display`s the [`Fill`](crate::Fill).
impl Display for Fill {
    /// `Display`s the [`Fill`](crate::Fill).
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_at_cursor(f)
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Fill`](crate::Fill).
impl<T: AsCoord, U: Into<Cell>> From<(T, U)> for Fill {
    /// Returns a new [`Fill`](crate::Fill).
    fn from((size, cell): (T, U)) -> Self {
        Self::new(size, cell)
    }
}

// ============ //
// Layer traits //
// ============ //

impl LayerIndex for Fill {
    fn size(&self) -> Coord {
        self.size
    }

    fn get_unchecked(&self, _: impl AsCoord) -> Cell {
        self.cell
    }
}
