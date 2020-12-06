use crate::*;

/// A `(point, size)` [`Rect`](crate::Rect).
pub type Rect = (Coord, Coord);

/// The `(COORD_ZERO, COORD_ZERO)` [`Rect`](crate::Rect).
pub(crate) const RECT_ZERO: Rect = (COORD_ZERO, COORD_ZERO);

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

    // TODO remove
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

/// ```
/// # use lay::*;
/// assert!(10.as_rect() == ((10, 10), (10, 10)));
/// ```
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

/// ```
/// # use lay::*;
/// assert!((1, 2).as_rect() == ((1, 1), (2, 2)));
/// assert!((1, (2, 3)).as_rect() == ((1, 1), (2, 3)));
/// assert!(((1, 2), 3).as_rect() == ((1, 2), (3, 3)));
/// assert!(((1, 2), (3, 4)).as_rect() == ((1, 2), (3, 4)));
/// ```
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

/// ```
/// # use lay::*;
/// assert!((1, 2, 3, 4).as_rect() == ((1, 2), (3, 4)));
/// ```
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
