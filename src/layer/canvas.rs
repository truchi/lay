use crate::*;

/// A [`Canvas`](crate::Canvas).
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Canvas {
    size:  Size,
    cells: Vec<Cell>,
}

/// ### Constructors
impl Canvas {
    /// Returns a new [`Canvas`](crate::Canvas).
    pub fn new<T: Into<Cell>>((width, height): Size, cell: T) -> Self {
        let len = width * height;
        let mut cells = Vec::with_capacity(len);
        cells.resize(len, cell.into());

        Self {
            size: (width, height),
            cells,
        }
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Canvas`](crate::Canvas).
impl<T: Into<Cell>> From<(Size, T)> for Canvas {
    /// Returns a new [`Canvas`](crate::Canvas).
    fn from((size, cell): (Size, T)) -> Self {
        Self::new(size, cell)
    }
}

// ============ //
// Layer traits //
// ============ //

impl LayerIndex for Canvas {
    fn size(&self) -> Size {
        self.size
    }

    fn get_unchecked(&self, (x, y): Position) -> Cell {
        let (width, _) = self.size;
        *self.cells.get(x + y * width).unwrap()
    }
}

impl LayerIndexMut for Canvas {
    fn get_unchecked_mut(&mut self, (x, y): Position) -> &mut Cell {
        let (width, _) = self.size;
        self.cells.get_mut(x + y * width).unwrap()
    }
}

impl Layer for Canvas {
    fn set(mut self, position: Position, cell: Cell) -> Self {
        LayerMut::set_mut(&mut self, position, cell);
        self
    }
}

impl LayerMut for Canvas {
    fn set_mut(&mut self, position: Position, cell: Cell) {
        if let Some(c) = LayerIndexMut::get_mut(self, position) {
            *c = cell;
        }
    }
}
