use crate::*;

/// A terminal [`Cell`](crate::Cell).
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Cell(pub Option<Styled<char>>);

impl Cell {
    /// An empty terminal [`Cell`](crate::Cell).
    pub const NONE: Cell = Cell(None);

    /// Returns a new [`Cell`](crate::Cell).
    pub fn new(styled: Styled<char>) -> Self {
        debug_assert!(!styled.content.is_whitespace(), "Char is whitespace");
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
