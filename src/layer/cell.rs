use crate::*;
use std::fmt::{Display, Error, Formatter};

/// A terminal [`Cell`](crate::Cell).
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Cell(pub Option<Styled<char>>);

impl Cell {
    /// An empty terminal [`Cell`](crate::Cell).
    pub const NONE: Cell = Cell(None);

    /// Returns a new [`Cell`](crate::Cell).
    ///
    /// Panics in debug if `styled` contains a control `char`.
    pub fn new(styled: Styled<char>) -> Self {
        debug_assert!(!styled.content.is_control(), "Control char");
        Self(Some(styled))
    }

    /// Superimposes `above` above `self`.
    pub fn above(mut self, above: Self) -> Self {
        self.above_mut(above);
        self
    }

    /// Superimposes `below` below `self`.
    pub fn below(self, mut below: Self) -> Self {
        below.above_mut(self);
        below
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
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self(Some(styled)) => Display::fmt(styled, f),
            _ => Ok(()),
        }
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns a new [`Cell`](crate::Cell).
impl From<Styled<char>> for Cell {
    /// Returns a new [`Cell`](crate::Cell).
    fn from(styled: Styled<char>) -> Self {
        Self::new(styled)
    }
}

/// Returns a new [`Cell`](crate::Cell).
impl From<Option<Styled<char>>> for Cell {
    /// Returns a new [`Cell`](crate::Cell).
    fn from(option: Option<Styled<char>>) -> Self {
        match option {
            Some(styled) => Self::new(styled),
            None => Self::NONE,
        }
    }
}

/// Returns a new [`Cell`](crate::Cell).
impl From<Option<Cell>> for Cell {
    /// Returns a new [`Cell`](crate::Cell).
    fn from(cell: Option<Cell>) -> Self {
        match cell {
            Some(cell) => cell,
            None => Self::NONE,
        }
    }
}

// ============ //
// Layer traits //
// ============ //

impl LayerIndex for Cell {
    fn width(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        1
    }

    fn get_unchecked(&self, _: usize, _: usize) -> Cell {
        *self
    }
}

impl LayerIndexMut for Cell {
    fn get_unchecked_mut(&mut self, _: usize, _: usize) -> &mut Cell {
        self
    }
}

impl Layer for Cell {
    fn set(self, _: usize, _: usize, cell: Cell) -> Self {
        cell
    }
}

impl LayerMut for Cell {
    fn set_mut(&mut self, _: usize, _: usize, cell: Cell) {
        *self = cell;
    }
}
