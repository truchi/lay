use crate::*;

/// [`Transform`](crate::Transform)ations (translate + resize).
///
/// `Transform<u16>`:
/// - `(origin, size)` segment -> `(new_point, new_size)` segment,
/// - Used to crop a row of a [`Layer`](crate::Layer).
///
/// `Transform<Coord>`:
/// - `(origin, size)` [`Rect`](crate::Rect) -> `(new_point, new_size)`
///   [`Rect`](crate::Rect),
/// - Used to crop a whole [`Layer`](crate::Layer).
///
/// In the form
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
    /// Resizes only.
    ///
    /// ```
    /// # use lay::*;
    /// assert!(2.transform(1) == (0, 2));
    /// ```
    u16 =>
        (_) (0, self)
    /// Resizes only.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((|size| size / 2).transform(4) == (0, 2));
    /// ```
    {F: FnOnce(u16) -> u16} F =>
        (size) (0, self(size))
    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1, 2).transform(4) == (1, 2));
    /// ```
    (u16, u16) =>
        (_) self
    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1, |size| size / 2).transform(4) == (1, 2));
    /// ```
    {F: FnOnce(u16) -> u16} (u16, F) =>
        (size) (self.0, self.1(size))
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
    /// assert!(2.transform((1, 1)) == ((0, 0), (2, 2)));
    /// ```
    u16 =>
        (_) (COORD_ZERO, self.as_coord())
    /// Resizes only.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((|size: Coord| size.sub(1)).transform((4, 6)) == ((0, 0), (3, 5)));
    /// ```
    {F: FnOnce(Coord) -> Coord} F =>
        (size) (COORD_ZERO, self(size))
    // =============
    // =============
    // =============
    // =============
    // =============
    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1,      3).transform((4, 4)) == ((1, 1), (3, 3)));
    /// assert!(((1, 2), 3).transform((4, 4)) == ((1, 2), (3, 3)));
    /// ```
    {T: AsCoord} (T, u16) =>
        (_) self.as_rect()
    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1,      (3, 4)).transform((5, 5)) == ((1, 1), (3, 4)));
    /// assert!(((1, 2), (3, 4)).transform((5, 5)) == ((1, 2), (3, 4)));
    /// ```
    {T: AsCoord} (T, Coord) =>
        (_) self.as_rect()
    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1,      |size: Coord| size.sub(2)).transform((4, 4)) == ((1, 1), (2, 2)));
    /// assert!(((1, 2), |size: Coord| size.sub(2)).transform((4, 4)) == ((1, 2), (2, 2)));
    /// ```
    {T: AsCoord, F: FnOnce(Coord) -> Coord} (T, F) =>
        (size) (self.0.as_coord(), self.1(size))
    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1,      (8, |size| size * 4)).transform((2, 4)) == ((1, 1), (8, 16)));
    /// assert!(((1, 2), (8, |size| size * 4)).transform((2, 4)) == ((1, 2), (8, 16)));
    /// ```
    {T: AsCoord, F: FnOnce(u16) -> u16} (T, (u16, F)) =>
        (size) (self.0.as_coord(), (self.1.0, self.1.1(size.1)))
    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1,      (|size| size / 2, 8)).transform((2, 4)) == ((1, 1), (1, 8)));
    /// assert!(((1, 2), (|size| size / 2, 8)).transform((2, 4)) == ((1, 2), (1, 8)));
    /// ```
    {T: AsCoord, F: FnOnce(u16) -> u16} (T, (F, u16)) =>
        (size) (self.0.as_coord(), (self.1.0(size.0), self.1.1))
    /// Translates and resizes.
    ///
    /// ```
    /// # use lay::*;
    /// assert!((1,      (|size| size / 2, |size| size * 3)).transform((2, 4)) == ((1, 1), (1, 12)));
    /// assert!(((1, 2), (|size| size / 2, |size| size * 3)).transform((2, 4)) == ((1, 2), (1, 12)));
    /// ```
    {T: AsCoord, FX: FnOnce(u16) -> u16, FY: FnOnce(u16) -> u16} (T, (FX, FY)) =>
        (size) (self.0.as_coord(), (self.1.0(size.0), self.1.1(size.1)))
]);
