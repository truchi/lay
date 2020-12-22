use super::*;
use std::{convert::TryFrom, marker::PhantomData};

/*
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
*/

/// Error type for [`Grid`](crate::Grid) constructors.
#[derive(Copy, Clone, Debug)]
pub enum GridError<T> {
    /// `width * height > usize::MAX`.
    Overflow(Size<usize>, T),
    /// `width * height != len`.
    Mismatch(Size<usize>, T),
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

// impl<Major, Cell, Collection: AsRef<[Cell]>> IGrid for Grid<Major, Cell,
// Collection> { type Cell = Cell;
//
// fn size(&self) -> Size<usize> {
// self.size
// }
//
// fn cell(&self, point: Point<usize>) -> Option<&Cell> {
// let index = RowMajor::cell(self.size, point)?;
//
// TODO unsafe
// self.cells.as_ref().get(index)
// }
// }
