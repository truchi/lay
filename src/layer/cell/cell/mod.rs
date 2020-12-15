mod layer;

use crate::*;

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

impl AsDamagedCell for Cell {
    fn get_cell(&self) -> Self {
        *self
    }

    fn get_damage(&self) -> Self {
        *self
    }

    fn save_mut(&mut self) {}
}
