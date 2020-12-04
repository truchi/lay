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
}

/// `Display`s the [`Styled`] if some,
/// move the cursor to the right otherwise.
impl Display for Cell {
    /// `Display`s the [`Styled`] if some,
    /// move the cursor to the right otherwise.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self(Some(styled)) => Display::fmt(styled, f),
            _ => Display::fmt(&Right(1), f),
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

impl LayerIndex for Cell {
    fn size(&self) -> Coord {
        (1, 1)
    }

    fn get_unchecked(&self, _: impl AsCoord) -> Cell {
        *self
    }
}

impl LayerIndexMut for Cell {
    fn get_unchecked_mut(&mut self, _: impl AsCoord) -> &mut Cell {
        self
    }
}

impl Layer for Cell {
    fn set(self, _: impl AsCoord, cell: impl Into<Cell>) -> Self {
        cell.into()
    }
}

impl LayerMut for Cell {
    fn set_mut(&mut self, _: impl AsCoord, cell: impl Into<Cell>) {
        *self = cell.into();
    }
}
