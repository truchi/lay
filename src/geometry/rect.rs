use crate::*;
use std::cmp::Ordering;

/// A `((x, y), (width, height))` [`Rect`](crate::Rect).
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Rect {
    pub point: Point,
    pub size:  Size,
}

// ============== //
// Implementation //
// ============== //

/// ### Consts
impl Rect {
    /// A [`Rect`](crate::Rect) of `((1, 1), (1, 1))`.
    pub const ONE: Rect = Rect {
        point: Point::ONE,
        size:  Size::ONE,
    };
    /// A [`Rect`](crate::Rect) of `((0, 0), (0, 0))`.
    pub const ZERO: Rect = Rect {
        point: Point::ZERO,
        size:  Size::ZERO,
    };
}

/// ### Constructors
impl Rect {
    /// Returns a new [`Rect`](crate::Rect).
    pub fn new(point: Point, size: Size) -> Self {
        Self { point, size }
    }
}

// ====== //
// Traits //
// ====== //

/// Compares both fields simultaneously.
impl PartialOrd for Rect {
    /// Compares both fields simultaneously.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (
            self.point.partial_cmp(&other.point),
            self.size.partial_cmp(&other.size),
        ) {
            (Some(point), Some(size)) if point == size => Some(point),
            _ => None,
        }
    }
}

impl Debug for Rect {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "({:?}, {:?})", self.point, self.size)
    }
}

// =========== //
// Conversions //
// =========== //

/// Returns the [`Rect`](crate::Rect)'s [`Point`](crate::Point).
impl From<Rect> for Point {
    /// Returns the [`Rect`](crate::Rect)'s [`Point`](crate::Point).
    fn from(rect: Rect) -> Self {
        rect.point
    }
}

/// Returns the [`Rect`](crate::Rect)'s [`Size`](crate::Size).
impl From<Rect> for Size {
    /// Returns the [`Rect`](crate::Rect)'s [`Size`](crate::Size).
    fn from(rect: Rect) -> Self {
        rect.size
    }
}

/// Returns the [`Rect`](crate::Rect)'s as `((x, y), (width, height))`.
impl From<Rect> for ((usize, usize), (usize, usize)) {
    /// Returns the [`Rect`](crate::Rect)'s as `((x, y), (width, height))`.
    fn from(rect: Rect) -> Self {
        (
            (rect.point.x.0, rect.point.y.0),
            (rect.size.width.0, rect.size.height.0),
        )
    }
}

/// Returns the [`Rect`](crate::Rect)'s as `(x, y, width, height)`.
impl From<Rect> for (usize, usize, usize, usize) {
    /// Returns the [`Rect`](crate::Rect)'s as `(x, y, width, height)`.
    fn from(rect: Rect) -> Self {
        (
            rect.point.x.0,
            rect.point.y.0,
            rect.size.width.0,
            rect.size.height.0,
        )
    }
}

/// Returns a new `((value, value), (value, value))` [`Rect`](crate::Rect).
impl From<usize> for Rect {
    /// Returns a new `((value, value), (value, value))` [`Rect`](crate::Rect).
    fn from(value: usize) -> Self {
        Self {
            point: Point {
                x: X(value),
                y: Y(value),
            },
            size:  Size {
                width:  Width(value),
                height: Height(value),
            },
        }
    }
}

/// Returns a new `((point, point), (size, size))` [`Rect`](crate::Rect).
impl From<(usize, usize)> for Rect {
    /// Returns a new `((point, point), (size, size))` [`Rect`](crate::Rect).
    fn from((point, size): (usize, usize)) -> Self {
        Self {
            point: Point {
                x: X(point),
                y: Y(point),
            },
            size:  Size {
                width:  Width(size),
                height: Height(size),
            },
        }
    }
}

/// Returns a new `((x, y), (width, height))` [`Rect`](crate::Rect).
impl From<((usize, usize), (usize, usize))> for Rect {
    /// Returns a new `((x, y), (width, height))` [`Rect`](crate::Rect).
    fn from(((x, y), (width, height)): ((usize, usize), (usize, usize))) -> Self {
        Self {
            point: Point { x: X(x), y: Y(y) },
            size:  Size {
                width:  Width(width),
                height: Height(height),
            },
        }
    }
}

/// Returns a new `((x, y), (width, height))` [`Rect`](crate::Rect).
impl From<(usize, usize, usize, usize)> for Rect {
    /// Returns a new `((x, y), (width, height))` [`Rect`](crate::Rect).
    fn from((x, y, width, height): (usize, usize, usize, usize)) -> Self {
        Self {
            point: Point { x: X(x), y: Y(y) },
            size:  Size {
                width:  Width(width),
                height: Height(height),
            },
        }
    }
}

/// Returns a new `(point, (0, 0))` [`Rect`](crate::Rect).
impl From<Point> for Rect {
    /// Returns a new `(point, (0, 0))` [`Rect`](crate::Rect).
    fn from(point: Point) -> Self {
        Self {
            point,
            size: Size::ZERO,
        }
    }
}

/// Returns a new `((0, 0), size)` [`Rect`](crate::Rect).
impl From<Size> for Rect {
    /// Returns a new `((0, 0), size)` [`Rect`](crate::Rect).
    fn from(size: Size) -> Self {
        Self {
            point: Point::ZERO,
            size,
        }
    }
}

/// Returns a new `(point, size)` [`Rect`](crate::Rect).
impl From<(Point, Size)> for Rect {
    /// Returns a new `(point, size)` [`Rect`](crate::Rect).
    fn from((point, size): (Point, Size)) -> Self {
        Self { point, size }
    }
}
