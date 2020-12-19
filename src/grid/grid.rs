use super::*;
use std::{convert::TryFrom, marker::PhantomData};

pub enum RowMajor {}
pub enum ColumnMajor {}

impl RowMajor {
    pub fn index<P: Into<Point<usize>>, S: Into<Size<usize>>>(point: P, size: S) -> Option<usize> {
        let point = point.into();
        let size = size.into();

        if point < size.as_point() {
            Some(Self::index_unchecked(point, size))
        } else {
            None
        }
    }

    pub fn index_unchecked<P: Into<Point<usize>>, S: Into<Size<usize>>>(
        point: P,
        size: S,
    ) -> usize {
        let (x, y) = point.into().into();
        let (width, _) = size.into().into();

        y * width + x
    }
}

/// Error type for [`Grid`](crate::Grid) constructors.
#[derive(Copy, Clone, Debug)]
pub enum GridError<T> {
    /// `width * height > usize::MAX`.
    Overflow(Size<usize>, T),
    /// `width * height != len`.
    Mismatch(Size<usize>, T),
}

pub(crate) type GridSlice<'a, Cell> = Grid<(), Cell, &'a [Cell]>;
pub(crate) type GridSliceMut<'a, Cell> = Grid<(), Cell, &'a mut [Cell]>;

impl<'a, Cell> GridSlice<'a, Cell> {
    pub(crate) fn get_cells(self) -> &'a [Cell] {
        self.cells
    }
}

impl<'a, Major, Cell, Collection: AsRef<[Cell]>> From<&'a Grid<Major, Cell, Collection>>
    for GridSlice<'a, Cell>
{
    fn from(grid: &'a Grid<Major, Cell, Collection>) -> Self {
        Self {
            size:    grid.size,
            cells:   grid.cells.as_ref(),
            phantom: PhantomData,
        }
    }
}

impl<'a, Major, Cell, Collection: AsMut<[Cell]>> From<&'a mut Grid<Major, Cell, Collection>>
    for GridSliceMut<'a, Cell>
{
    fn from(grid: &'a mut Grid<Major, Cell, Collection>) -> Self {
        Self {
            size:    grid.size,
            cells:   grid.cells.as_mut(),
            phantom: PhantomData,
        }
    }
}

/// 2D [`Grid`](crate::Grid).
///
/// RowMajor:      ColumnMajor:
/// - cell         - cell
/// - row          - col
/// - row_iter     - col_iter
/// - col_iter     - row_iter
/// - cells_iter   - cells_iter
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Grid<Major, Cell, Collection: ?Sized> {
    phantom: PhantomData<(Major, Cell)>,
    size:    Size<usize>,
    cells:   Collection,
}

/// ### Constructors
impl<Major, Cell, Collection: AsRef<[Cell]>> Grid<Major, Cell, Collection> {
    /// Creates a new [`Grid`](crate::Grid)
    /// or returns a [`GridError`](GridError).
    pub fn new<S: Into<Size<usize>>>(
        size: S,
        cells: Collection,
    ) -> Result<Self, GridError<Collection>> {
        let size = size.into();

        match size.checked_area() {
            None => Err(GridError::Overflow(size, cells)),
            Some(area) =>
                if area != cells.as_ref().len() {
                    Err(GridError::Mismatch(size, cells))
                } else {
                    Ok(Self {
                        size,
                        cells,
                        phantom: PhantomData,
                    })
                },
        }
    }
}

/// ### Methods
impl<Major, Cell, Collection: AsRef<[Cell]>> Grid<Major, Cell, Collection> {
    /// Returns the [`Size`](crate::Size).
    pub fn size(&self) -> Size<usize> {
        self.size
    }
}

impl<Major, Cell, Collection: AsRef<[Cell]>, S: Into<Size<usize>>> TryFrom<(S, Collection)>
    for Grid<Major, Cell, Collection>
{
    type Error = GridError<Collection>;

    fn try_from((size, cells): (S, Collection)) -> Result<Self, Self::Error> {
        Self::new(size, cells)
    }
}

impl<Major, Cell, Collection: AsRef<[Cell]>> AsRef<[Cell]> for Grid<Major, Cell, Collection> {
    fn as_ref(&self) -> &[Cell] {
        self.cells.as_ref()
    }
}

impl<Major, Cell, Collection: AsMut<[Cell]>> AsMut<[Cell]> for Grid<Major, Cell, Collection> {
    fn as_mut(&mut self) -> &mut [Cell] {
        self.cells.as_mut()
    }
}

// ========================================== //
// ========================================== //
// ========================================== //
// ========================================== //

pub trait GridSize {
    fn size(&self) -> Size<usize>;
}

pub trait GridCell {
    type Cell;
    fn cell<P: Into<Point<usize>>>(&self, point: P) -> Option<&Self::Cell>;
}

// ========================================== //
// ========================================== //
// ========================================== //
// ========================================== //

impl<Major, Cell, Collection: AsRef<[Cell]>> GridSize for Grid<Major, Cell, Collection> {
    fn size(&self) -> Size<usize> {
        self.size
    }
}

impl<Cell, Collection: AsRef<[Cell]>> GridCell for Grid<RowMajor, Cell, Collection> {
    type Cell = Cell;

    fn cell<P: Into<Point<usize>>>(&self, point: P) -> Option<&Cell> {
        // index.get(self.into())
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cell() {
        let grid = vec![
            1, 2, 3, //
            4, 5, 6, //
            7, 8, 9, //
        ];
        let grid = Grid::<(), _, _>::new(3, grid).unwrap();

        assert_eq!(grid.cell(Point::from(0)), Some(&1));
        assert_eq!(grid.cell(Point::from(1)), Some(&5));
        assert_eq!(grid.cell(Point::from(2)), Some(&9));
        assert_eq!(grid.cell(Point::from(3)), None);
    }
}
