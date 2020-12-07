/// A `(x, y)` / `(width, height)` [`Coord`](crate::Coord).
///
/// Used for points and sizes. See [`Rect`](crate::Rect).
pub type Coord = (u16, u16);

/// The `(0, 0)` [`Coord`](crate::Coord).
pub(crate) const COORD_ZERO: Coord = (0, 0);

/// Conversion to [`Coord`](crate::Coord) trait.
pub trait AsCoord: Sized {
    /// Returns the `x` component.
    fn x(&self) -> u16;

    /// Returns the `y` component.
    fn y(&self) -> u16;

    /// Converts to [`Coord`](crate::Coord).
    #[inline(always)]
    fn as_coord(&self) -> Coord {
        (self.x(), self.y())
    }

    /// Adds two [`Coord`](crate::Coord) together.
    #[inline(always)]
    fn add(&self, other: impl AsCoord) -> Coord {
        (self.x() + other.x(), self.y() + other.y())
    }

    /// Subs (saturating) two [`Coord`](crate::Coord) together.
    #[inline(always)]
    fn sub(&self, other: impl AsCoord) -> Coord {
        (
            self.x().saturating_sub(other.x()),
            self.y().saturating_sub(other.y()),
        )
    }

    /// Lower than comparison on each fields.
    #[inline(always)]
    fn lt(&self, other: impl AsCoord) -> bool {
        self.x() < other.x() && self.y() < other.y()
    }

    /// Returns the minimum on each fields.
    #[inline(always)]
    fn min(&self, other: impl AsCoord) -> Coord {
        (self.x().min(other.x()), self.y().min(other.y()))
    }

    /// Returns the maximun on each fields.
    #[inline(always)]
    fn max(&self, other: impl AsCoord) -> Coord {
        (self.x().max(other.x()), self.y().max(other.y()))
    }
}

// =============== //
// Implementations //
// =============== //

/// ```
/// # use lay::*;
/// assert!(10.as_coord() == (10, 10));
/// ```
impl AsCoord for u16 {
    #[inline(always)]
    fn x(&self) -> u16 {
        *self
    }

    #[inline(always)]
    fn y(&self) -> u16 {
        *self
    }
}

/// ```
/// # use lay::*;
/// assert!((3, 4).as_coord() == (3, 4));
/// ```
impl AsCoord for Coord {
    #[inline(always)]
    fn x(&self) -> u16 {
        self.0
    }

    #[inline(always)]
    fn y(&self) -> u16 {
        self.1
    }
}
