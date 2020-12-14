use crate::*;
use std::iter::{once, Map, Once, Skip, Take};

/// A terminal [`Cell`](crate::Cell).
///
/// Should only contain a graphical `char`.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Cell(pub Option<Styled<char>>);

/// ### Consts
impl Cell {
    /// An empty terminal [`Cell`](crate::Cell).
    pub const NONE: Cell = Cell(None);
}

/// ### Constructors
impl Cell {
    /// Returns a new [`Cell`](crate::Cell).
    pub fn new(option: Option<impl Into<Styled<char>>>) -> Self {
        match option {
            Some(styled) => Self::from_styled(styled),
            _ => Self::NONE,
        }
    }

    /// Returns a new [`Cell`](crate::Cell).
    pub fn from_styled(styled: impl Into<Styled<char>>) -> Self {
        Self(Some(styled.into()))
    }
}

/// ### Methods
impl Cell {
    /// Superimposes `above` above `self`.
    pub fn above(mut self, above: Self) -> Self {
        self.above_mut(above);
        self
    }

    /// Superimposes `below` below `self`.
    pub fn below(mut self, below: Self) -> Self {
        self.below_mut(below);
        self
    }

    /// Superimposes `above` above `self`, mutably.
    pub fn above_mut(&mut self, above: Self) {
        self.0 = above.0.or(self.0);
    }

    /// Superimposes `below` below `self`, mutably.
    pub fn below_mut(&mut self, mut below: Self) {
        below.above_mut(*self);
        *self = below;
    }

    pub fn get_damage(self, previous: Cell) -> Cell {
        if let Some(previous) = previous.0 {
            if let Some(current) = self.0 {
                if previous.content == current.content {
                    Self::NONE
                } else {
                    self
                }
            } else {
                self
            }
        } else {
            self
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self(Some(styled)) => f
                .debug_tuple("Cell")
                .field(&styled.content)
                .field(&styled.style)
                .finish(),
            _ => f.write_str("NONE"),
        }
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Cell`](crate::Cell).
impl<T: Into<Styled<char>>> From<Option<T>> for Cell {
    /// Returns a new [`Cell`](crate::Cell).
    fn from(option: Option<T>) -> Self {
        Self::new(option)
    }
}

/// Returns a new [`Cell`](crate::Cell).
impl<T: Into<Styled<char>>> From<T> for Cell {
    /// Returns a new [`Cell`](crate::Cell).
    fn from(styled: T) -> Self {
        Self::from_styled(styled)
    }
}

/// Returns a new [`Cell`](crate::Cell).
// TODO useless now
impl From<Option<Cell>> for Cell {
    /// Returns a new [`Cell`](crate::Cell).
    fn from(cell: Option<Cell>) -> Self {
        match cell {
            Some(cell) => cell.into(),
            None => Self::NONE,
        }
    }
}

impl AsRef<Option<Styled<char>>> for Cell {
    fn as_ref(&self) -> &Option<Styled<char>> {
        &self.0
    }
}

impl AsMut<Option<Styled<char>>> for Cell {
    fn as_mut(&mut self) -> &mut Option<Styled<char>> {
        &mut self.0
    }
}

// ============ //
// Layer traits //
// ============ //

impl LayerSize for Cell {
    fn size(&self) -> Coord {
        (1, 1)
    }
}

impl<'a> Layer<'a> for Cell {
    type Cells = Take<Skip<Once<Cell>>>;
    type Row = Take<Skip<Once<Cell>>>;
    type Rows = Map<Take<Skip<Once<(Cell, u16, u16)>>>, fn((Cell, u16, u16)) -> Self::Row>;

    fn cropped_row(&'a self, row: u16, col: u16, len: u16) -> Self::Row {
        once(*self)
            .skip(row as usize + col as usize)
            .take(len as usize)
    }

    fn cropped_rows(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Rows {
        once((*self, col, width))
            .skip(row as usize)
            .take(height as usize)
            .map(|(cell, col, width)| Layer::cropped_row(&cell, 0, col, width))
    }

    fn cropped_cells(&'a self, col: u16, row: u16, width: u16, height: u16) -> Self::Cells {
        once(*self)
            .skip(row as usize + col as usize)
            .take(width as usize * height as usize)
    }
}

impl<'a> LayerMut<'a> for Cell {
    type CellsMut = Take<Skip<Once<&'a mut Cell>>>;
    type RowMut = Take<Skip<Once<&'a mut Cell>>>;
    type RowsMut = Map<
        Take<Skip<Once<(&'a mut Cell, u16, u16)>>>,
        fn((&'a mut Cell, u16, u16)) -> Self::RowMut,
    >;

    fn cropped_row_mut(&'a mut self, row: u16, col: u16, len: u16) -> Self::RowMut {
        once(self)
            .skip(row as usize + col as usize)
            .take(len as usize)
    }

    fn cropped_rows_mut(
        &'a mut self,
        col: u16,
        row: u16,
        width: u16,
        height: u16,
    ) -> Self::RowsMut {
        once((self, col, width))
            .skip(row as usize)
            .take(height as usize)
            .map(|(cell, col, width)| LayerMut::cropped_row_mut(cell, 0, col, width))
    }

    fn cropped_cells_mut(
        &'a mut self,
        col: u16,
        row: u16,
        width: u16,
        height: u16,
    ) -> Self::CellsMut {
        once(self)
            .skip(row as usize + col as usize)
            .take(width as usize * height as usize)
    }
}

// ================================ //

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct DamagedCell {
    pub current: Cell,
    previous:    Cell,
}

impl DamagedCell {
    pub fn new(current: impl Into<Cell>) -> Self {
        Self {
            current:  current.into(),
            previous: Cell::NONE,
        }
    }
}

impl<T: Into<Cell>> From<T> for DamagedCell {
    fn from(current: T) -> Self {
        Self::new(current)
    }
}

// ================================ //

pub trait DamageCell {
    fn get_cell(&self) -> Cell;
    fn get_damage(&self) -> Cell;

    fn save_mut(&mut self);

    fn save(mut self) -> Self
    where
        Self: Sized,
    {
        self.save_mut();
        self
    }
}

impl DamageCell for Cell {
    fn get_cell(&self) -> Self {
        *self
    }

    fn get_damage(&self) -> Self {
        *self
    }

    fn save_mut(&mut self) {}
}

impl DamageCell for DamagedCell {
    fn get_cell(&self) -> Cell {
        self.current
    }

    fn get_damage(&self) -> Cell {
        self.current.get_damage(self.previous)
    }

    fn save_mut(&mut self) {
        self.previous = self.current;
    }
}
