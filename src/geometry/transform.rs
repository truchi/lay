use crate::*;

/// [`Transform`](crate::Transform)ations (translate + resize).
///
/// Segment/[`Rect`](crate::Rect) transformation with bound checks.
///
/// - `Transform<u16>`:
///   - `(origin, size)` segment -> `(new_point, new_size)` segment,
///   - Used to crop a row of a [`Layer`](crate::Layer).
/// - `Transform<Coord>`:
///   - `(origin, size)` [`Rect`](crate::Rect) -> `(new_point, new_size)`
///     [`Rect`](crate::Rect),
///   - Used to crop a whole [`Layer`](crate::Layer).
///
/// Guaranties on the returned view:
/// - `new_point <= size`
/// - `new_point + new_size <= size`
///
/// In the form:
/// - `()`: neither translates nor resizes,
/// - `T`: resizes only,
/// - `(T, U)`: translates and resizes.
///
/// See also [`View`](crate::View).
pub trait Transform<T> {
    /// Transforms a `(origin, size)` segment/[`Rect`](crate::Rect).
    fn transform(self, size: T) -> (T, T);
}

macro_rules! transform {
    ($self:ident $([
        <$T:ty> $(
            $(#[$meta:meta])+
            $({$($bounds:tt)*})? $Self:ty =>
            ($size:pat) $transform:expr
        )*
    ])*) => { $( $(
        $(#[$meta])+
        impl$(<$($bounds)*>)? Transform<$T> for $Self {
            #[inline(always)]
            fn transform($self, $size: $T) -> ($T, $T) { $transform }
        }
    )* )* };
}

transform!(self [
<u16>
    /// Neither translates nor resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(().transform(5) == (0, 5));
    /// ```
    () =>
        (size) (0, size)

    // TODO is resize the default, really?

    /// Resizes only.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(1.transform(2) == (0, 1)); // Cropped
    /// assert!(2.transform(2) == (0, 2)); // Full
    /// assert!(3.transform(2) == (0, 2)); // Full
    /// ```
    u16 =>
        (size) (0, self.min(size))

    /// Resizes only.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((|size| size - 1).transform(4) == (0, 3)); // Cropped
    /// assert!((|size| size + 1).transform(4) == (0, 4)); // Full
    /// ```
    {F: FnOnce(u16) -> u16} F =>
        (size) self(size).transform(size)

    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1, 2).transform(4) == (1, 2)); // Cropped
    /// assert!((1, 4).transform(4) == (1, 3)); // Cropped start
    /// assert!((9, 9).transform(4) == (4, 0)); // Empty
    /// ```
    (u16, u16) =>
        (size) {
            let point = self.0.min(size); // FIXME size - 1?
            let size = clamp_into(point, self.1, size);

            (point, size)
        }

    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1, |size| size / 2).transform(4) == (1, 2)); // Cropped
    /// assert!((1, |size| size * 2).transform(4) == (1, 3)); // Cropped start
    /// assert!((9, |size| size * 2).transform(4) == (4, 0)); // Empty
    /// ```
    {F: FnOnce(u16) -> u16} (u16, F) =>
        (size) (self.0, self.1(size)).transform(size)
] [
<Coord>
    /// Neither translates nor resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(().transform((5, 6)) == ((0, 0), (5, 6)));
    /// ```
    () =>
        (size) (COORD_ZERO, size)

    /// Resizes only.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(1.transform((2, 2)) == ((0, 0), (1, 1))); // Cropped
    /// assert!(2.transform((2, 2)) == ((0, 0), (2, 2))); // Full
    /// assert!(3.transform((2, 2)) == ((0, 0), (2, 2))); // Full
    /// ```
    u16 =>
        (size) from_segments((self.transform(size.0), self.transform(size.1)))

    /// Resizes only.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((|size: Coord| size.sub(1)).transform((4, 6)) == ((0, 0), (3, 5))); // Cropped
    /// assert!((|size: Coord| size       ).transform((4, 6)) == ((0, 0), (4, 6))); // Full
    /// assert!((|size: Coord| size.add(1)).transform((4, 6)) == ((0, 0), (4, 6))); // Full
    /// ```
    {F: FnOnce(Coord) -> Coord} F =>
        (size) {
            let (width, height) = self(size);
            from_segments((width.transform(size.0), height.transform(size.1)))
        }

    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(((1, 2), 3).transform((9, 9)) == ((1, 2), (3, 3))); // Cropped
    /// assert!(((1, 2), 9).transform((9, 9)) == ((1, 2), (8, 7))); // Cropped start
    /// assert!(((9, 9), 9).transform((9, 9)) == ((9, 9), (0, 0))); // Empty
    /// ```
    {T: AsCoord} (T, u16) =>
        (size) (self.0, self.1.as_coord()).transform(size)

    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(((1, 2), (3, 4)).transform((9, 9)) == ((1, 2), (3, 4))); // Cropped
    /// assert!(((1, 2), (9, 9)).transform((9, 9)) == ((1, 2), (8, 7))); // Cropped start
    /// assert!(((9, 9), (9, 9)).transform((9, 9)) == ((9, 9), (0, 0))); // Empty
    /// ```
    {T: AsCoord} (T, Coord) =>
        (size) { from_segments((
            (self.0.x(), self.1.0).transform(size.0),
            (self.0.y(), self.1.1).transform(size.1),
        )) }

    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(((1, 2), |size: Coord| size.sub(10)).transform((4, 4)) == ((1, 2), (0, 0))); // Cropped
    /// assert!(((1, 2), |size: Coord| size.add(10)).transform((4, 4)) == ((1, 2), (3, 2))); // Cropped start
    /// assert!(((9, 9), |size: Coord| size.add(10)).transform((4, 4)) == ((4, 4), (0, 0))); // Empty
    /// ```
    {T: AsCoord, F: FnOnce(Coord) -> Coord} (T, F) =>
        (size) (self.0, self.1(size)).transform(size)

    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(((1, 2), (2, |size| size / 4)).transform((8, 4)) == ((1, 2), (2, 1))); // Cropped
    /// assert!(((1, 2), (9, |size| size * 4)).transform((8, 4)) == ((1, 2), (7, 2))); // Cropped Start
    /// assert!(((9, 9), (9, |size| size * 4)).transform((8, 4)) == ((8, 4), (0, 0))); // Empty
    /// ```
    {T: AsCoord, F: FnOnce(u16) -> u16} (T, (u16, F)) =>
        (size) (self.0, (self.1.0, self.1.1(size.1))).transform(size)

    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(((2, 1), (|size| size / 4, 2)).transform((4, 8)) == ((2, 1), (1, 2))); // Cropped
    /// assert!(((2, 1), (|size| size * 4, 9)).transform((4, 8)) == ((2, 1), (2, 7))); // Cropped Start
    /// assert!(((9, 9), (|size| size * 4, 9)).transform((4, 8)) == ((4, 8), (0, 0))); // Empty
    /// ```
    {T: AsCoord, F: FnOnce(u16) -> u16} (T, (F, u16)) =>
        (size) (self.0, (self.1.0(size.0), self.1.1)).transform(size)

    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(((1, 2), (|size| size / 2, |size| size / 3)).transform((8, 6)) == ((1, 2), (4, 2))); // Cropped
    /// assert!(((1, 2), (|size| size * 2, |size| size * 3)).transform((8, 6)) == ((1, 2), (7, 4))); // Cropped start
    /// assert!(((9, 9), (|size| size * 2, |size| size * 3)).transform((8, 6)) == ((8, 6), (0, 0))); // Empty
    /// ```
    {T: AsCoord, FX: FnOnce(u16) -> u16, FY: FnOnce(u16) -> u16} (T, (FX, FY)) =>
        (size) (self.0, (self.1.0(size.0), self.1.1(size.1))).transform(size)
]);

// ======= //
// Helpers //
// ======= //

/// Clamps `size` so that `point + size <= max_size`,
/// or `0` if `point >= max_size`.
#[inline(always)]
fn clamp_into(point: u16, size: u16, max_size: u16) -> u16 {
    if point + size <= max_size {
        size
    } else {
        max_size.saturating_sub(point)
    }
}

/// Returns a `((x, y), (width, height))` [`Rect`](crate::Rect) from a
/// `((x, width), (y, height))` segment tuple.
#[inline(always)]
fn from_segments(rect: Rect) -> Rect {
    ((rect.x(), rect.width()), (rect.y(), rect.height()))
}

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
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

    #[test]
    fn from_segments() {
        assert_eq!(super::from_segments(((1, 2), (3, 4))), ((1, 3), (2, 4)));
    }
}
