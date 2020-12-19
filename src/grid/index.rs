use super::*;

const POINT_ERROR: &str = "Point outside the grid";
const INDEX_ERROR: &str = "Index out of bounds";

pub trait CellIndex {
    /// Returns the cell or `None` if `point < size`.
    fn get<Cell>(self, grid: GridSlice<Cell>) -> Option<&Cell>;

    /// Returns the cell without any bound checks.
    ///
    /// ### Safety
    ///
    /// The following **MUST** hold:  
    /// - `y * width + x < cells.len()` (panics in debug)
    ///
    /// This can be guaranteed with:  
    /// - `point < size` (panics in debug)
    /// - (`size.area <= cells.len`, guaranteed by [`Grid`](crate::Grid)
    ///   constructors)
    unsafe fn get_unchecked<Cell>(self, grid: GridSlice<Cell>) -> &Cell;

    /// Returns the cell or panics if `point < size`.
    fn index<Cell>(self, grid: GridSlice<Cell>) -> &Cell;
}

impl CellIndex for Point<usize> {
    fn get<Cell>(self, grid: GridSlice<Cell>) -> Option<&Cell> {
        let size = grid.size();

        if self < size.as_point() {
            // SAFETY: point < size
            Some(unsafe { CellIndex::get_unchecked(self, grid) })
        } else {
            None
        }
    }

    unsafe fn get_unchecked<Cell>(self, grid: GridSlice<Cell>) -> &Cell {
        let size = grid.size();
        debug_assert!(self < size.as_point(), POINT_ERROR);

        let index = self.y * size.width + self.x;
        let cells = grid.get_cells();

        debug_assert!(index < cells.len(), INDEX_ERROR);
        cells.get_unchecked(index)
    }

    fn index<Cell>(self, grid: GridSlice<Cell>) -> &Cell {
        CellIndex::get(self, grid).expect(POINT_ERROR)
    }
}

pub trait SafeCellIndex {
    /// Returns the cell or `None` if `point < size`.
    fn get<Cell>(self, grid: GridSlice<Cell>) -> Option<&Cell>;

    /// Returns the cell without size checks.
    ///
    /// Still checks for `[T]` bounds with panicking.
    fn get_unchecked<Cell>(self, grid: GridSlice<Cell>) -> &Cell;

    /// Returns the cell or panics if `point < size`.
    fn index<Cell>(self, grid: GridSlice<Cell>) -> &Cell;
}

impl SafeCellIndex for Point<usize> {
    fn get<Cell>(self, grid: GridSlice<Cell>) -> Option<&Cell> {
        let size = grid.size();

        if self < size.as_point() {
            Some(SafeCellIndex::get_unchecked(self, grid))
        } else {
            None
        }
    }

    fn get_unchecked<Cell>(self, grid: GridSlice<Cell>) -> &Cell {
        let size = grid.size();
        debug_assert!(self < size.as_point(), POINT_ERROR);

        let index = self.y * size.width + self.x;
        let cells = grid.get_cells();

        cells.get(index).expect(INDEX_ERROR)
    }

    fn index<Cell>(self, grid: GridSlice<Cell>) -> &Cell {
        CellIndex::get(self, grid).expect(INDEX_ERROR)
    }
}
