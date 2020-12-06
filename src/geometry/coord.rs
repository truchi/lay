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

    //   #[inline(always)]
    //   fn transform(&self, transform: impl Transform) -> Rect {
    //       let point = self.min(transform.point());
    //       let size = coord_clamp_into(&point, &transform.resize(self.as_coord()),
    // self);
    //
    //       (point, size)
    //   }
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

// ======= //
// Helpers //
// ======= //

/// Gives `size` satisfying `point + size <= max_size`.
#[inline(always)]
fn _clamp_into(point: u16, size: u16, max_size: u16) -> u16 {
    if point + size <= max_size {
        size
    } else {
        max_size.saturating_sub(point)
    }
}

/// `clamp_into` on both fields.
#[inline(always)]
fn _coord_clamp_into(point: &impl AsCoord, size: &impl AsCoord, max_size: &impl AsCoord) -> Coord {
    (
        _clamp_into(point.x(), size.x(), max_size.x()),
        _clamp_into(point.y(), size.y(), max_size.y()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn clamp_into() {
        let fun = super::clamp_into;

        // point > max_size
        assert_eq!(fun(0, 0, 0), 0, "clamp_into(0, 0, 0)");
        assert_eq!(fun(27, 8, 0), 0, "super::clamp_into(27, 8, 0)");

        // point == max_size
        assert_eq!(fun(32, 0, 32), 0, "clamp_into(32, 0, 32)");
        assert_eq!(fun(32, 10, 32), 0, "clamp_into(32, 10, 32)");

        // point < max_size
        assert_eq!(fun(18, 0, 20), 0, "clamp_into(18, 0, 20)");
        assert_eq!(fun(18, 1, 20), 1, "clamp_into(18, 1, 20)");
        assert_eq!(fun(18, 2, 20), 2, "clamp_into(18, 2, 20)");
        assert_eq!(fun(18, 3, 20), 2, "clamp_into(18, 3, 20)");
    }
}
