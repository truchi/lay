//! # Geometry utilities

/// A `(x, y)` / `(width, height)` [`Coord`](crate::Coord).
///
/// Used for points and sizes. See [`Rect`](crate::Rect).
pub type Coord = (u16, u16);

/// A `(point, size)` [`Rect`](crate::Rect).
pub type Rect = (Coord, Coord);

/// The `(0, 0)` [`Coord`](crate::Coord).
pub(crate) const COORD_ZERO: Coord = (0, 0);

/// The `(COORD_ZERO, COORD_ZERO)` [`Rect`](crate::Rect).
pub(crate) const RECT_ZERO: Rect = (COORD_ZERO, COORD_ZERO);

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

    /// Lower than comparison on both fields.
    #[inline(always)]
    fn lt(&self, other: impl AsCoord) -> bool {
        self.x() < other.x() && self.y() < other.y()
    }
}

/// Conversion to [`Rect`](crate::Rect) trait.
pub trait AsRect: Sized {
    /// Returns the `point` component.
    fn point(&self) -> Coord;

    /// Returns the `size` component.
    fn size(&self) -> Coord;

    /// Returns the `x` component.
    #[inline(always)]
    fn x(&self) -> u16 {
        self.point().0
    }

    /// Returns the `y` component.
    #[inline(always)]
    fn y(&self) -> u16 {
        self.point().1
    }

    /// Returns the `width` component.
    #[inline(always)]
    fn width(&self) -> u16 {
        self.size().0
    }

    /// Returns the `height` component.
    #[inline(always)]
    fn height(&self) -> u16 {
        self.size().1
    }

    /// Converts to [`Rect`](crate::Rect).
    #[inline(always)]
    fn as_rect(&self) -> Rect {
        (self.point(), self.size())
    }

    /// Crops a [`Rect`](crate::Rect)'s `size` so that `rect` does not have
    /// surface beyond `to`.
    #[inline(always)]
    fn crop(&self, to: impl AsCoord) -> Rect {
        (
            self.point(),
            (
                clamp_into(self.width(), self.x(), to.x()),
                clamp_into(self.height(), self.y(), to.y()),
            ),
        )
    }
}

// =============== //
// Implementations //
// =============== //

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

impl AsCoord for Coord {
    #[inline(always)]
    fn x(&self) -> u16 {
        self.0
    }

    #[inline(always)]
    fn y(&self) -> u16 {
        self.0
    }
}

impl AsRect for u16 {
    #[inline(always)]
    fn point(&self) -> Coord {
        (*self, *self)
    }

    #[inline(always)]
    fn size(&self) -> Coord {
        (*self, *self)
    }
}

impl<T: AsCoord, U: AsCoord> AsRect for (T, U) {
    #[inline(always)]
    fn point(&self) -> Coord {
        self.0.as_coord()
    }

    #[inline(always)]
    fn size(&self) -> Coord {
        self.1.as_coord()
    }
}

impl AsRect for (u16, u16, u16, u16) {
    #[inline(always)]
    fn point(&self) -> Coord {
        (self.0, self.1)
    }

    #[inline(always)]
    fn size(&self) -> Coord {
        (self.2, self.3)
    }
}

// ======= //
// Helpers //
// ======= //

/// Clamps `u` into `[from, to]`, and sets `from` as the new `0`.
#[inline(always)]
fn clamp_into(u: u16, from: u16, to: u16) -> u16 {
    if to <= from {
        0
    } else {
        u.min(to - from)
    }
}
