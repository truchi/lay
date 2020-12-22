use crate::*;
use std::ops::{Range, RangeBounds};

/// Grid trait.
pub trait IGrid<'a> {
    /// The type of the elements.
    type Cell: 'a;

    /// The type of the row iterator.
    type Row: Iterator<Item = &'a Self::Cell>;

    /// The type of the column iterator.
    type Col: Iterator<Item = &'a Self::Cell>;

    /// Returns the [`Size`](crate::Size) of this `grid`.
    fn size(&self) -> Size<usize>;

    /// Returns the element at `point` without bounds checking.
    ///
    /// Callers **MUST** ensure:
    /// - `point < size`
    unsafe fn cell_unchecked(&self, point: Point<usize>) -> &Self::Cell;

    /// Returns an iterator over elements at `row` in `range` without bounds
    /// checking.
    ///
    /// Callers **MUST** ensure:
    /// - `row < height`
    /// - `start <= end`
    /// - `end <= width`
    unsafe fn row_unchecked(&self, row: usize, range: Range<usize>) -> Self::Row;

    /// Returns an iterator over elements at `col` in `range` without bounds
    /// checking.
    ///
    /// Callers **MUST** ensure:
    /// - `col < width`
    /// - `start <= end`
    /// - `end <= height`
    unsafe fn col_unchecked(&self, col: usize, range: Range<usize>) -> Self::Col;

    /// Returns the element at `point` without bounds checking,
    /// or `None` if `point >= size`.
    fn cell(&self, point: Point<usize>) -> Option<&Self::Cell> {
        if point < self.size().as_point() {
            // SAFETY:
            // point < size
            Some(unsafe { self.cell_unchecked(point) })
        } else {
            None
        }
    }

    /// Returns an iterator over elements at `row` in `range`,
    /// or `None` if `row >= height`.
    fn row(&'a self, row: usize, range: impl RangeBounds<usize>) -> Option<Self::Row> {
        let (width, height) = self.size().into();
        let range = Row::validate(self, row, range)?;

        // SAFETY:
        // Row::validate guaranties that:
        debug_assert!(row < height);
        debug_assert!(range.start <= range.end);
        debug_assert!(range.end <= width);
        Some(unsafe { self.row_unchecked(row, range) })
    }

    /// Returns an iterator over elements at `col` in `range`,
    /// or `None` if `col >= width`.
    fn col(&self, col: usize, range: impl RangeBounds<usize>) -> Option<Self::Col> {
        let (width, height) = self.size().into();

        if col < width {
            let range = nice_range(height, range);
            // SAFETY:
            // col < width
            // nice_range guaranties that:
            debug_assert!(col < width);
            debug_assert!(range.start <= range.end);
            debug_assert!(range.end <= height);
            Some(unsafe { self.col_unchecked(col, range) })
        } else {
            None
        }
    }
}

macro_rules! row_col {
    ($width:ident $height:ident $(
        $(#[$meta:meta])* $Type:ident $at:ident
        $axis:ident $axis_len:ident
        $other_axis:ident $other_axis_len:ident
    )*) => { $(
        $(#[$meta])*
        pub struct $Type<'a, T: ?Sized> {
            grid:  &'a T,
            $at:   usize,
            range: Range<usize>,
        }

        impl<'a, T: IGrid<'a> + ?Sized> $Type<'a, T> {
            doc!("Creates a new [`" s!($Type) "`](crate::" s!($Type) ") without bounds checking.\n\n"
                "See [`IGrid::" s!($at) "_unchecked`](crate::IGrid::" s!($at) "_unchecked) for safety.",
            pub unsafe fn new_unchecked(grid: &'a T, $at: usize, range: Range<usize>) -> Self {
                Self { grid, $at, range }
            });

            doc!("Creates a new [`" s!($Type) "`](crate::" s!($Type) ").",
            pub fn new(grid: &'a T, $at: usize, range: impl RangeBounds<usize>) -> Option<Self> {
                let range = Self::validate(grid, $at, range)?;

                Some(Self { grid, $at, range })
            });

            doc!("Validates `" s!($at) "` and `range`.\n\n"
                "Nicifies `range` (see [`nice_range`](crate::nice_range)) or returns `None` if `" s!($at) " >= " s!($axis_len) "`.",
            pub fn validate(grid: &T, $at: usize, range: impl RangeBounds<usize>) -> Option<Range<usize>> {
                let ($width, $height) = grid.size().into();

                if $at < $axis_len {
                    Some(nice_range($other_axis_len, range))
                } else {
                    None
                }
            });

            /// Returns the [`Point`](crate::Point) at the current iteration step.
            fn point(&self) -> Point<usize> {
                Point {
                    $axis: self.$at,
                    $other_axis: self.range.start,
                }
            }
        }

        impl<'a, T: IGrid<'a>> Iterator for $Type<'a, T> {
            type Item = &'a T::Cell;

            fn next(&mut self) -> Option<Self::Item> {
                let Range { start, end } = self.range;

                if start < end {
                    let point = self.point();
                    self.range.start += 1;

                    // SAFETY:
                    // constructors guaranty that:
                    debug_assert!(point < self.grid.size().as_point());
                    Some(unsafe { self.grid.cell_unchecked(point) })
                } else {
                    None
                }
            }
        }
    )* };
}

row_col!(width height
    /// Row iterator helper.
    Row row y height x width
    /// Column iterator helper.
    Col col x width y height
);
