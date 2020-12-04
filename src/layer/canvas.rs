use crate::*;

/// A [`Canvas`](crate::Canvas).
#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Canvas {
    size:  Coord,
    cells: Vec<Cell>,
}

/// ### Constructors
impl Canvas {
    /// Returns a new [`Canvas`](crate::Canvas).
    pub fn new(size: impl AsCoord, cell: impl Into<Cell>) -> Self {
        let size = size.as_coord();
        let (width, height) = size;
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
impl<T: AsCoord, U: Into<Cell>> From<(T, U)> for Canvas {
    /// Returns a new [`Canvas`](crate::Canvas).
    fn from((size, cell): (T, U)) -> Self {
        Self::new(size, cell)
    }
}

// ============ //
// Layer traits //
// ============ //

impl LayerIndex for Canvas {
    fn size(&self) -> Coord {
        self.size
    }

    fn get_unchecked(&self, point: impl AsCoord) -> Cell {
        let (x, y) = point.as_coord();
        let (width, _) = self.size;

        match self.cells.get(x as usize + y as usize * width as usize) {
            Some(cell) => *cell,
            _ => Cell::NONE,
        }
    }
}

impl LayerIndexMut for Canvas {
    fn get_unchecked_mut(&mut self, point: impl AsCoord) -> &mut Cell {
        let (x, y) = point.as_coord();
        let (width, _) = self.size;

        self.cells
            .get_mut(x as usize + y as usize * width as usize)
            .expect("Out of bounds access")
    }
}

impl Layer for Canvas {
    fn set_mut(&mut self, point: impl AsCoord, cell: impl Into<Cell>) {
        if let Some(c) = LayerIndexMut::get_mut(self, point) {
            *c = cell.into();
        }
    }
}
