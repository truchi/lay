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
    pub fn new(size: impl Into<Size>, cell: impl Into<Cell>) -> Self {
        let size = size.into();
        let (width, height) = size.into();
        let (width, height): (usize, usize) = (width.into(), height.into());

        let mut cells = Vec::with_capacity(width * height);
        cells.resize(width * height, cell.into());

        Self { size, cells }
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Canvas`](crate::Canvas).
impl<T: Into<Size>, U: Into<Cell>> From<(T, U)> for Canvas {
    /// Returns a new [`Canvas`](crate::Canvas).
    fn from((size, cell): (T, U)) -> Self {
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

    fn get_unchecked(&self, point: impl Into<Point>) -> Cell {
        let (x, y) = point.into().into();
        let (width, _) = self.size.into();

        *self
            .cells
            .get(x as usize + y as usize * width as usize)
            .unwrap()
    }
}

impl LayerIndexMut for Canvas {
    fn get_unchecked_mut(&mut self, point: impl Into<Point>) -> &mut Cell {
        let (x, y) = point.into().into();
        let (width, _) = self.size.into();

        self.cells
            .get_mut(x as usize + y as usize * width as usize)
            .unwrap()
    }
}

impl Layer for Canvas {
    fn set(mut self, point: impl Into<Point>, cell: impl Into<Cell>) -> Self {
        LayerMut::set_mut(&mut self, point, cell);
        self
    }
}

impl LayerMut for Canvas {
    fn set_mut(&mut self, point: impl Into<Point>, cell: impl Into<Cell>) {
        if let Some(c) = LayerIndexMut::get_mut(self, point) {
            *c = cell.into();
        }
    }
}
